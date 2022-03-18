#[macro_use] extern crate maplit;

mod ui_engine;
mod views;
mod app;
#[macro_use] mod vk_api;



use std::cell::RefCell;
use std::{mem, panic};
use inkview_sys::c_api;
use std::sync::{Arc, Mutex};
use inkview_sys::c_api::Event;
use backtrace::Backtrace;

use crate::ui_engine::ui_view::UIView;
use crate::ui_engine::events::UIEvent;

use crate::app::main_screen::MainScreen;
use crate::app::auth_screen::AuthScreen;
use crate::ui_engine::app_context::{AppContext, ScreenCommand};

use crate::ui_engine::geometry::UIRect;

use inkview_sys::{ include_bmp, PbBmp};
use crate::views::log_panel::LogPanelView;

struct PbHandler{
    ctx: AppContext,
    iter_num: u32,
    screens: Vec<RefCell<Box<dyn UIView>>>,

}

impl inkview_sys::EventHandler for PbHandler {
    fn handle_event(&mut self, event: Event, par1: i32, par2: i32) -> i32 {
        self.ctx.log(&format!("event: {:?}({}, {})", event, par1, par2));
        //inkview_sys::clear_screen();

        match event {

            Event::SHOW | Event::POINTERDOWN | Event::POINTERMOVE | Event::POINTERUP | Event::ASYNC_TASK_FINISHED => {
                //self.ctx.log(&format!("Log: {}", self.iter_num));
                self.iter_num += 1 ;

                let event = match event {
                    Event::SHOW => {
                            UIEvent::Repaint

                    },

                    Event::POINTERDOWN => {
                        UIEvent::PointerDown{x: par1 as u32, y: par2 as u32}
                    },
                    Event::POINTERMOVE => {
                        UIEvent::PointerMove{x: par1 as u32, y: par2 as u32}
                    }
                    Event::POINTERUP => {
                        UIEvent::PointerUp{x: par1 as u32, y: par2 as u32}
                    },
                    Event::ASYNC_TASK_FINISHED => {
                        UIEvent::AsyncTaskFinished{task_id: par1 as u32, task_sub_id: par2 as u32}
                    },


                    _ => UIEvent::None
                };

                self.screens.last().unwrap().borrow_mut().process_event(&mut self.ctx, event);

                let mut command = ScreenCommand::None;

                mem::swap(&mut self.ctx.screen_command, &mut command);

                if let ScreenCommand::PushScreen(screen) = command {
                    self.screens.push(screen)
                }

                self.screens.last().unwrap().borrow().render(&mut self.ctx);
                self.ctx.log_panel.render_no_ctx();

            },

            Event::KEYPRESS => unsafe {
                if par1 == inkview_sys::c_api::Key::MENU as i32{
                    inkview_sys::clear_screen();
                    self.screens.last().unwrap().borrow_mut().process_event(&mut self.ctx, UIEvent::None);

                    self.ctx.log_panel.render_no_ctx();

                    self.screens.last().unwrap().borrow().render(&mut self.ctx);

                    inkview_sys::full_update();

                }else {
                    inkview_sys::c_api::CloseApp();

                }
            }

            _ => {}
        }
        0
    }
}



fn main() {

    unsafe{
        inkview_sys::c_api::OpenScreen();
    }
    panic::set_hook(Box::new(|info| {
        let backtrace = Backtrace::new();
        //let mut file = File::open("/var/tmp/panic_log.txt").unwrap();
        //file.write_all(format!("Panic! {:?} {:?}", info, backtrace).as_ref());
        unsafe{
            inkview_sys::c_api::OpenScreen();
        }
        inkview_sys::message(c_api::Icon::ERROR, "Panic", &format!("panic: {:?} {:?}", info, backtrace), 5);
        unsafe { c_api::CloseApp(); }
    }));



    let log_panel = LogPanelView::new(UIRect::new(0, 1300, 1404, 1872 - 1300), "Log:\n".to_string());

    let runner = PbHandler{ ctx: AppContext::new(log_panel),
        screens: vec![RefCell::new(Box::new(AuthScreen::new(UIRect::new(0, 0, 1404, 1872 ))))],
        iter_num: 0
    };



    let h: Arc<Mutex<dyn inkview_sys::EventHandler>> = Arc::new(Mutex::new(runner));


    inkview_sys::main(&h);


}
