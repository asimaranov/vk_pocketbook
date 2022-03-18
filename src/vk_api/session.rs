use std::iter;
use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::vk_api::error::VkAuthError;
use ureq;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct VkSession {
    pub token: String,
    pub user_id: Option<u64>,

}

impl VkSession {
    pub fn from_token(token: &str) -> Self {
        return VkSession {
            token: token.to_string(),
            user_id: None,

        };
    }

    pub fn new(token: String, user_id: Option<u64>) -> Self {
        return VkSession { token, user_id };
    }

    pub fn token(&self) -> String {
        self.token.clone()
    }


    pub fn auth_as_mobile_with_client(login: String, password: String) -> Result<Self, VkAuthError> {
        let mut rng = rand::thread_rng();

        let len = rng.gen_range(5, 8);

        let random_device_id: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(len)
            .collect();


        let r = ureq::post("https://oauth.vk.com/token")
            .send_form(&[
                ("scope", "all"),
                ("client_id", "2274003"),
                ("client_secret", "hHbZxrka2uZ6jB1inYsH"),
                ("2fa_supported", "1"),
                ("lang", "ru"),
                ("device_id", &random_device_id),
                ("grant_type", "password"),
                ("username", &login),
                ("password", &password)
            ]).map_err(|e|VkAuthError::ConnectionError(e.to_string()))?;

        let resp_string = r.into_string().map_err(|e|VkAuthError::ConnectionError(e.to_string()))?;

        let json: serde_json::Value = serde_json::from_str(&resp_string).map_err(|e|VkAuthError::UnknownError{ message: "Unable to parse".to_string(), json_string: resp_string })?;
        let obj = json.as_object().unwrap();

        if obj.contains_key("error") {
            Err(VkAuthError::AuthError {
                error_name: obj["error"].as_str().unwrap().to_string(),
                error_type: if obj.contains_key("error_type") { obj["error_type"].as_str().unwrap().to_string() } else { obj["error"].as_str().unwrap().to_string() },
            })
        } else {
            Ok(VkSession::new(obj["access_token"].as_str().unwrap().to_string(), Some(obj["user_id"].as_u64().unwrap())))
        }
    }
}





