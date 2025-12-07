use std::{fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};
use postgres::{Client, NoTls};
fn main() {

    let mut db_connection = Client::connect("host=localhost user=restox_user password=090770 dbname=restox_db", NoTls).unwrap();
    let tables = db_connection.query("SELECT * FROM stores;", &[]).unwrap();
    for table in tables {
        let id : i32 = table.get(0);
        let name: &str = table.get(1);
        println!("{} {}", id ,name);
    }

    //start_server("7878");
}



fn start_server(port:&str){
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).unwrap();
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream);
        println!("Connection established!");
    }
}

fn handle_connection(mut stream:TcpStream){
    let buff_reader = BufReader::new(&stream);
    let http_request:Vec<_> = buff_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {http_request:#?}");
    let status_line = "HTTP/1.1 200 OK";
    // let contents = fs::read_to_string("hello.html").unwrap();
    let contents = r#"{"message":"hello"}"#;
    let length = contents.len();

    let respose = format!("{status_line}\r\nContent-Type: application/json\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(respose.as_bytes()).unwrap();
}

