#![allow(unused)]
use wasm_bindgen::prelude::*;
use std::error::Error;
use std::net;
struct RsOnly;
use std::str::StrSlice;
use std::io::{TcpListener, TcpStream};
use std::io::net::ip::SocketAddr;
use std::io::{Listener, Writer, Acceptor, Buffer};
use std::task::spawn;
use std::io::BufferedStream;
use std::comm::SharedChan;
impl RsOnly{
    fn browser_req(String url){
        if url.starts_with("https"){
            let port = "80";
        } else if url.starts_with("http"){
            let port = "443";
            for(x=0;x<7;x++){
                let url=url.remove(0);
            }
        }
        let addr: SocketAddr = from_str("{}:{}",url,port).unwrap();
        let listener=TcpListener::bind(addr).unwrap();
        let mut accepter=listener.listen().unwrap();
    } //use sockets on port
}

#[wasm_bindgen]
pub fn browser_req()-> String{
    let resultrequest=RsOnly::browser_req(url).to_string();
    resultrequest
}

fn main(){
    
}