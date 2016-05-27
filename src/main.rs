#[cfg(not(feature="threaded"))]
#[macro_use] extern crate mioco;
#[cfg(feature="threaded")]
#[cfg(not(feature="threaded"))]
use std::net::TcpListener;
use std::net::SocketAddr;
use std::sync::Arc;
use std::io::BufReader;
use std::io::prelude::*;
use std::thread;
use std::str::FromStr;

mod cards;
mod events;

use events::Event;


#[cfg(feature="threaded")]
fn main() {
    let listener = Arc::new(TcpListener::bind("127.0.0.1:8080").unwrap());
    let mut threads = Vec::new();
    for _ in 0..4 {
        let listener_clone = listener.clone();        
        threads.push(thread::spawn(move || {
            let (mut stream, ip) = listener_clone.accept().unwrap();
            println!("accepted!");
            let bufstream = BufReader::new(stream.try_clone().unwrap());
            writeln!(&mut stream, "PING");
            for line in bufstream.lines() {
                println!("{:?}", line);
            }
        }))
    }
    threads.pop().unwrap().join();

}

#[cfg(not(feature="threaded"))]
fn main() {
    use mioco::tcp::{TcpListener, Shutdown, TcpStream};
    use mioco::Evented;
    use mioco::EventSourceId;
    use mioco::RW;
    struct Client { 
        stream: TcpStream,
        bufstream: BufReader<TcpStream>,
        started: bool,
        cards: Vec<cards::Card>
    };
    mioco::start(|| {
        let addr = SocketAddr::from_str("127.0.0.1:8080").unwrap();
        let listener = TcpListener::bind(&addr).unwrap();
        let mut clients: Vec<Client>  = Vec::new();
        loop {
            unsafe {
                listener.select_add(RW::read());
                for client in clients.iter() { client.stream.select_add(RW::both()) }
            }
            let EventSourceId(id) = mioco::select_wait().id();
            println!("{}", id);
            match id {
                0 => {
                    let stream = listener.accept();
                    if let Ok(mut stream) = stream {
                        let _ = writeln!(&mut stream, "PING");
                        let mut bufstream = BufReader::new(stream.try_clone().unwrap());
                        let mut line = String::new();
                        bufstream.read_line(&mut line);                        
                        if line.len() >= 4 {
                            if &line[0..4]=="PONG" {
                                clients.push(Client {
                                    stream: stream,
                                    bufstream: bufstream,
                                    started: false,
                                    cards: Vec::new()
                                });  
                            }
                        }
                    }   
                }
                n if n>=1 => {
                    let mut line = String::new();
                    clients[n-1].bufstream.read_line(&mut line);
                    println!("{:?}", Event::from(&line[..]));
                    println!("{}", line);
                }
                _ => unreachable!()
            }
        }

    });
}
