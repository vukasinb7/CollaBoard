use serde::{Deserialize, Serialize};

#[derive(Clone, Debug,Serialize,Deserialize)]
pub struct Ctx {
    pub exp: usize,
    pub iat: usize,
    pub email: String,
}

//Constructor
impl Ctx {
    pub fn new(exp: usize, iat: usize, email: String) -> Self {
        Self { exp, iat, email }
    }
}

//Property accessors
impl Ctx {
    pub fn email(&self) -> String { self.email.clone() }
    pub fn iat(&self) -> usize { self.iat }
    pub fn exp(&self) -> usize { self.exp }
}