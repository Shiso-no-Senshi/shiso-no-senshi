#![allow(non_snake_case)]

pub mod file_navigator;

use dioxus::prelude::*;

pub trait Navigator {
    fn Navigator(&self) -> Element;
    fn get_icon(&self) -> svg::Document;
}
