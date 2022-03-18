use std::cmp::{max, min};
use inkview_sys::{Color, PbBmp};
use crate::ui_engine::app_context::AppContext;
use crate::ui_engine::events::UIEvent;
use crate::ui_engine::geometry::{UIRect, UISize};
use crate::ui_engine::ui_view::UIView;

pub struct GridItem {
    title: String,
    payload: String,
    image: PbBmp,
}


impl GridItem {
    pub(crate) fn new(title: String, payload: String, image: PbBmp) -> Self {
        Self { title, payload, image }
    }
}

pub struct GridView {
    rect: UIRect,
    grid_item_size: UISize,
    items: Vec<GridItem>,
    page_id: u32,
    pub selected_item_payload: Option<String>
}

impl GridView {
    pub(crate) fn new(rect: UIRect, grid_item_size: UISize, items: Vec<GridItem>) -> Self {
        Self {
            rect,
            grid_item_size,
            items,
            page_id: 0,
            selected_item_payload: None
        }
    }
}

impl UIView for GridView {
    fn get_rect(&self) -> UIRect {
        self.rect
    }

    fn render(&self, ctx: &mut AppContext) {
        let min_offset = 10;
        let col_additional_offset = (self.rect.size.width % (self.grid_item_size.width + 2 * min_offset)) / 2;
        let col_items_count = self.rect.size.width / (self.grid_item_size.width + 2 * min_offset);
        let row_items_count = self.rect.size.height / (self.grid_item_size.height + 2 * min_offset);
        let page_size = row_items_count * col_items_count;


        let page_items = &self.items[(self.page_id * page_size) as usize..min(((self.page_id + 1) * page_size) as usize, self.items.len())];

        for (i, page_item) in page_items.iter().enumerate() {

            let (col, row) = (i as u32 % col_items_count, i as u32 / col_items_count);

            let item_rect = UIRect::new(
                self.rect.x() + col * (self.grid_item_size.width + 2 * min_offset) + min_offset + col_additional_offset,
                self.rect.y() + row * (self.grid_item_size.height + 2 * min_offset) + min_offset, self.grid_item_size.width - 2 * min_offset,
                self.grid_item_size.height - 2 * min_offset );
            inkview_sys::draw_rect(item_rect.x() as i32, item_rect.y() as i32, item_rect.w() as i32, item_rect.h() as i32, Color::BLACK);

            let text_rect_h = 30;

            inkview_sys::draw_text_rect(item_rect.x() as i32, (item_rect.y() + item_rect.h() - 50) as i32, item_rect.w() as i32, text_rect_h, &page_item.title, inkview_sys::c_api::ALIGN_CENTER | inkview_sys::c_api::VALIGN_MIDDLE);
        }
        ctx.log(&format!("Col: {:?}, row: {:?}", col_items_count, row_items_count));

    }


    fn process_event(&mut self, ctx: &mut AppContext, event: UIEvent) {
        if let UIEvent::PointerDown{x, y} = event {
            if self.rect.contains(x, y){
                let min_offset = 10;
                let col_additional_offset = (self.rect.size.width % (self.grid_item_size.width + 2 * min_offset)) / 2;
                let col_items_count = self.rect.size.width / (self.grid_item_size.width + 2 * min_offset);
                let row_items_count = self.rect.size.height / (self.grid_item_size.height + 2 * min_offset);
                let page_size = row_items_count * col_items_count;

                let page_items = &self.items[(self.page_id * page_size) as usize..min(((self.page_id + 1) * page_size) as usize, self.items.len())];

                for (i, page_item) in page_items.iter().enumerate() {

                    let (col, row) = (i as u32 % col_items_count, i as u32 / col_items_count);
                    let item_rect = UIRect::new(
                        self.rect.x() + col * (self.grid_item_size.width + 2 * min_offset) + min_offset + col_additional_offset,
                        self.rect.y() + row * (self.grid_item_size.height + 2 * min_offset) + min_offset, self.grid_item_size.width - 2 * min_offset,
                        self.grid_item_size.height - 2 * min_offset );
                    if item_rect.contains(x, y){
                        self.selected_item_payload = Some(page_item.payload.clone());
                    }
                }
            }
        }
    }
}