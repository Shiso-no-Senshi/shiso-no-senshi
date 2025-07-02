pub mod message;

use dioxus::prelude::*;
pub trait Editor {
    fn Editor(&self) -> Element;

    fn set_content(self: &mut Self, content: Vec<u8>);
    fn get_content(&self) -> Vec<u8>;
}

pub trait Navigator {
    fn Navigator(&self) -> Element;
    fn Icon(&self) -> svg::Document;

    fn new(on_navigate_callback: Option<fn(Vec<u8>)>) -> Self;

    fn navigate(location: &str) -> Result<Vec<u8>, NavigateError>;

    fn restore(location: &str) -> Vec<u8>;
    fn persist(location: &str, content: &Vec<u8>);
}

pub struct NavigateError {
    location: String,
    message: String,
}

impl NavigateError {
    pub fn new(location: &str, message: &str) -> Self {
        Self {
            location: String::from(location),
            message: String::from(message),
        }
    }
}
