use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AddAppDTO {
    pub app: String,
}

#[derive(Deserialize, Debug)]
pub struct AddApiDTO {
    pub api: String,
}
