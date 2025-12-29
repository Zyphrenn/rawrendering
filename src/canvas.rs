use crate::canvas::polygon::Polygon;
use std::f32::consts::PI;

// TODO: Fix mess with using isize/usize/f32 as params
// Maybe the functions can be sorted into two categories:
// Low-level, using only usize for efficiency
// High-level, using isize and f32 to make things more comfortable to use

pub mod polygon;

pub struct Canvas<'a> {
    pub pixels: &'a mut [u32],
    pub width: usize,
    pub height: usize,
}

impl<'a> Canvas<'a> {
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u32 {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x]
        } else {
            0x00000000
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.pixels.fill(color);
    }

    // TODO: Implement color mixing in set_pixel
    pub fn draw_rect_with_transparency(
        &mut self,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        color: u32,
    ) {
        let from_x = x1.min(x2);
        let from_y = y1.min(y2);

        let to_x = x1.max(x2);
        let to_y = y1.max(y2);

        for x_px in from_x..to_x {
            for y_px in from_y..to_y {
                let old_color = self.get_pixel(x_px, y_px);
                let old_r = (old_color >> 16) & 0xFF; // 0xFF0000 -> 0x0000FF (4 hex sh = 16 bin sh), requires mask
                let old_g = (old_color >> 8) & 0xFF; // 0x00FF00 -> 0x0000FF (2 hex sh = 8 bin sh), requires mask
                let old_b = old_color & 0xFF; // 0x0000FF -> 0x0000FF (0 hex sh = 0 bin sh), requires mask

                let new_r = color >> 16 & 0xFF;
                let new_g = (color >> 8) & 0xFF;
                let new_b = color & 0xFF;

                let transparency = (color >> 24) as f32 / 255f32; // 0xFF00000000 -> 0x000000FF (6 hex sh = 24 bin sh), no mask required, u32 ends here

                // Additive color mixing (not desirable, but good PoC for now)
                let fin_r = (old_r + (new_r as f32 * transparency) as u32) & 0xFF;
                let fin_g = (old_g + (new_g as f32 * transparency) as u32) & 0xFF;
                let fin_b = (old_b + (new_b as f32 * transparency) as u32) & 0xFF;

                let fin_color = fin_r << 16 | fin_g << 8 | fin_b;

                self.set_pixel(x_px, y_px, fin_color);
            }
        }
    }
}
