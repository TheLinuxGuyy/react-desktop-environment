#![allow(unused)]
use wasm_bindgen::prelude::*;
use std::mem;
#[wasm_bindgen]
pub fn browser() -> String{
    let page=reqwest::blocking::get("https://httpbin.org/ip")?.text()?;
    let page = String::from_utf8(page.into()).unwrap();
    page;
}

fn main(){

}