use std::net::{TcpListener,TcpStream};
use crate::{Result,KvsEngine,Request,Response};
//use serde::Deserialize;
use std::io::BufReader;
use std::fmt;
pub enum EngineType {
    KvStore,
    SledKvStore,
}

//for to_string() can be used on enum EngineType when combine the current dir in kvs_server.rs
impl fmt::Display for EngineType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EngineType::KvStore => write!(f,"kvs"),
            EngineType::SledKvStore => write!(f,"sled"),
        }
    }
}


pub struct KvServer <E> 
where 
E: KvsEngine, // KvStore & SledKvStore
{
    engine: E,
}

impl <E: KvsEngine> KvServer<E> {
    // construct
    pub fn new(engine: E) -> Self {
        KvServer { engine }
    }

    //serve and listen at addr
    //循环处理每一个stream
    pub fn serve(&mut self, addr: String) -> Result<()> {
        let listener = TcpListener::bind(addr)?;
        for stream in listener.incoming() { 
            let stream = stream?;
            self.handle_connection(stream)?;
        }
        Ok(())
    }
    // deserialize the stream to data gram strcut
    // call from struct
    fn handle_connection(&mut self, mut stream: TcpStream) -> Result<()> {
         //序列化request
         //let a = Request::deserialize(&mut serde_json::Deserializer::new(BufReader::new(&mut stream)))?;
         //@proticol.md::Response
         let request:Request = serde_json::from_reader(BufReader::new(&mut stream))?;
         let response;
         match request {
            Request::GET(key) => {
                match self.engine.get(key) {
                    Ok(value) => response = Response::Ok(value),
                    Err(err) => response = Response::Err(err.to_string()),
                }
            }
            Request::SET(key, val) => {
                match self.engine.set(key, val) {
                    Ok(()) => response = Response::Ok(None),
                    Err(err) => response = Response::Err(err.to_string()),
                }
            }
            Request::RM(key) => {
                match self.engine.remove(key) {
                    Ok(()) => response = Response::Ok(None),
                    Err(err) => response = Response::Err(err.to_string()),
                }
            }
         }

        serde_json::to_writer(stream, &response)?;
        
        Ok(())
    }
}