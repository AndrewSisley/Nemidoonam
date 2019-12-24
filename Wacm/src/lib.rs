extern crate serde;

use serde::{Serialize};

#[derive(Serialize)]
pub struct Component {
    pub css: String,
    pub html: String,
}