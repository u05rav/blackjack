use rand::Rng;

#[derive(Copy, Clone)]
#[derive(Debug)]
enum Suit { 
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Copy, Clone)]
#[derive(Debug)]
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

#[derive(Copy, Clone)]
#[derive(Debug)]
struct Card {
    suit: Suit,
    num: CardNum,
}

struct Deck {
    cards: [Card; 52]
}

impl Deck {
    fn new() -> Deck {
        let cards : [Card; 52] = [Card{ suit: Suit::Hearts, num: CardNum::Ace }, Card{ suit: Suit::Hearts, num: CardNum::Two }, Card{ suit: Suit::Hearts, num: CardNum::Three }, Card{ suit: Suit::Hearts, num: CardNum::Four }, Card{ suit: Suit::Hearts, num: CardNum::Five }, Card{ suit: Suit::Hearts, num: CardNum::Six }, Card{ suit: Suit::Hearts, num: CardNum::Seven }, Card{ suit: Suit::Hearts, num: CardNum::Eight }, Card{ suit: Suit::Hearts, num: CardNum::Nine }, Card{ suit: Suit::Hearts, num: CardNum::Ten }, Card{ suit: Suit::Hearts, num: CardNum::Jack }, Card{ suit: Suit::Hearts, num: CardNum::Queen }, Card{ suit: Suit::Hearts, num: CardNum::King }, Card{ suit: Suit::Diamonds, num: CardNum::Ace }, Card{ suit: Suit::Diamonds, num: CardNum::Two }, Card{ suit: Suit::Diamonds, num: CardNum::Three }, Card{ suit: Suit::Diamonds, num: CardNum::Four }, Card{ suit: Suit::Diamonds, num: CardNum::Five }, Card{ suit: Suit::Diamonds, num: CardNum::Six }, Card{ suit: Suit::Diamonds, num: CardNum::Seven }, Card{ suit: Suit::Diamonds, num: CardNum::Eight }, Card{ suit: Suit::Diamonds, num: CardNum::Nine }, Card{ suit: Suit::Diamonds, num: CardNum::Ten }, Card{ suit: Suit::Diamonds, num: CardNum::Jack }, Card{ suit: Suit::Diamonds, num: CardNum::Queen }, Card{ suit: Suit::Diamonds, num: CardNum::King }, Card{ suit: Suit::Spades, num: CardNum::Ace }, Card{ suit: Suit::Spades, num: CardNum::Two }, Card{ suit: Suit::Spades, num: CardNum::Three }, Card{ suit: Suit::Spades, num: CardNum::Four }, Card{ suit: Suit::Spades, num: CardNum::Five }, Card{ suit: Suit::Spades, num: CardNum::Six }, Card{ suit: Suit::Spades, num: CardNum::Seven }, Card{ suit: Suit::Spades, num: CardNum::Eight }, Card{ suit: Suit::Spades, num: CardNum::Nine }, Card{ suit: Suit::Spades, num: CardNum::Ten }, Card{ suit: Suit::Spades, num: CardNum::Jack }, Card{ suit: Suit::Spades, num: CardNum::Queen }, Card{ suit: Suit::Spades, num: CardNum::King }, Card{ suit: Suit::Clubs, num: CardNum::Ace }, Card{ suit: Suit::Clubs, num: CardNum::Two }, Card{ suit: Suit::Clubs, num: CardNum::Three }, Card{ suit: Suit::Clubs, num: CardNum::Four }, Card{ suit: Suit::Clubs, num: CardNum::Five }, Card{ suit: Suit::Clubs, num: CardNum::Six }, Card{ suit: Suit::Clubs, num: CardNum::Seven }, Card{ suit: Suit::Clubs, num: CardNum::Eight }, Card{ suit: Suit::Clubs, num: CardNum::Nine }, Card{ suit: Suit::Clubs, num: CardNum::Ten }, Card{ suit: Suit::Clubs, num: CardNum::Jack }, Card{ suit: Suit::Clubs, num: CardNum::Queen }, Card{ suit: Suit::Clubs, num: CardNum::King }];
        Deck { cards }
    }

    fn shuffle(&mut self) {
        for start in 0..52 {
            let end = rand::thread_rng().gen_range(0..52);

            if start != end {
                let tmp = self.cards[start];
                self.cards[start] = self.cards[end];
                self.cards[end] = tmp;
            }
        }
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();

    for card in deck.cards.into_iter(){
        println!("{:?}", card)
    }
}
