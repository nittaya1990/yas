use std::cell::RefCell;
use std::ops::Coroutine;
use std::rc::Rc;
use image::{Rgb, RgbImage};
use yas::game_info::GameInfo;
use crate::scanner_controller::repository_layout::config::StarRailRepositoryScannerLogicConfig;
use yas::utils;
use log::{info, error};
use std::time::SystemTime;
use yas::capture::{Capturer, GenericCapturer};
use yas::system_control::SystemControl;
use crate::scanner_controller::repository_layout::window_info::StarRailRepositoryScanControllerWindowInfo;
use anyhow::{anyhow, Result};
use clap::{ArgMatches, FromArgMatches};
use yas::utils::color_distance;
use yas::window_info::{FromWindowInfoRepository, WindowInfoRepository};
use crate::scanner_controller::repository_layout::scroll_result::ScrollResult;

pub struct StarRailRepositoryScanController {
    // to detect whether an item changes
    pool: f64,

    // Stores initial gap colors for line gap detection
    initial_flag: [Rgb<u8>; 50],

    // for scrolls
    scrolled_rows: u32,
    avg_scroll_one_row: f64,

    avg_switch_time: f64,
    scanned_count: usize,

    game_info: GameInfo,

    row: usize,
    col: usize,
    // item_count: usize,

    config: StarRailRepositoryScannerLogicConfig,
    window_info: StarRailRepositoryScanControllerWindowInfo,
    system_control: SystemControl,
    capturer: Rc<dyn Capturer<RgbImage>>,
}

fn calc_pool(row: &Vec<u8>) -> f32 {
    let len = row.len() / 3;
    let mut pool: f32 = 0.0;

    for i in 0..len {
        pool += row[i * 3] as f32;
    }
    pool
}

fn get_capturer() -> Result<Rc<dyn Capturer<RgbImage>>> {
    Ok(Rc::new(GenericCapturer::new()?))
}

// constructor
impl StarRailRepositoryScanController {
    pub fn new(
        window_info_repo: &WindowInfoRepository,
        config: StarRailRepositoryScannerLogicConfig,
        game_info: GameInfo
    ) -> Result<Self> {
        let window_info = StarRailRepositoryScanControllerWindowInfo::from_window_info_repository(
            game_info.window.to_rect_usize().size(),
            game_info.ui,
            game_info.platform,
            window_info_repo
        )?;

        let row_count = window_info.starrail_repository_item_row;
        let col_count = window_info.starrail_repository_item_col;

        Ok(StarRailRepositoryScanController {
            system_control: SystemControl::new(),

            row: row_count as usize,
            col: col_count as usize,

            window_info,
            config,

            pool: 0.0,

            initial_flag: [Rgb([0, 0, 0]); 50],

            scrolled_rows: 0,
            avg_scroll_one_row: 0.0,

            avg_switch_time: 0.0,

            game_info,
            scanned_count: 0,

            capturer: get_capturer()?,
        })
    }

    pub fn from_arg_matches(
        window_info_repo: &WindowInfoRepository,
        arg_matches: &ArgMatches,
        game_info: GameInfo
    ) -> Result<Self> {
        Self::new(
            window_info_repo,
            StarRailRepositoryScannerLogicConfig::from_arg_matches(arg_matches)?,
            game_info
        )
    }
}

pub enum ReturnResult {
    Interrupted,
    Finished,
}

