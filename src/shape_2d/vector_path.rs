use crate::graphics_2d::Graphics2D;
use crate::object_2d::Object2D;
use crate::shape_2d::path_trace::PathTrace;
use crate::shape_2d::Shape2D;

pub(crate) struct VectorPath2D<'a> {
    pub(crate) base: Object2D,
    pub(crate) path: &'a [Box<dyn PathTrace>],
}

impl Shape2D for VectorPath2D<'_> {
    fn draw(&self, graphics_2d: &mut Graphics2D) {
        for path_trace in self.path.iter() {
            path_trace.draw(&self.base, graphics_2d);
        }
    }
}