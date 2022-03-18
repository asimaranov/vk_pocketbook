use std::error::Error;
use std::fmt::{Debug, Formatter};
use serde_json::{Value, Map};

#[derive(Debug, Clone)]
pub enum  VkAuthError{
    ConnectionError(String),
    AuthError{error_name: String, error_type: String},
    UnknownError{message: String, json_string: String}

}






#[derive(Debug, Clone)]
pub enum VkApiError{
    ApiError{error_code: u32, error_msg: String, json_string: String},
    ConnectionError(String),
    UnknownError{message: String, json_string: String}
}


