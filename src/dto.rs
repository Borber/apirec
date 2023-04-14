use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AddAppDTO {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct AddApiDTO {
    pub api: String,
}

#[derive(Deserialize, Debug)]
pub struct GetApiDTO {
    pub apis: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct PostApiDTO {
    pub apis: Vec<String>,
}
