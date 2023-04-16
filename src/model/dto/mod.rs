use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AddAppDTO {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct AddApiDTO {
    pub api: String,
}
