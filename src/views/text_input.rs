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



pub struct TextInputView {
    pub rect: UIRect,
    pub label: String,
    input_pointer: *const u8
}

impl TextInputView {
    pub fn new(rect: UIRect, label: String) -> Self {
        return Self {
            rect,
            label,
            input_pointer: 0 as *const u8,

        };
    }
    pub fn input_text(&self) -> Option<String> {
        return if self.input_pointer == 0 as *const u8 {
            None
        } else {
            let c_string = unsafe{ CStr::from_ptr(self.input_pointer) };
            Some(c_string.to_string_lossy().parse().unwrap())
        };
    }


}

impl UIView for TextInputView {
    fn get_rect(&self) -> UIRect {
        self.rect
    }

    fn render(&self, ctx: &mut AppContext) {
        inkview_sys::draw_rect(self.rect.origin.x as i32, self.rect.origin.y as i32,
                               self.rect.size.width as i32, self.rect.size.height as i32,
                               Color::BLACK);

        let font = inkview_sys::open_font("Roboto", 50, 1);
        set_font(font, inkview_sys::Color::BLACK);
        let text = self.input_text();
        let is_empty = text.is_none();
        //ctx.log(&format!("Going to render text: {:?}", text));
        let display_text = text.unwrap_or("|".to_string());
        //if let Some(text) = text{

        let result = inkview_sys::draw_text_rect(self.rect.origin.x as i32, self.rect.origin.y as i32,
                                                 self.rect.size.width as i32, self.rect.size.height as i32,
                                                 &*display_text,
                                                 (if is_empty {inkview_sys::c_api::ALIGN_LEFT} else { inkview_sys::c_api::ALIGN_CENTER }) | inkview_sys::c_api::VALIGN_MIDDLE);

        //}
    }

    fn process_event(&mut self, ctx: &mut AppContext, event: UIEvent) {
        if let UIEvent::PointerDown{x, y} = event {
            if self.rect.contains(x, y){
                inkview_sys::open_keyboard_ex(&*self.label, &*self.input_text().unwrap_or("".to_string()), 30, &mut self.input_pointer);

            }
        }

    }
}





pub struct TextPasswordView {
    pub rect: UIRect,
    pub label: String,
    input_pointer: *const u8
}

impl TextPasswordView {
    pub fn new(rect: UIRect, label: String) -> Self {
        return Self {
            rect,
            label,
            input_pointer: 0 as *const u8,

        };
    }
    pub fn input_text(&self) -> Option<String> {
        return if self.input_pointer == 0 as *const u8 {
            None
        } else {
            let c_string = unsafe{ CStr::from_ptr(self.input_pointer) };
            Some(c_string.to_string_lossy().parse().unwrap())
        };
    }


}

impl UIView for TextPasswordView {
    fn get_rect(&self) -> UIRect {
        self.rect
    }

    fn render(&self, ctx: &mut AppContext) {
        inkview_sys::draw_rect(self.rect.origin.x as i32, self.rect.origin.y as i32,
                               self.rect.size.width as i32, self.rect.size.height as i32,
                               Color::BLACK);

        let font = inkview_sys::open_font("Roboto", 50, 1);
        set_font(font, inkview_sys::Color::BLACK);
        let text = self.input_text();
        let is_empty = text.is_none();
        //ctx.log(&format!("Going to render text: {:?}", text));
        let mut display_text = text.map(|x|"*".repeat(x.len())).unwrap_or("|".to_string());
        if display_text.is_empty(){
            display_text = "|".to_string()
        }
        //if let Some(text) = text{

        let result = inkview_sys::draw_text_rect(self.rect.origin.x as i32, self.rect.origin.y as i32,
                                                 self.rect.size.width as i32, self.rect.size.height as i32,
                                                 &*display_text,
                                                 (if is_empty {inkview_sys::c_api::ALIGN_LEFT} else { inkview_sys::c_api::ALIGN_CENTER }) | inkview_sys::c_api::VALIGN_MIDDLE);

        //}
    }

    fn process_event(&mut self, ctx: &mut AppContext, event: UIEvent) {
        if let UIEvent::PointerDown{x, y} = event {
            if self.rect.contains(x, y){
                inkview_sys::open_keyboard_ex(&*self.label, &*self.input_text().unwrap_or("".to_string()), 30, &mut self.input_pointer);

            }
        }

    }
}
