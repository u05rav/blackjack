use rand::Rng;

#[derive(Copy, Clone, Debug)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Copy, Clone, Debug)]
enum CardNum {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug)]
struct Card {
    suit: Suit,
    num: CardNum,
}

struct Deck {
    cards: [Card; 52],
}

impl Deck {
    fn new() -> Deck {
        let cards: [Card; 52] = [
            Card {
                suit: Suit::Hearts,
                num: CardNum::Ace,
            },
            Card {
                suit: Suit::Hearts,
                num: CardNum::Two,
            },
            Card {
                suit: Suit::Hearts,
                num: CardNum::Three,
            },
            Card {
                suit: Suit::Hearts,
                num: CardNum::Four,
            },
            Card {
                suit: Suit::Hearts,
                num: CardNum::Five,
            },
            Card {
                suit: Suit::Hearts,
                num: CardNum::Six,
            },
            Card {
                suit: Suit::Hearts,
                num: CardNum::Seven,
            },
            Card {
                suit: Suit::Hearts,
                num: CardNum::Eight,
            },
            Card {
                suit: Suit::Hearts,
                num: CardNum::Nine,
            },
            Card {
                suit: Suit::Hearts,
                num: CardNum::Ten,
            },
            Card {
                suit: Suit::Hearts,
                num: CardNum::Jack,
            },
            Card {
                suit: Suit::Hearts,
                num: CardNum::Queen,
            },
            Card {
                suit: Suit::Hearts,
                num: CardNum::King,
            },
            Card {
                suit: Suit::Diamonds,
                num: CardNum::Ace,
            },
            Card {
                suit: Suit::Diamonds,
                num: CardNum::Two,
            },
            Card {
                suit: Suit::Diamonds,
                num: CardNum::Three,
            },
            Card {
                suit: Suit::Diamonds,
                num: CardNum::Four,
            },
            Card {
                suit: Suit::Diamonds,
                num: CardNum::Five,
            },
            Card {
                suit: Suit::Diamonds,
                num: CardNum::Six,
            },
            Card {
                suit: Suit::Diamonds,
                num: CardNum::Seven,
            },
            Card {
                suit: Suit::Diamonds,
                num: CardNum::Eight,
            },
            Card {
                suit: Suit::Diamonds,
                num: CardNum::Nine,
            },
            Card {
                suit: Suit::Diamonds,
                num: CardNum::Ten,
            },
            Card {
                suit: Suit::Diamonds,
                num: CardNum::Jack,
            },
            Card {
                suit: Suit::Diamonds,
                num: CardNum::Queen,
            },
            Card {
                suit: Suit::Diamonds,
                num: CardNum::King,
            },
            Card {
                suit: Suit::Spades,
                num: CardNum::Ace,
            },
            Card {
                suit: Suit::Spades,
                num: CardNum::Two,
            },
            Card {
                suit: Suit::Spades,
                num: CardNum::Three,
            },
            Card {
                suit: Suit::Spades,
                num: CardNum::Four,
            },
            Card {
                suit: Suit::Spades,
                num: CardNum::Five,
            },
            Card {
                suit: Suit::Spades,
                num: CardNum::Six,
            },
            Card {
                suit: Suit::Spades,
                num: CardNum::Seven,
            },
            Card {
                suit: Suit::Spades,
                num: CardNum::Eight,
            },
            Card {
                suit: Suit::Spades,
                num: CardNum::Nine,
            },
            Card {
                suit: Suit::Spades,
                num: CardNum::Ten,
            },
            Card {
                suit: Suit::Spades,
                num: CardNum::Jack,
            },
            Card {
                suit: Suit::Spades,
                num: CardNum::Queen,
            },
            Card {
                suit: Suit::Spades,
                num: CardNum::King,
            },
            Card {
                suit: Suit::Clubs,
                num: CardNum::Ace,
            },
            Card {
                suit: Suit::Clubs,
                num: CardNum::Two,
            },
            Card {
                suit: Suit::Clubs,
                num: CardNum::Three,
            },
            Card {
                suit: Suit::Clubs,
                num: CardNum::Four,
            },
            Card {
                suit: Suit::Clubs,
                num: CardNum::Five,
            },
            Card {
                suit: Suit::Clubs,
                num: CardNum::Six,
            },
            Card {
                suit: Suit::Clubs,
                num: CardNum::Seven,
            },
            Card {
                suit: Suit::Clubs,
                num: CardNum::Eight,
            },
            Card {
                suit: Suit::Clubs,
                num: CardNum::Nine,
            },
            Card {
                suit: Suit::Clubs,
                num: CardNum::Ten,
            },
            Card {
                suit: Suit::Clubs,
                num: CardNum::Jack,
            },
            Card {
                suit: Suit::Clubs,
                num: CardNum::Queen,
            },
            Card {
                suit: Suit::Clubs,
                num: CardNum::King,
            },
        ];
        Deck { cards }
    }

    fn shuffle(&mut self) {
        for start in 0..self.cards.len() {
            let end = rand::thread_rng().gen_range(0..52);

            if start != end {
                let tmp = self.cards[start];
                self.cards[start] = self.cards[end];
                self.cards[end] = tmp;
            }
        }
    }
}

struct Shoe {
    cards: Vec<Card>,
}

impl Shoe {
    fn new(num_decks: u32) -> Shoe {
        let mut cards: Vec<Card> = Vec::new();

        let deck = Deck::new();

        for _ in 0..num_decks {
            for card in deck.cards.into_iter() {
                cards.push(card.clone())
            }
        }

        Shoe { cards }
    }

    fn shuffle(&mut self) {
        for start in 0..self.cards.len() {
            let end = rand::thread_rng().gen_range(0..52);

            if start != end {
                let tmp = self.cards[start];
                self.cards[start] = self.cards[end];
                self.cards[end] = tmp;
            }
        }
    }
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Hand {
        let cards: Vec<Card> = Vec::new();
        Hand { cards }
    }
}

fn main() {
    let mut shoe = Shoe::new(6);
    shoe.shuffle();

    let dealer = Hand::new();
    let player = Hand::new();
}
