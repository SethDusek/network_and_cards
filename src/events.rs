#[derive(Debug, PartialEq)]
pub enum Event {
    Pong,
    Start,
    Take,
    Pass,
    Score,
    Cards,
    None
}

impl <'a> From<&'a str> for Event {
    fn from(t: &'a str) -> Event {
        assert!(t.len()>=4); //4 is a requirement as even the shortest Event is 4 letters
        match &t.to_lowercase()[..] {
            event if &event[..4]=="pong" => Event::Pong,
            event if &event[..5]=="start" => Event::Start,
            event if &event[..4]=="take" => Event::Take,
            event if &event[..4]=="pass" => Event::Pass,
            event if &event[..5]=="score" => Event::Score,
            event if &event[..5]=="cards" => Event::Cards,
            _ => Event::None
        }
    }
}

