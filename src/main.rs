#[macro_use] extern crate mioco;
extern crate rand;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::FromStr;
use std::net::SocketAddr;
use mioco::tcp::{TcpListener, Shutdown, TcpStream};
use mioco::Evented;
use mioco::EventSourceId;
use mioco::RW;
use std::fmt::Formatter;
use rand::Rng;

mod cards;
mod events;
use events::Event;
use cards::*;

struct Client { 
    stream: TcpStream,
    bufstream: BufReader<TcpStream>,
    started: bool,
    games_won: usize,
    username: String,
    cards: Vec<(cards::Card, cards::Suite)>
}

impl std::fmt::Debug for Client {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "Client {{ started: {}, cards: {:?}, score: {} }}", self.started, self.cards, score(&self.cards))
    }
}

fn is_started(clients: &[Client]) -> bool {
    for client in clients {
        if client.started == false { return false; }
    }
    true
}

fn score(vec: &[(Card, Suite)]) -> u8 {
    let mut aces = Vec::new();
    let mut score = 0u8;
    for card in 0..vec.len() {
        if vec[card].0 == Card::Ace {
            aces.push(vec[card]);
            continue;
        }
        score+=vec[card].0.into();        
    }
    for _ in aces {
        if score+11>21 {
            score+=1;
        }
        else { score+=11;}
    }
    score
}

fn reset(vec: &mut [Client]) {
    for client in vec {
        client.cards = Vec::new();
        client.started = false;
    }
}
fn main() {
    mioco::start(|| {
        let mut rng = rand::thread_rng();
        let addr = SocketAddr::from_str("127.0.0.1:8080").unwrap();
        let listener = TcpListener::bind(&addr).unwrap();
        let mut clients: Vec<Client>  = Vec::new();
        let mut deck = make_deck();
        println!("here?");
        let mut started = false;
        let mut turn = 0;
        loop {
            unsafe {
                listener.select_add(RW::read());
                for client in &clients { client.stream.select_add(RW::read()) }
            }
            let EventSourceId(id) = mioco::select_wait().id();
            println!("{}", id);
            match id {
                0 => {
                    let stream = listener.accept();
                    if let Ok(mut stream) = stream {
                        let _ = writeln!(&mut stream, "USERNAME");
                        let mut bufstream = BufReader::new(stream.try_clone().unwrap());
                        let mut line = String::new();
                        bufstream.read_line(&mut line);                        
                        let splitted: Vec<&str> = line.split(" ").collect();
                        if splitted[0].len() >= 8 && splitted.len()>=2 {
                            if &splitted[0][0..8]=="USERNAME" {
                                clients.push(Client {
                                    stream: stream,
                                    bufstream: bufstream,
                                    started: false,
                                    username: splitted[1].to_owned(),
                                    games_won: 0,
                                    cards: Vec::new()
                                });  
                            }
                        }
                    }   
                }
                n if n>=1 => {
                    let mut line = String::new();
                    if clients[n-1].stream.take_socket_error().is_err() || clients[n-1].bufstream.read_line(&mut line).is_err() {
                        println!("closed!");
                        clients.remove(n-1);
                        continue;
                    };
                    if line.trim_right().len()<4 {
                        writeln!(&mut clients[n-1].stream, "MSG Invalid Command");
                        continue;
                    }
                    let event = Event::from(&line[..]);
                    if event == Event::Start && clients[n-1].started == false {
                        if started { let _ = writeln!(&mut clients[n-1].stream, "MSG Game has already started"); }
                        clients[n-1].started = true;
                        if is_started(&clients) {
                            started = true;
                            for client in &mut clients {
                                client.cards.push(deck.pop().unwrap());
                            }
                        }
                    }
                    else if event == Event::Start && clients[n-1].started {
                        writeln!(&mut clients[n-1].stream, "MSG You have already started!");
                    }
                    else if event == Event::Score {
                        let score = score(&clients[n-1].cards);
                        writeln!(&mut clients[n-1].stream, "SCORE {}", score);
                    }
                    else if event == Event::Cards {
                        let mut cards = String::new();
                        let mut last = false;
                        for card in &clients[n-1].cards {
                            if !last { cards = cards + &card.msg() + " " }
                            else { cards = cards + &card.msg() }
                        }
                        println!("cardstr {}", cards);
                        writeln!(&mut clients[n-1].stream, "CARDS {}", cards);
                    }
                    if (n-1)==turn {
                        if event == Event::Take && deck.len()>0 && started {
                            let card = deck.pop().unwrap();
                            writeln!(&mut clients[n-1].stream, "{}", card.msg());
                            clients[n-1].cards.push(deck.pop().unwrap());
                            if score(&clients[n-1].cards) > 21 {
                                writeln!(&mut clients[n-1].stream, "BUST");
                                println!("BUST!");
                                reset(&mut clients);
                                started = false;
                            }
                            else if score(&clients[n-1].cards) == 21 {
                                let username = clients[n-1].username.clone();
                                println!("{} has won the game", username);
                                for client in &mut clients { writeln!(client.stream, "WINNER {}", username); };
                                clients[n-1].games_won+=1;
                                reset(&mut clients);
                                started = false;
                            }
                            println!("{:?}", clients[n-1].cards);
                        }
                        else if deck.len() == 0 {
                            for client in &mut clients {
                                writeln!(&mut client.stream, "DECK_EMPTY");
                                client.cards = Vec::new();
                                client.started = false;
                            }
                            started = false;
                            deck = make_deck();

                        }
                        else if event == Event::Pass {}
                        if clients.len()>n { turn+=1 } else { turn = 0 }

                    }
                    println!("{:?}", clients);

                }
                _ => unreachable!()
            }
        }

    });
}
