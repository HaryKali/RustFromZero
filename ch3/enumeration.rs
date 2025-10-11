enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

struct PokerCard {
    suit: PokerSuit,
    value: u8
}

fn main() {
    let c1 = PokerCard {
        suit: PokerSuit::Clubs,
        value: 1,
    };
    let c2 = PokerCard {
        suit: PokerSuit::Diamonds,
        value: 12,
    };
}