use std::f32::consts::PI;
use crate::graphics_2d::Graphics2D;
use crate::object_2d::Object2D;
use crate::shape_2d::Shape2D;

#[derive(Default)]
pub(crate) struct Circle {
    pub(crate) base: Object2D,
    pub(crate) radius: usize,
    pub(crate) fill_color: u32,
    pub(crate) outline_color: u32,
    //TODO: pub(crate) outline_thickness: usize, (Maybe a better name than thickness)
}

impl Shape2D for Circle {
    fn draw(&self, graphics_2d: &mut Graphics2D) {
        // No zero-radius circles
        if self.radius == 0 { return; }

        let outline_alpha = self.outline_color >> 24;

        // Check whether color is fully transparent, if so, don't draw fill
        if self.fill_color >> 24 != 0xFF {  // 0xFF000000 -> 0x000000FF (6 hex sh = 24 bin sh)
            let (start_offset, end_offset) = if outline_alpha == 0xFF {
                (-(self.radius as isize) + 1, (self.radius as isize) - 1)
            } else {
                (-(self.radius as isize), self.radius as isize)
            };

            for x_px in start_offset..end_offset {
                for y_px in start_offset..end_offset {
                    if x_px * x_px + y_px * y_px <= (self.radius * self.radius) as isize {
                        graphics_2d._set_pixel(
                            (x_px + self.radius as isize) as usize,
                            (y_px + self.radius as isize) as usize,
                            self.fill_color,
                            &self.base
                        );
                    }
                }
            }
        }

        // Draw outline (if present)
        if outline_alpha != 0xFF {
            let r_f32 = self.radius as f32;

            let sample_points = 4f32 / 2f32.sqrt() * PI * r_f32; // Maybe try not to use sqrt, but for now it's to make scaling consistent, also it can grow pretty quickly
            let mut i = 0f32;

            loop {
                let px_x = (i.cos() + 1f32) * r_f32;
                let px_y = (i.sin() + 1f32) * r_f32;

                if px_x >= 0f32 && px_y >= 0f32 {
                    graphics_2d._set_pixel(px_x as usize, px_y as usize, self.outline_color, &self.base);
                }


                if i >= sample_points {
                    break;
                }
                i += 1f32;
            }
        }
    }
}
