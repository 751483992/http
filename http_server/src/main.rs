use std::net::{TcpListener,TcpStream};
use std::thread;
use std::io::{Read, Write};
use http::{Response, StatusCode};
use std::io::prelude::*;
fn handle_read(mut stream:&TcpStream){
    let mut buf =   [0u8;8];
    match stream.read(&mut buf){
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("{:?} ", req_str);
        },
        Err(e) => println!("Unable to read steam {} ",e),
    }
}
fn handle_read2(mut stream:TcpStream){
    let mut buffer =[0;1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));


}

fn handle_write(mut stream:TcpStream){
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    match stream.write(response){
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response {} ",e),
    }
}

fn handle_client(stream:TcpStream){
    handle_read(&stream);
    handle_write(stream);
}

fn main() {

    // let listner = TcpListener::bind("127.0.0.1:8080").unwrap();
    // println!("Listnening for connection on port is {}",8080);
    //
    // for stream in listner.incoming(){
    //     match stream {
    //         Ok(stream) => {
    //             thread::spawn(||{
    //                 handle_client(stream)
    //             });
    //         }
    //         Err(e) => {
    //             println!("Unable to connect {}",e);
    //         }
    //     }
    // }

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");

        handle_read2(stream);
    }

    // //use http crates
    // let response  = Response::builder().status(StatusCode::MOVED_PERMANENTLY).header("Location","https://www.rust-lang.org/install.html")
    //     .body(()).unwrap();
}