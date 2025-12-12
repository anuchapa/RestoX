use std::{fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

use crate::{controllers::Controller};



pub struct SimpleServer<'a>{
    port:  &'a str,
    pub controllers: Vec<Box<dyn Controller>>,
}

impl<'a> SimpleServer<'a> {
    pub fn new(port:&'a str) -> SimpleServer<'a>{
        SimpleServer { port, controllers: Vec::new() }
    }

    pub fn start_server(&mut self){
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port));
        let listener = match listener {
            Ok(l) => {
                println!("Server is runnning on port {}",self.port);
                l
            },
            Err(e) => {
                panic!("Failed to bind {}",e);
            }
        };
        
        for stream in listener.incoming(){
            let mut stream = stream.unwrap();
            println!("Connection established!");
            let request = self.read_request(&stream);
            let part:Vec<&str> = request[0].split_whitespace().collect();
            let (method,uri,version )= (part[0],part[1],part[2]);
            let response = match self.active_controller(uri).unwrap(){
                res=>{
                    res
                },
            };
            self.send_response(&mut stream, response);
            
        }
        
    }

    fn active_controller(&mut self,uri : &str)->Option<String>{
        for controller in &mut self.controllers {
            if let Some(res)=controller.call(uri){
                return Some(res);
            }
        }
        None
    }

    fn read_request(&self,stream:&TcpStream) -> Vec<String> {
        let buff_reader = BufReader::new(stream);
        let http_request:Vec<_> = buff_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        http_request
    }

    fn send_response(&self,stream:&mut TcpStream,json_response:String){
        let status_line = "HTTP/1.1 200 OK";
        let length = json_response.len();
        let respone_format = format!("{status_line}\r\nContent-Type: application/json\r\nContent-Length: {length}\r\n\r\n{json_response}");
        stream.write_all(respone_format.as_bytes()).unwrap();
    }
}




// pub fn handle_connection(mut stream:TcpStream){
//     let buff_reader = BufReader::new(&stream);
//     let http_request:Vec<_> = buff_reader
//         .lines()
//         .map(|result| result.unwrap())
//         .take_while(|line| !line.is_empty())
//         .collect();
//     println!("Request: {http_request:#?}");
//     let status_line = "HTTP/1.1 200 OK";
//     // let contents = fs::read_to_string("hello.html").unwrap();
//     let contents = r#"{"message":"hello"}"#;
//     let length = contents.len();

//     let respose = format!("{status_line}\r\nContent-Type: application/json\r\nContent-Length: {length}\r\n\r\n{contents}");
//     stream.write_all(respose.as_bytes()).unwrap();
// }