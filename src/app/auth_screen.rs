use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use inkview_sys::{PbBmp, include_bmp};
use crate::ui_engine::app_context::AppContext;
use crate::ui_engine::color::UIColor;
use crate::ui_engine::events::UIEvent;
use crate::ui_engine::geometry::UIRect;
use crate::ui_engine::ui_view::UIView;
use crate::views::button_view::ButtonView;
use crate::views::geometry::FilledRectView;
use crate::views::labeled_text_input::{LabeledPasswordInputView, LabeledTextInputView};

use crate::views::text_input::TextInputView;
use tinybmp::{Bmp, FileType, Header, Pixel};
use crate::views::image_view::ImageView;
use crate::views::text_view::TextView;

use std::sync::mpsc::{channel, Receiver};
use kv::Json;
use crate::app::groups_screen::GroupsScreen;
use crate::vk_api::error::VkAuthError;
use crate::vk_api::session::VkSession;

enum AuthRequest {
    Error{reason: String},
    Ok{token: String}
}


pub struct AuthScreen {
    rect: UIRect,
    login_input: LabeledTextInputView,
    password_input: LabeledPasswordInputView,

    button_view: ButtonView,
    redraw: bool,
    is_first_draw: bool,
    vk_logo: ImageView,
    info_text: TextView,
    auth_request_channel: Option<Receiver<Result<VkSession, VkAuthError>>>,


}

impl AuthScreen {
    pub fn new(rect: UIRect) -> Self {
        let vk_logo = ImageView::new(UIRect::new(rect.w() / 2 - (711 / 2), 100, 711, 123), include_bmp!("sprites/vk_logo_1.bmp").into());

        let text_padding = 50;
        let info_text = TextView::new(UIRect::new(text_padding, 300, rect.w() - 2 * text_padding, 170), "Клиент имеет открытый исходный код, который можно найти в репозитории https://github.com/asimaranov/vk_pocketbook.git".to_string(), 40);

        let input_w = 500;
        let input_x = rect.w() / 2 - (input_w / 2);

        let login_input = LabeledTextInputView::new(UIRect::new(input_x, 550, input_w, 100), "Login".to_string());
        let password_input = LabeledPasswordInputView::new(UIRect::new(input_x, 670, input_w, 100), "Password".to_string());

        let button_view = ButtonView::new(
            UIRect::new(rect.w() / 2 - (300 / 2), 800, 300, 100),
            "Ok".to_string(),
            UIColor::BLACK,
        );

        return Self {
            rect,
            login_input,
            password_input,

            button_view,
            redraw: false,
            is_first_draw: true,

            vk_logo,
            info_text,
            auth_request_channel: None
        };
    }
}

impl UIView for AuthScreen {
    fn get_rect(&self) -> UIRect {
        self.rect
    }

    fn render(&self, ctx: &mut AppContext) {
        //let panel_height = inkview_sys::panel_height();
        if self.redraw {
            inkview_sys::clear_screen();
            inkview_sys::set_panel_type(inkview_sys::PanelType::DISABLED);
        }

        self.login_input.render(ctx);
        self.password_input.render(ctx);

        self.button_view.render(ctx);




        self.vk_logo.render(ctx);
        self.info_text.render(ctx);
        if self.redraw {
            inkview_sys::full_update();
        }
    }

    fn process_event(&mut self, ctx: &mut AppContext, event: UIEvent) {
        if self.is_first_draw {
            let bucket = ctx.cache.bucket::<&str, Json<VkSession>>(None).expect("Unable to access cache");
            if let Ok(Some(session)) = bucket.get("vk_auth"){
                ctx.open_screen(Box::new(GroupsScreen::new(self.rect, session.0)))
            }else {
                ctx.log(&format!("Unable to find cache" ))
            }
            self.is_first_draw = false;
        }
        if let UIEvent::AsyncTaskFinished{task_id, task_sub_id} = event {
            if task_id == 100 && task_sub_id == 500{
                if let Some(channel) = &self.auth_request_channel{
                    let resp = channel.recv().unwrap();
                    match resp {
                        Ok(session) => {
                            inkview_sys::message(inkview_sys::c_api::Icon::INFORMATION, "Ответ сервера", &format!("Авторизация успешна: {:?}", &session), 5000);
                            let bucket = ctx.cache.bucket::<&str, Json<VkSession>>(None).expect("Unable to get cache file");
                            bucket.set("vk_auth", Json(session.clone()));
                            bucket.flush();
                            ctx.open_screen(Box::new(GroupsScreen::new(self.rect, session)));

                        },
                        Err(err) => {
                            inkview_sys::message(inkview_sys::c_api::Icon::ERROR, "Ответ сервера", &format!("Ошибка авторизации: {:?}", err), 5000);
                        }
                    }
                }

            }
        }
        self.redraw = event == UIEvent::Repaint;

        self.login_input.process_event(ctx, event);
        self.password_input.process_event(ctx, event);
        self.button_view.process_event(ctx, event);


        if self.button_view.is_clicked() {
            let (tx, rx) = channel();
            let login = self.login_input.input_text().unwrap_or("".to_string());
            let password = self.password_input.input_text().unwrap_or("".to_string());

            thread::spawn(move || {

                let event_handler = inkview_sys::get_event_handler();

                let resp = VkSession::auth_as_mobile_with_client(login, password);

                tx.send(resp);

                inkview_sys::send_event(event_handler, inkview_sys::c_api::Event::ASYNC_TASK_FINISHED as i32, 100, 500)
            });
            self.auth_request_channel = Some(rx);

        };
    }
}