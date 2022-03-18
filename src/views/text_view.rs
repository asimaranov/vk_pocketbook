use crate::ui_engine::geometry::UIRect;
use inkview_sys::PbBmp;
use crate::ui_engine::app_context::AppContext;
use crate::ui_engine::ui_view::UIView;

pub struct TextView{
    rect: UIRect,
    text: String,
    text_size: u32
}

impl UIView for TextView{
    fn get_rect(&self) -> UIRect {
        self.rect
    }

    fn render(&self, ctx: &mut AppContext) {
        let font = inkview_sys::open_font("Roboto", self.text_size as i32, 1);
        inkview_sys::set_font(font, inkview_sys::Color::BLACK);

        let result = inkview_sys::draw_text_rect(self.rect.origin.x as i32, self.rect.origin.y as i32,
                                                 self.rect.size.width as i32, self.rect.size.height as i32,
                                                 &*self.text,
                                                 inkview_sys::c_api::ALIGN_CENTER | inkview_sys::c_api::VALIGN_MIDDLE);

    }
}

impl TextView {
    pub(crate) fn new(rect: UIRect, text: String, text_size: u32) -> Self{
        return Self{
            rect, text, text_size
        }
    }
}