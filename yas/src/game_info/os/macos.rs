use crate::{common::utils::*, core::ui::Resolution};
use crate::game_info::{GameInfo, Platform};

pub fn get_game_info() -> GameInfo {
    let (pid, ui) = get_pid_and_ui();

    let (rect, window_title) = unsafe { find_window_by_pid(pid).unwrap() };

    info!("找到游戏窗口：{} (PID: {})", window_title, pid);

    GameInfo {
        window: rect,
        resolution_family: Resolution::new(rect.size),
        is_cloud: false,
        ui,
        platform: Platform::MacOS
    }
}