impl StarRailRepositoryScanController {
    pub fn get_generator(
        object: Rc<RefCell<StarRailRepositoryScanController>>,
        item_count: usize,
    ) -> impl Coroutine<Yield = (), Return = Result<ReturnResult>> {
        let generator = #[coroutine] move || {
            let mut scanned_row = 0;
            let mut scanned_count = 0;
            let mut start_row = 0;

            let total_row = (item_count + object.borrow().col - 1) / object.borrow().col;
            let last_row_col = if item_count % object.borrow().col == 0 {
                object.borrow().col
            } else {
                item_count % object.borrow().col
            };

            info!(
                "扫描任务共 {} 个物品，共计 {} 行，尾行 {} 个",
                item_count, total_row, last_row_col
            );

            object.borrow_mut().move_to(0, 0);

            #[cfg(target_os = "macos")]
            utils::sleep(20);

            // todo remove unwrap
            object.borrow_mut().system_control.mouse_click().unwrap();
            utils::sleep(1000);

            object.borrow_mut().sample_initial_color().unwrap();

            let row = object.borrow().row.min(total_row);

            'outer: while scanned_count < item_count {
                '_row: for row in start_row..row {
                    let row_item_count = if scanned_row == total_row - 1 {
                        last_row_col
                    } else {
                        object.borrow().col
                    };

                    '_col: for col in 0..row_item_count {
                        // Exit if right mouse button is down, or if we've scanned more than the maximum count
                        if utils::is_rmb_down() {
                            return Ok(ReturnResult::Interrupted);
                        }
                        if scanned_count > item_count {
                            return Ok(ReturnResult::Finished);
                        }

                        object.borrow_mut().move_to(row, col);
                        object.borrow_mut().system_control.mouse_click().unwrap();

                        #[cfg(target_os = "macos")]
                        utils::sleep(20);

                        let _ = object.borrow_mut().wait_until_switched();

                        // have to make sure at this point no mut ref exists
                        yield;

                        scanned_count += 1;
                        object.borrow_mut().scanned_count = scanned_count;
                    } // end '_col

                    scanned_row += 1;

