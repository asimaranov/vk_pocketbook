use super::session::VkSession;
use num::Num;
use ureq;
use std::collections::HashMap;
use std::fmt::Display;
use std::iter::IntoIterator;
use serde_json::{Value, Map};
use std::fs::read_to_string;
use serde_derive::Deserialize;
use serde_derive::Serialize;


use std::io::Read;
use std::vec::Vec;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use std::sync::Arc;
use std::collections::hash_map::RandomState;
use std::iter;
use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::vk_api::error::VkAuthError;
use crate::vk_api::error::VkApiError;

#[derive(Clone)]
pub struct VkApiErrorCallback {
    pub error_code: u32,

}

#[derive(Clone, Debug)]
pub struct VkApi {

    pub session: VkSession,
    pub version: String,

}

impl VkApi {
    pub fn new(session: VkSession) -> Self {
        VkApi { session, version: "5.124".to_string() }
    }
    pub fn with_default_http_client(session: VkSession) -> Self {
        VkApi { session, version: "5.124".to_string() }
    }
}

#[derive(Clone)]
pub enum VkApiArg {
    Integer(i64),
    Float(f32),
    String(String),
    Bool(bool),

    IntArray(Vec<i64>),
    FloatArray(Vec<f32>),
    StringArray(Vec<String>),
    BoolArray(Vec<bool>),
}

impl VkApiArg {
    fn serialize(&self) -> String {
        match &self {
            VkApiArg::Integer(i) => i.to_string(),
            VkApiArg::Float(f) => f.to_string(),
            VkApiArg::String(s) => s.to_string(),
            VkApiArg::Bool(b) => b.to_string(),
            VkApiArg::IntArray(ia) => {
                let strings: Vec<String> = ia.iter().map(|x| x.to_string()).collect();
                strings.join(",")
            }
            _ => panic!("Not implemented")
        }
    }
}

/*trait VkApiCallable {
    fn call(&self, method: &str, params: HashMap<String, VkApiArg>) -> Result<Value, VkApiError>;
}*/ // Async functions in traits are not currently supported


impl VkApi {
    pub fn call(&self, method: &str, params: HashMap<String, VkApiArg>) -> Result<Value, VkApiError> {
        let mut params_map: HashMap<String, String> = HashMap::new();

        params_map.insert("access_token".to_string(), self.session.token());
        params_map.insert("v".to_string(), self.version.clone());

        for (key, value) in params {
            params_map.insert(key, value.serialize());
        }
        let pairs = params_map.iter().map(|(x, y)| (x.as_str(), y.as_str())).collect::<Vec<(&str, &str)>>();
        let resp = ureq::post(&format!("https://api.vk.com/method/{}", method)).send_form(&pairs);

        let resp = resp.map_err(|e| VkApiError::ConnectionError(e.to_string()))?;
        let resp_text = resp.into_string().map_err(|e| VkApiError::ConnectionError(format!("Unable to get resp text: {:?}", e)))?;
        let resp_json: Value = serde_json::from_str(&resp_text).map_err(|e|VkApiError::UnknownError{ message: "Unable to parse json".to_string(), json_string: resp_text.clone() })?;


            if let Value::Object(map) = resp_json {
                if map.contains_key("response") {
                    return Ok(map["response"].clone());
                } else if map.contains_key("error") {
                    let error = &map["error"];
                    if let Value::Object(error_info) = error {
                        if let (Some(Value::Number(code)), Some(Value::String(msg))) = (error_info.get("error_code"), error_info.get("error_msg")) {
                            return Err(VkApiError::ApiError { error_code: code.as_u64().unwrap() as u32, error_msg: msg.clone(), json_string: resp_text.clone() });
                        }
                    }
                    return Err(VkApiError::UnknownError { message: format!("Unable to collect error info: {:?}", resp_text.clone()), json_string: resp_text.clone() });
                }
            }


        return Err(VkApiError::UnknownError { message: "Something weird received".to_string() ,json_string: resp_text.clone() });
    }
}

pub trait VkApiType {
    fn get_enum_type(&self) -> VkApiArg;
}

impl VkApiType for i64 {
    fn get_enum_type(&self) -> VkApiArg {
        VkApiArg::Integer(*self)
    }
}

impl VkApiType for Vec<i64> {
    fn get_enum_type(&self) -> VkApiArg {
        VkApiArg::IntArray(self.clone())
    }
}

impl VkApiType for bool {
    fn get_enum_type(&self) -> VkApiArg {
        VkApiArg::Bool(*self)
    }
}

impl VkApiType for String {
    fn get_enum_type(&self) -> VkApiArg {
        VkApiArg::String(self.clone())
    }
}

impl VkApiType for &'static str {
    fn get_enum_type(&self) -> VkApiArg {
        VkApiArg::String(self.to_string())
    }
}
