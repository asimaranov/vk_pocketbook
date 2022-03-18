use std::cell::RefCell;
use kv::{Config, Store};

use crate::ui_engine::ui_view::UIView;
use crate::views::log_panel::LogPanelView;

pub enum ScreenCommand{
    PushScreen(RefCell<Box<dyn UIView>>),
    PopScreen,
    None
}

pub struct AppContext{
    pub log_panel: LogPanelView,
    pub cache: Store,
    pub screen_command: ScreenCommand,


}
impl AppContext{
    pub fn log(&mut self, message: &str){

        self.log_panel.log(message)
    }
    pub fn debug(&mut self, message: &str){
        self.log_panel.log(message)
    }

    pub fn new(log_panel: LogPanelView) -> Self{

        let mut cfg = Config::new("/mnt/ext1/vk_app_cache.db");

        let store = Store::new(cfg).expect("Unable to create cache");

        return Self{
            log_panel,
            cache: store,
            screen_command: ScreenCommand::None

        }
    }
    pub fn pop_screen(&mut self, screen: Box<dyn UIView>){
        self.screen_command = ScreenCommand::PopScreen
    }

    pub fn open_screen(&mut self, screen: Box<dyn UIView>){
        self.screen_command = ScreenCommand::PushScreen(RefCell::new(screen));
        inkview_sys::repaint();
    }


}