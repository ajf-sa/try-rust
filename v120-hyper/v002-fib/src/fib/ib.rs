use std::{collections::HashMap, error, io};

use tokio::net::TcpStream;



pub type HandlerFn = fn(&str) -> Result<(),io::Error>;


pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    PATCH,
}

#[derive(Clone, Copy)]
pub struct App {
    client :&'static  str,
   
}


impl App{
    pub fn new(cleint:&'static  str) -> Self {
        App { client:cleint}
    }
    fn register(self,method :Method,path :&str,handler : HandlerFn) {
      
        let h = handler;
        h(self.client);
    }
    pub fn add(self,method :Method,path :&str,handler : HandlerFn) {
            self.register(method, path, handler)
    }

    pub fn get(self,path: &str ,handler : HandlerFn) {
        self.add(Method::GET,path, handler)
    } 

    pub fn post(self,path: &str ,handler : HandlerFn) {
        self.add(Method::POST,path, handler)
    } 
   
}