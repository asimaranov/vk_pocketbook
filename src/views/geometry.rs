use inkview_sys::Color;
use crate::ui_engine::app_context::AppContext;
use crate::ui_engine::color::UIColor;
use crate::ui_engine::geometry::UIRect;
use crate::ui_engine::ui_view::UIView;

pub struct FilledRectView {
    pub(crate) color: UIColor,
    rect: UIRect,
}

impl UIView for FilledRectView {


    fn get_rect(&self) -> UIRect {
        return self.rect;
    }

    fn render(&self, ctx: &mut AppContext) {
        inkview_sys::fill_area(
            self.rect.origin.x as i32,
            self.rect.origin.y as i32,
            self.rect.size.width as i32,
            self.rect.size.height as i32,
            Color(self.color.0)
        )
    }
}

impl FilledRectView {
    pub(crate) fn new(rect: UIRect, color: UIColor) -> Self {
        return Self{
            color,
            rect
        }

    }
}