use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestBody {
    pub name: String,
    pub age: i32
}