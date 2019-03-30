use crate::graphics_2d::*;

mod pure2d;

pub(crate) fn new() -> Box<dyn ContextualGraphics> {
    pure2d::new()
}