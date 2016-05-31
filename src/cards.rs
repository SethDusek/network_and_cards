extern crate rand;

use self::rand::thread_rng;
use self::rand::Rng;

const DECK_SIZE: usize = 52;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Card {
    Jack,
    King,
    Queen,
    Ace,
    Normal(u8)
}
impl From<u8> for Card {
    fn from(card: u8) -> Card {
        match card {
            1 | 11 => Card::Ace,
            10 => {
                let mut rng = thread_rng();
                let random = rng.gen_range(1, 3);
                if random == 1 { Card::Jack } else if random == 2 { Card::King } else { Card::Queen }
            }
            _ => Card::Normal(card)
        }
    }
}

impl Card {
    pub fn random() -> Card {
        let mut rng = thread_rng();
        let random = rng.gen_range(1, 21);
        Card::from(random)
    }
}

impl Into<u8> for Card {
    fn into(self) -> u8 {
        match self {
            Card::Jack => 10,
            Card::King => 10,
            Card::Queen => 10,
            Card::Ace => 11,
            Card::Normal(val) => val
        }
    }
}
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Suite {
    Spades,
    Hearts,
    Clubs,
    Diamonds
}


pub fn make_deck() -> Vec<(Card, Suite)> {
    let mut deck: Vec<(Card, Suite)> = Vec::with_capacity(DECK_SIZE);
    let mut rng = thread_rng();
    for _ in 0..DECK_SIZE {
        'inner: loop {
            let card = Card::random();
            let suite = match rng.gen_range(1, 5) {
                1 => Suite::Spades,
                2 => Suite::Hearts,
                3 => Suite::Clubs,
                4 => Suite::Diamonds,
                _ => unreachable!()
            };
            if deck.len() > 0 {
                let mut collided = false;                            
                println!("{}", deck.len());
                for card_num in 0..deck.len() {
                    if deck[card_num].0==card && deck[card_num].1==suite { collided = true; break; };
                }
                if collided { continue; }
                else { deck.push((card, suite)); break; }
            }
            else {
                deck.push((card, suite));
                break 'inner;
            }
        }
            
    }
    println!("{}", deck.len());
    deck
}

pub trait Message {
    fn msg(&self) -> String;
}

impl Message for (Card, Suite) {
    fn msg(&self) -> String {
        let card = match self.0 {
            Card::Jack => "Jack".into(),
            Card::Ace => "Ace".into(),
            Card::King => "King".into(),
            Card::Queen => "Queen".into(),
            Card::Normal(num) => num.to_string(),
        };
        let suite = match self.1 {
            Suite::Spades => ",Spades",
            Suite::Hearts => ",Hearts",
            Suite::Clubs => ",Clubs",
            Suite::Diamonds => ",Diamonds"
        };
        card + suite
    }
}

