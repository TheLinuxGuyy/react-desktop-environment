#![allow(unused)]
use wasm_bindgen::prelude::*;
struct RsOnly;
impl RsOnly{
    fn browser_req() -> Result<(),Box<dyn Error>>{
        let page=reqwest::blocking::get("https://httpbin.org/ip")?.text?;
        page
    }
}

#[wasm_bindgen]
fn browser_req()-> String{
    resultrequest=RsOnly::browser_req.to_string();
    resultrequest
}

fn main(){
    
}