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










pub struct LogPanelView {
    rect: UIRect,
    log: String,
    is_changed: bool,

}

impl LogPanelView {
    pub fn new(rect: UIRect, log: String) -> Self {
        return Self{
            rect,
            log,
            is_changed: true
        }
    }
}

impl UIView for LogPanelView {
    fn get_rect(&self) -> UIRect {
        self.rect
    }

    fn render(&self, ctx: &mut AppContext) {
        self.render_no_ctx()
    }

    fn process_event(&mut self, ctx: &mut AppContext, event: UIEvent) {
        self.is_changed = false;
    }

}

impl LogPanelView {

    pub fn log(&mut self, message: &str){

        let date = Local::now();


        self.is_changed = true;
        self.log.push_str(&format!("{} {} \n", date.format("[%Y-%m-%d %H:%M:%S]"), (message.to_string())))
    }
    pub fn render_no_ctx(&self){
        //inkview_sys::draw_rect(self.rect.origin.x as i32, self.rect.origin.y as i32,
        //                       self.rect.size.width as i32, self.rect.size.height as i32,
        //                       Color::BLACK);

        let font = inkview_sys::open_font("Roboto", 40, 1);
        set_font(font, inkview_sys::Color::BLACK);

        let l = self.log.split( '\n').collect::<Vec<&str>>().iter().rev().take(11).rev().map(|&x|x).collect::<Vec<&str>>().join("\n");





        let result = inkview_sys::draw_text_rect(self.rect.origin.x as i32, self.rect.origin.y as i32,
                                                 self.rect.size.width as i32, self.rect.size.height as i32,
                                                 &*l,
                                                 inkview_sys::c_api::ALIGN_LEFT | inkview_sys::c_api::VALIGN_BOTTOM);
        if self.is_changed{
            inkview_sys::partial_update(self.rect.x() as i32, self.rect.y() as i32, self.rect.w() as i32, self.rect.h() as i32)
        }
    }
}