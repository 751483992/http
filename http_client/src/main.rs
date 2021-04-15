use std::net::{TcpStream, TcpListener};
use std::io::{Write, Read};
use http::Request;
fn main() {
    //http 原始
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let message="http_request".as_bytes();
    stream.write(&message).unwrap();
    // match stream.read(&mut buf){
    //     Ok(_) => {
    //         let req_str = String::from_utf8_lossy(&buf);
    //         println!("{:?} ", req_str);
    //     },
    //     Err(e) => println!("Unable to read stream {} ",e),
    // }

}
