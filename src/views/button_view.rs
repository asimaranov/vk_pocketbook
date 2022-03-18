use std::cell::Cell;
use std::ffi::{CStr, CString};
use std::sync::Mutex;
use inkview_sys::{Color, set_font};

use crate::ui_engine::app_context::AppContext;
use crate::ui_engine::color::UIColor;
use crate::ui_engine::events::UIEvent;
use crate::ui_engine::geometry::UIRect;
use crate::ui_engine::ui_view::UIView;
extern crate chrono;

use chrono::Local;

pub struct ButtonView {
    rect: UIRect,
    pub(crate) text: String,
    pub(crate) color: UIColor,
    pub is_clicked: bool

}

impl ButtonView {
    pub fn new(rect: UIRect, text: String, color: UIColor) -> Self {
        return Self{
            rect,
            text,
            color,
            is_clicked: false
        }
    }
    pub fn is_clicked(&self) -> bool {
        self.is_clicked
    }
}

impl UIView for ButtonView {
    fn get_rect(&self) -> UIRect {
        self.rect
    }

    fn render(&self, ctx: &mut AppContext) {
        inkview_sys::draw_rect(self.rect.origin.x as i32, self.rect.origin.y as i32,
                               self.rect.size.width as i32, self.rect.size.height as i32,
                               Color(self.color.0));

        let font = inkview_sys::open_font("Roboto", 50, 1);
        set_font(font, inkview_sys::Color::BLACK);

        let result = inkview_sys::draw_text_rect(self.rect.origin.x as i32, self.rect.origin.y as i32,
                                                 self.rect.size.width as i32, self.rect.size.height as i32,
                                                 &*self.text,
                                                 inkview_sys::c_api::ALIGN_CENTER | inkview_sys::c_api::VALIGN_MIDDLE);
    }


    fn process_event(&mut self, ctx: &mut AppContext, event: UIEvent) {
        self.is_clicked = false;
        if let UIEvent::PointerDown{x, y} = event {
            if self.rect.contains(x, y){
                self.is_clicked = true;

            }
        }

    }
}
