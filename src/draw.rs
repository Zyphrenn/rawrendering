pub struct Canvas<'a> {
    pub pixels: &'a mut [u32],
    pub width: usize,
    pub height: usize,
}

impl<'a> Canvas<'a> {
    pub fn put_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color;
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.pixels.fill(color);
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        for x_px in x..x + w {
            for y_px in y..y+ h {
                self.put_pixel(x_px, y_px, color);
            }
        }
    }

    pub fn draw_rect_outline(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        if w > 0 {
            for x_px in x..x +w {
                //NOTE: Puts pixels even if h == 0, this results in h = 1 and h = 0 looking the same
                self.put_pixel(x_px, y, color);
                if h > 0 {
                    self.put_pixel(x_px, y + h - 1, color);
                }
            }
        }

        if h > 0 {
            for y_px in y..y +h {
                //NOTE: Puts pixels even if w == 0, this results in w = 1 and w = 0 looking the same
                self.put_pixel(x, y_px, color);
                if w > 0 {
                    self.put_pixel(x + w - 1, y_px, color);
                }
            }
        }
    }
}