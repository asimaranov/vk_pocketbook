use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use inkview_sys::{PbBmp, include_bmp};
use crate::ui_engine::app_context::AppContext;
use crate::ui_engine::color::UIColor;
use crate::ui_engine::events::UIEvent;
use crate::ui_engine::geometry::{UIRect, UISize};
use crate::ui_engine::ui_view::UIView;
use crate::views::button_view::ButtonView;
use crate::views::geometry::FilledRectView;
use crate::views::labeled_text_input::{LabeledPasswordInputView, LabeledTextInputView};

use crate::views::text_input::TextInputView;
use tinybmp::{Bmp, FileType, Header, Pixel};
use crate::views::image_view::ImageView;
use crate::views::text_view::TextView;

use std::sync::mpsc::{channel, Receiver};
use serde_json::Value;
use crate::vk_api::api::VkApi;
use crate::vk_api::error::{VkApiError, VkAuthError};
use crate::vk_api::session::VkSession;
use crate::vk_args;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::views::grid_view::{GridItem, GridView};
use crate::vk_api::api::VkApiType;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct GroupsGetResponse {
    pub count: i64,
    pub items: Vec<VkGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct VkGroup {
    pub id: i64,
    pub name: String,
    pub screen_name: String,
    pub is_closed: i64,
    #[serde(rename = "type")]
    pub group_type: String,
    pub photo_50: String,
    pub photo_100: String,
    pub photo_200: String,
}




pub struct GroupsScreen {
    rect: UIRect,
    vk_session: VkSession,
    is_first_draw: bool,
    vk_api: VkApi,

    grid_view: Option<GridView>

}


impl GroupsScreen {
    pub fn new(rect: UIRect, vk_session: VkSession) -> Self {

        return Self {
            rect,
            vk_session: vk_session.clone(),
            is_first_draw: false,
            vk_api: VkApi::new(vk_session),

            grid_view: None

        };
    }
}

impl UIView for GroupsScreen {
    fn get_rect(&self) -> UIRect {
        self.rect
    }

    fn render(&self, ctx: &mut AppContext) {

        if self.is_first_draw {
            inkview_sys::set_panel_type(inkview_sys::PanelType::DISABLED);
            inkview_sys::clear_screen();
        }

        if let Some(grid) = &self.grid_view{
            grid.render(ctx);

        }

        if self.is_first_draw {
            inkview_sys::full_update();
        }
    }

    fn process_event(&mut self, ctx: &mut AppContext, event: UIEvent) {

        self.is_first_draw = event == UIEvent::Repaint;
        if let (UIEvent::PointerDown, Some(grid_view)) = (event, &mut self.grid_view) {
            grid_view.process_event(ctx, event);
            if let Some(enabled_payload) = &grid_view.selected_item_payload{
                ctx.log(&format!("Selected group: {}", enabled_payload));

            }
        }
        if let (UIEvent::Repaint, None) = (event, &self.grid_view) {
            let resp = self.vk_api.call("groups.get", vk_args!("extended" => true));

            match resp {
                Ok(resp) => {

                    let resp: Result<GroupsGetResponse, _> = serde_json::from_value(resp);
                    match resp{
                        Ok(resp) => {
                            let groups_info = resp.items;

                            let mut grid_items = Vec::<GridItem>::new();

                            for group_info in groups_info{
                                grid_items.push(GridItem::new( group_info.name.clone(),  group_info.id.to_string(), include_bmp!("sprites/vk_logo_1.bmp").into() ));
                            }

                            self.grid_view = Some(GridView::new(self.rect, UISize { width: 400, height: 400 }, grid_items));

                        }
                        Err(err) => {
                            //inkview_sys::message(inkview_sys::c_api::Icon::ERROR, "Vk api parse error", &format!("{:?}", err), 5000);
                            //ctx.log(&format!("Vk api request error: {:?}, resp: {:?}", err, resp_copy))

                        }
                    }
                }
                Err(err) => {
                    inkview_sys::message(inkview_sys::c_api::Icon::ERROR, "Vk api request error", &format!("{:?}", err), 5000);
                    ctx.log(&format!("Vk api request error: {:?}", err))

                }
            }
        }
    }
}