use crate::ui_engine::app_context::AppContext;
use crate::ui_engine::events::UIEvent;
use crate::ui_engine::geometry::UIRect;

pub trait UIView{
    fn get_rect(&self) -> UIRect;

    fn render(&self, ctx: &mut AppContext);
    fn process_event(&mut self, ctx: &mut AppContext, event: UIEvent){}
}

