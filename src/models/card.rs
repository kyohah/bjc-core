use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum Card {
    Ace,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    Face, // T, J, Q, Kとして扱う
}

impl PartialOrd for Card {
    // N2 < N3 < N4 < N5 < N6 < N7 < N8 < N9 < Face < Ace
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        // Card.value() で大小比較
        self.value().cmp(&other.value())
    }
}

impl Card {
    pub(crate) const ALL: [Card; 10] = [
        Card::Ace,
        Card::N2,
        Card::N3,
        Card::N4,
        Card::N5,
        Card::N6,
        Card::N7,
        Card::N8,
        Card::N9,
        Card::Face,
    ];

    pub(crate) fn value(&self) -> usize {
        match self {
            Card::Ace => 11,
            Card::N2 => 2,
            Card::N3 => 3,
            Card::N4 => 4,
            Card::N5 => 5,
            Card::N6 => 6,
            Card::N7 => 7,
            Card::N8 => 8,
            Card::N9 => 9,
            Card::Face => 10,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "A" | "1" => Some(Card::Ace),
            "2" => Some(Card::N2),
            "3" => Some(Card::N3),
            "4" => Some(Card::N4),
            "5" => Some(Card::N5),
            "6" => Some(Card::N6),
            "7" => Some(Card::N7),
            "8" => Some(Card::N8),
            "9" => Some(Card::N9),
            "10" | "Face" | "T" | "J" | "Q" | "K" => Some(Card::Face),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_order() {
        let card1 = Card::Ace;
        let card2 = Card::N2;
        assert!(card1 > card2);
    }
}
