use serde::Serialize;



#[derive(Serialize)]
pub struct Description {
    pub a: String,
    pub b: i32,
}