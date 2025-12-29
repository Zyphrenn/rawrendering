pub(crate) mod cubic_bezier_curve;
pub(crate) mod line;

use crate::graphics_2d::Graphics2D;
use crate::object_2d::Object2D;

pub(crate) trait PathTrace {
    fn draw(&self, base: &Object2D, graphics_2d: &mut Graphics2D);
}
