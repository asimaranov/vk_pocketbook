use crate::ui_engine::app_context::AppContext;
use crate::ui_engine::color::UIColor;
use crate::ui_engine::events::UIEvent;
use crate::ui_engine::geometry::UIRect;
use crate::ui_engine::ui_view::UIView;
use crate::views::button_view::ButtonView;
use crate::views::geometry::FilledRectView;


pub struct MainScreen {
    rect: UIRect,
    rect_view: FilledRectView,
    button_view: ButtonView,
}

impl MainScreen {
    pub(crate) fn new(rect: UIRect) -> Self {
        return Self {
            rect,
            rect_view: FilledRectView::new(UIRect::new(100, 100, 300, 100),  UIColor::BLACK),
            button_view: ButtonView::new(UIRect::new(100, 200, 300, 100),
                                         "Ok".to_string(),
                                         UIColor::BLACK),
        };
    }
}

impl UIView for MainScreen {
    fn get_rect(&self) -> UIRect {
        self.rect
    }

    fn render(&self, ctx: &mut AppContext ) {
        //let panel_height = inkview_sys::panel_height();


        self.rect_view.render(ctx);
        self.button_view.render(ctx)
    }

    fn process_event(&mut self, ctx: &mut AppContext, event: UIEvent) {
        todo!()
    }
}