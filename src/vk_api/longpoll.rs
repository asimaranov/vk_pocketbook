#[macro_use]
use super::api::{VkApi, VkApiArg, VkApiType};
use serde_json::{to_string, Value};
/*
pub enum LongpollMode {
    GetAttachments = 1,
    GetExtended = 1 << 3,
    GetPts = 1 << 5,
    GetExtraOnline = 1 << 6,
    GetRandomId = 1 << 7,
    ALL = (LongpollMode::GetAttachments as u16 + LongpollMode::GetExtended as u16 + LongpollMode::GetPts as u16 + LongpollMode::GetExtraOnline as u16 + LongpollMode::GetRandomId as u16) as isize

}

pub struct LongPoll{
    api: VkApi,
    is_connected: bool,

    need_pts: bool,
    ts: Option<u64>,
    pts: Option<u64>,
    server: Option<String>,
    key: Option<String>,
    mode: LongpollMode,
    wait: u32
}



impl LongPoll{
    pub fn new(api: VkApi, need_pts: bool, mode: LongpollMode, wait: u32) -> Self{
        return LongPoll{
            api,
            is_connected: false,

            need_pts,
            ts: None,
            pts: None,
            server: None,
            key: None,
            mode,
            wait
        }
    }

    pub fn with_pts(api: VkApi) -> Self{
        Self::new(api, true, LongpollMode::ALL, 25)
    }

    pub async fn update_server(&mut self){
        let response = self.api.call("messages.getLongPollServer", vk_args!("lp_version" => 3, "need_pts" => self.need_pts)).await.ok().expect("Vk api error");

        let ts = &response["ts"].as_u64().unwrap();
        let server = response["server"].as_str().unwrap().to_string();
        let key = response["key"].as_str().unwrap().to_string();

        if self.need_pts{
            let pts = &response["pts"].as_u64().unwrap();
            self.pts = Some(*pts);
        }


        self.ts = Some(*ts);
        self.server = Some(server);
        self.key = Some(key);

        println!("Server updated. Server: {server:?}, ts: {ts:?}, pts: {pts:?}", server=self.server, ts=ts, pts=self.pts)

    }
    /*
    pub async fn get_events(&mut self) -> Vec<Value>{
        if self.key == None || self.ts == None{
            self.update_server().await;
        }

        let pts_str: String;

        let mut params = hashmap!("act" => "a_check".to_string(), "key" => self.key.as_ref().unwrap().clone(), "ts" => self.ts.unwrap().to_string(), "wait" => self.wait.to_string() );

        if self.pts != None{
            params.insert("pts", self.pts.unwrap().to_string());
        }

         let resp = self.client
             .get(&format!("http://{}", &self.server.as_ref().unwrap()))
             .query(&params).send();

        let events = resp.await.unwrap().text().await.unwrap();

        let resp: Value = serde_json::from_str(&events).unwrap();
        let resp_obj = resp.as_object().unwrap();

        self.ts = Some(resp_obj["ts"].as_u64().unwrap());

        let updates = resp_obj["updates"].as_array().unwrap();

        updates.clone()

    }*/

}

*/