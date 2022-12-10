use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TaskRequest {
    pub name: String,
    pub description: String,
}