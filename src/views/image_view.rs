use crate::ui_engine::geometry::UIRect;
use inkview_sys::PbBmp;
use crate::ui_engine::app_context::AppContext;
use crate::ui_engine::ui_view::UIView;

pub struct ImageView{
    rect: UIRect,
    image: PbBmp,
}

impl UIView for ImageView{
    fn get_rect(&self) -> UIRect {
        self.rect
    }

    fn render(&self, ctx: &mut AppContext) {
        inkview_sys::draw_bitmap(self.rect.origin.x as i32, self.rect.origin.y as i32, self.image.0)
    }
}

impl ImageView {
    pub(crate) fn new(rect: UIRect, image: PbBmp) -> Self{
        let ico = inkview_sys::scale_bitmap_to(image.get_pointer(), rect.size.width as i32 , rect.size.height as i32);
        inkview_sys::mirror_bitmap(ico, inkview_sys::MirrorFlag::Y_MIRROR as i32);
        return Self{
            rect, image: PbBmp(ico)
        }
    }
}