                    // todo this is dangerous, use uniform integer type instead
                    if scanned_row >= object.borrow().config.max_row as usize {
                        info!("到达最大行数，准备退出……");
                        break 'outer;
                    }
                } // end '_row

                let remain = item_count - scanned_count;
                let remain_row = (remain + object.borrow().col - 1) / object.borrow().col;
                let scroll_row = remain_row.min(object.borrow().row);
                start_row = object.borrow().row - scroll_row;

                match object.borrow_mut().scroll_rows(scroll_row as i32) {
                    ScrollResult::TimeLimitExceeded => {
                        return Err(anyhow!("翻页超时，扫描终止……"));
                    },
                    ScrollResult::Interrupt => {
                        return Ok(ReturnResult::Interrupted);
                    },
                    _ => (),
                }

                utils::sleep(100);
            }

            Ok(ReturnResult::Finished)
        };

        generator
    }

    #[inline(always)]
    pub fn sample_initial_color(&mut self) -> Result<()> {
        self.initial_flag = self.capture_flag()?;
        Ok(())
    }

    #[inline(always)]
    pub fn capture_flag(&self) -> Result<[Rgb<u8>; 50]> {
        let mut flag = [Rgb([0, 0, 0]); 50];
        let window_origin = self.game_info.window.to_rect_f64().origin();
        let rect = self.window_info.flag_rect.translate(window_origin);
        let im = self.capturer.capture_rect(rect.to_rect_i32())?;

        // Gap size between repository top and first item row varies with resolution.
        // At 1920x1080, it's 20 pixels.
        for y in 0..self.window_info.flag_rect.height as usize {
            let color = im.get_pixel(0, y as u32);
            flag[y] = color.clone();
        }

        Ok(flag)
    }

    #[inline(always)]
    pub fn check_flag(&self) -> Result<()> {
        let flag = self.capture_flag()?;
        // println!("{:?}", &flag[..20]);
        // let mut same_count = 0;
        for y in 0..self.window_info.flag_rect.height as usize {
            if color_distance(&self.initial_flag[y], &flag[y]) < 10 {
                // same_count += 1;
                return Ok(())
            }
        }
        // let ratio = same_count as f64 / self.window_info.flag_rect.height;
        // println!("{:?}", ratio);
        // if ratio > 0.5 {
        //     Ok(())
        // } else {
            Err(anyhow!("Flag changed"))
        // }
    }

    pub fn align_row(&mut self) {
        for _ in 0..10 {
            if self.check_flag().is_err() {
                self.mouse_scroll(1, false);
                utils::sleep(self.config.scroll_delay.try_into().unwrap());
            } else {
                break;
            }
        }
    }

    pub fn move_to(&mut self, row: usize, col: usize) {
        let (row, col) = (row as u32, col as u32);
        let origin = self.game_info.window.to_rect_f64().origin();

        let gap = self.window_info.item_gap_size;
        let margin = self.window_info.scan_margin_pos;
        let size = self.window_info.item_size;

        let left = origin.x + margin.x + (gap.width + size.width) * (col as f64) + size.width / 2.0;
        let top = origin.y + margin.y + (gap.height + size.height) * (row as f64) + size.height / 2.0;

        self.system_control.mouse_move_to(left as i32, top as i32).unwrap();

        #[cfg(target_os = "macos")]
        utils::sleep(20);
    }

    pub fn scroll_one_row(&mut self) -> ScrollResult {
        let mut state = 0;
        let mut count = 0;
        let max_scroll = 25;

        while count < max_scroll {
            if utils::is_rmb_down() {
                return ScrollResult::Interrupt;
            }

            #[cfg(windows)]
            let _ = self.system_control.mouse_scroll(1, false);

            utils::sleep(self.config.scroll_delay.try_into().unwrap());
            count += 1;

            match (state, self.check_flag()) {
                (0, Err(_)) => state = 1,
                (1, Ok(_)) => {
                    self.update_avg_row(count);
                    return ScrollResult::Success;
                }
                _ => {}
            }
        }

        ScrollResult::TimeLimitExceeded
    }

    pub fn scroll_rows(&mut self, count: i32) -> ScrollResult {
        if cfg!(not(target_os = "macos")) && self.scrolled_rows >= 5 {
            let length = self.estimate_scroll_length(count);

            for _ in 0..length {
                if let Err(e) = self.system_control.mouse_scroll(1, false) {
                    error!("Scrolling failed: {:?}", e);
                    return ScrollResult::Interrupt;
                }
            }

            utils::sleep(self.config.scroll_delay.try_into().unwrap());

            self.align_row();
            return ScrollResult::Skip;
        }

        for _ in 0..count {
            match self.scroll_one_row() {
                ScrollResult::Success | ScrollResult::Skip => continue,
                ScrollResult::Interrupt => return ScrollResult::Interrupt,
                v => {
                    error!("Scrolling failed: {:?}", v);
                    return v;
                },
            }
        }

        ScrollResult::Success
    }

    pub fn wait_until_switched(&mut self) -> Result<()> {
        if self.game_info.is_cloud {
            utils::sleep(self.config.cloud_wait_switch_item.try_into()?);
            return Ok(());
        }

        let now = SystemTime::now();

        let mut consecutive_time = 0;
        let mut diff_flag = false;
        while now.elapsed()?.as_millis() < self.config.max_wait_switch_item as u128 {
            let im = self.capturer.capture_relative_to(
                self.window_info.pool_rect.to_rect_i32(),
                self.game_info.window.origin()
            )?;

            let pool = calc_pool(im.as_raw()) as f64;

            if (pool - self.pool).abs() > 0.000001 {
                self.pool = pool;
                diff_flag = true;
                consecutive_time = 0;
            } else if diff_flag {
                consecutive_time += 1;
                if consecutive_time == 1 {
                    self.avg_switch_time = (self.avg_switch_time * self.scanned_count as f64
                        + now.elapsed().unwrap().as_millis() as f64)
                        / (self.scanned_count as f64 + 1.0);
                    self.scanned_count += 1;
                    return anyhow::Ok(());
                }
            }
        }

        Err(anyhow!("Wait until switched failed"))
    }

    #[inline(always)]
    pub fn mouse_scroll(&mut self, length: i32, try_find: bool) {
        #[cfg(windows)]
        self.system_control.mouse_scroll(length, try_find).unwrap();

        #[cfg(target_os = "linux")]
        self.system_control.mouse_scroll(length, try_find);

        #[cfg(target_os = "macos")]
        {
            match self.game_info.ui {
                crate::common::UI::Desktop => {
                    self.system_control.mouse_scroll(length);
                    utils::sleep(20);
                },
                crate::common::UI::Mobile => {
                    if try_find {
                        self.system_control.mac_scroll_fast(length);
                    } else {
                        self.system_control.mac_scroll_slow(length);
                    }
                },
            }
        }
    }

    #[inline(always)]
    fn update_avg_row(&mut self, count: i32) {
        let current = self.avg_scroll_one_row * self.scrolled_rows as f64 + count as f64;
        self.scrolled_rows += 1;
        self.avg_scroll_one_row = current / self.scrolled_rows as f64;

        info!(
            "avg scroll one row: {} ({})",
            self.avg_scroll_one_row, self.scrolled_rows
        );
    }

    #[inline(always)]
    fn estimate_scroll_length(&self, count: i32) -> i32 {
        ((self.avg_scroll_one_row * count as f64 - 3.0).round() as i32).max(0)
    }
}