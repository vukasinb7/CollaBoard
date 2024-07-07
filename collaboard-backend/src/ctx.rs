use serde::{Deserialize, Serialize};

#[derive(Clone, Debug,Serialize,Deserialize)]
pub struct Ctx {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}
