use crate::positioning::{Pos, Rect};


pub trait DrawCaptureRegion {
    fn draw_capture_region(&self, image: &mut image::RgbImage);
}

// todo other types
impl DrawCaptureRegion for Pos<f64> {
    fn draw_capture_region(&self, image: &mut image::RgbImage) {
        let blue = image::Rgb([0, 0, 255]);

        let x = self.x as u32;
        let y = self.y as u32;

        for i in x - 1..=x + 1 {
            for j in y - 1..=y + 1 {
                image.put_pixel(i, j, blue);
            }
        }

        for i in x - 5..=x + 5 {
            image.put_pixel(i, y + 5, blue);
            image.put_pixel(i, y - 5, blue);
        }

        for j in y - 5..=y + 5 {
            image.put_pixel(x + 5, j, blue);
            image.put_pixel(x - 5, j, blue);
        }
    }
}

impl DrawCaptureRegion for Rect<f64> {
    fn draw_capture_region(&self, image: &mut image::RgbImage) {
        let red = image::Rgb([255, 0, 0]);

        let left = self.left as u32;
        let top = self.top as u32;
        let width = self.width as u32;
        let height = self.height as u32;
        let bottom = top + height;
        let right = left + width;

        for x in left..right {
            image.put_pixel(x, top, red);
            image.put_pixel(x, bottom, red);
        }

        for y in top..bottom {
            image.put_pixel(left, y, red);
            image.put_pixel(right, y, red);
        }
    }
}