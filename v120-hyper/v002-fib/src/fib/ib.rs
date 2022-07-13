use std::{collections::{HashMap, HashSet}, error, io, array, rc::Rc, sync::Mutex};

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

// #[derive(Clone,Copy)]
// pub struct Router {
//     routers: Vec<Box<String>>
// }

// impl Router {
//     pub fn new() -> Self {
//         Router {
//             routers: Vec::new(),
//         }
//     }
//  }



#[derive(Clone,Copy)]
pub struct App<'a>{
    client :&'a  str,
   pub count : usize,
   pub routers : &'a Mutex<HashSet<String>>
   
 
   
}




impl <'a>App<'a>{
    pub fn new(cleint:&'static  str) -> Self {
        App { client:cleint,count:0,routers: &Mutex::new(HashSet::new())}
    }
    fn register(self,method :Method,path :&str,handler : HandlerFn) {
        let h = handler;
        h(self.client);
    }
    pub fn add(mut self,method :Method,path :&str,handler : HandlerFn) {
            self.register(method, path, handler)
    }

    pub fn get(&mut self,mut path: &str ,handler : HandlerFn) {
        self.count = self.count + 1;
        let mut s = Vec::<String>::new();
        s.push("wef".to_string());
        let mut s = String::from("WEF");
        self.routers.
        self.add(Method::GET,path, handler)
    } 

    pub fn post(&mut self,path: &str ,handler : HandlerFn) {
        self.count = self.count + 1;
        self.add(Method::POST,path, handler)
    } 
   
}