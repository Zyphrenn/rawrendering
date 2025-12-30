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
            let alpha = (color >> 24 & 0xFF) as f32 / 255f32; // 0xFF00000000 -> 0x000000FF (6 hex sh = 24 bin sh), mask isn't strictly necessary for u32

            if alpha == 1f32 { return; } // Transparent, don't draw
            else if alpha == 0f32 { 
                // Opaque, don't calculate color mixing since it's just the new color anyway
                self.pixels[y * self.width + x] = color;
            } else {
                // Color is semi-transparent, color mixing needs to be calculated
                // Old "background" color
                let old_color = self.get_pixel(x, y);

                let bg_r = old_color >> 16 & 0xFF;  // 0xFF0000 -> 0x0000FF (4 hex sh = 16 bin sh)
                let bg_g = old_color >> 8 & 0xFF;   // 0x00FF00 -> 0x0000FF (2 hex sh = 8 bin sh)
                let bg_b = old_color & 0xFF;        // 0x0000FF -> 0x0000FF (0 hex sh = 0 bin sh)

                // New "foreground" color
                let fg_r = color >> 16 & 0xFF;
                let fg_g = color >> 8 & 0xFF;
                let fg_b = color & 0xFF;

                // Mix bg and fg colors
                let fin_r = ((bg_r as f32 * alpha) + (fg_r as f32 * (1f32 - alpha))) as u32 & 0xFF;
                let fin_g = ((bg_g as f32 * alpha) + (fg_g as f32 * (1f32 - alpha))) as u32 & 0xFF;
                let fin_b = ((bg_b as f32 * alpha) + (fg_b as f32 * (1f32 - alpha))) as u32 & 0xFF;

                let fin_color = fin_r << 16 | fin_g << 8 | fin_b;
                self.pixels[y * self.width + x] = fin_color;
            }
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

}
