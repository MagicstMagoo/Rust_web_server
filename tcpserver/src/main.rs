use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:1145").unwrap();
    println!("running on 0.0.0.0:1145");

    for stream in listener.incoming(){
        let mut stream = stream.unwrap();
        println!("connection established");
        let mut buffer = [0;1024];

        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
