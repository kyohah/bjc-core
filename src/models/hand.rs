use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

use crate::models::{Card, Cards, Deck};
use smallvec::smallvec;

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum HandTotal {
    BlackJack,
    Value(usize),
    Burst,
}

impl HandTotal {
    pub(crate) fn is_blackjack(&self) -> bool {
        matches!(self, HandTotal::BlackJack)
    }

    pub(crate) fn is_burst(&self) -> bool {
        matches!(self, HandTotal::Burst)
    }
}

impl PartialOrd for HandTotal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandTotal {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (HandTotal::BlackJack, HandTotal::BlackJack) => Ordering::Equal,
            (HandTotal::BlackJack, _) => Ordering::Greater,
            (_, HandTotal::BlackJack) => Ordering::Less,
            (HandTotal::Burst, HandTotal::Burst) => Ordering::Equal,
            (HandTotal::Burst, _) => Ordering::Greater,
            (_, HandTotal::Burst) => Ordering::Less,
            (HandTotal::Value(v1), HandTotal::Value(v2)) => v1.cmp(v2),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub enum Hand {
    Soft(HandTotal),
    Hard(HandTotal),
    Pair(Card),
    None,
}

impl Hand {
    pub(crate) fn is_burst(&self) -> bool {
        matches!(self, Hand::Hard(HandTotal::Burst))
    }

    pub(crate) fn is_blackjack(&self) -> bool {
        matches!(self, Hand::Soft(HandTotal::BlackJack))
    }

    // 21かどうか
    pub(crate) fn is_21(&self) -> bool {
        match self {
            Hand::Hard(HandTotal::Value(v)) => *v == 21,
            Hand::Soft(HandTotal::Value(v)) => *v == 21,
            _ => false,
        }
    }

    // 11以下かどうか, ただし、pairは含まない
    pub(crate) fn is_lteq_11(&self) -> bool {
        match self {
            Hand::Hard(HandTotal::Value(v)) => *v <= 11,
            Hand::Soft(HandTotal::Value(v)) => *v <= 11,
            _ => false,
        }
    }

    pub(crate) fn hand_total(&self) -> HandTotal {
        match self {
            Hand::Hard(HandTotal::Value(v)) => HandTotal::Value(*v),
            Hand::Soft(HandTotal::Value(v)) => HandTotal::Value(*v),
            Hand::Soft(HandTotal::BlackJack) => HandTotal::BlackJack,
            Hand::Hard(HandTotal::Burst) => HandTotal::Burst,
            Hand::Pair(c) => {
                if c == &Card::Ace {
                    HandTotal::Value(12)
                } else {
                    HandTotal::Value(c.value() * 2)
                }
            }
            Hand::None => HandTotal::Value(0),
            _ => panic!("Invalid hand"),
        }
    }

    pub(crate) fn is_pair(&self) -> bool {
        matches!(self, Hand::Pair(_))
    }

    pub(crate) fn add_mut(&mut self, card: Card) {
        *self = match self {
            Hand::Hard(HandTotal::Value(v)) => {
                // aceのとき、10以下なら11. 11以上なら1

                let new_value = *v
                    + if card == Card::Ace && *v >= 11 {
                        1
                    } else {
                        card.value()
                    };
                if new_value > 21 {
                    Hand::Hard(HandTotal::Burst)
                } else {
                    Hand::Hard(HandTotal::Value(new_value))
                }
            }
            // 20(A, 9) + A → 21 hard
            // 20(A, 9) + 2 → 12 hard
            // 20(A, 9) + 10 → 20 hard
            // 12(A, 2) + A → 13 soft
            // 12(A, 2) + 2 → 14 soft
            // 12(A, 2) + 10 → 13 hard
            Hand::Soft(HandTotal::Value(v)) => {
                // softなら、すでに、Aが11として使われているので、Aは1として使う
                let add_value: usize = if card == Card::Ace { 1 } else { card.value() };
                let new_value = *v + add_value;
                if new_value > 21 {
                    let new_value = new_value - 10;
                    Hand::Hard(HandTotal::Value(new_value))
                } else {
                    Hand::Soft(HandTotal::Value(new_value))
                }
            }
            Hand::Pair(c) => Cards::from_smallvec(smallvec![*c, *c, card]).into(),
            Hand::None => Cards::from_smallvec(smallvec![card]).into(),
            _ => panic!("へんなもの追加しないで self: {:?} card {:?}", self, card),
        };
    }

    pub fn add(&self, card: Card) -> Self {
        let mut new_hand = self.clone();
        new_hand.add_mut(card);
        new_hand
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // Hard > Soft > Pair > None
        match (self, other) {
            (Hand::Hard(h1), Hand::Hard(h2)) => h1.cmp(h2),
            (Hand::Hard(_), _) => Ordering::Greater,
            (Hand::Soft(h1), Hand::Soft(h2)) => h1.cmp(h2),
            (Hand::Soft(_), Hand::Hard(_)) => Ordering::Less,
            (Hand::Soft(_), _) => Ordering::Greater,
            (Hand::Pair(c1), Hand::Pair(c2)) => c1.cmp(c2),
            (Hand::Pair(_), _) => Ordering::Less,
            (Hand::None, Hand::None) => Ordering::Equal,
            (Hand::None, _) => Ordering::Less,
        }
    }
}

impl From<Cards> for Hand {
    fn from(item: Cards) -> Self {
        let cards_len = item.cards.len();
        if cards_len == 0 {
            Hand::None
        } else if cards_len == 1 {
            let card = item.cards[0];
            if card == Card::Ace {
                Hand::Soft(HandTotal::Value(11))
            } else {
                Hand::Hard(HandTotal::Value(card.value()))
            }
        } else if cards_len == 2 && item.cards[0] == item.cards[1] {
            Hand::Pair(item.cards[0])
        } else {
            let mut total = 0;
            let mut aces: usize = 0;
            for &card in &item.cards {
                total += card.value();
                if card == Card::Ace {
                    aces += 1;
                }
            }
            while total > 21 && aces > 0 {
                total -= 10;
                aces -= 1;
            }
            if total == 21 && cards_len == 2 {
                Hand::Soft(HandTotal::BlackJack)
            } else if aces > 0 {
                Hand::Soft(HandTotal::Value(total))
            } else if total > 21 {
                Hand::Hard(HandTotal::Burst)
            } else {
                Hand::Hard(HandTotal::Value(total))
            }
        }
    }
}

impl From<Deck> for Hand {
    fn from(item: Deck) -> Self {
        let cards: Cards = item.into();
        cards.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;

    #[test]
    fn test_total_value_no_ace() {
        let hand = Hand::from(Cards::new_from_strs(&vec!["2", "3", "T"]));

        assert_eq!(hand, Hand::Hard(HandTotal::Value(15)));
    }

    #[test]
    fn test_total_value_with_one_ace() {
        let cards: Hand = Cards::new_from_strs(&vec!["A", "9"]).into();
        assert_eq!(cards, Hand::Soft(HandTotal::Value(20)));
    }

    #[test]
    fn test_total_value_with_multiple_aces() {
        let cards: Hand = Cards::new_from_strs(&vec!["A", "A", "9"]).into();
        assert_eq!(cards, Hand::Soft(HandTotal::Value(21)));
    }

    #[test]
    fn test_total_value_with_bust_ace() {
        let cards: Hand = Cards::new_from_strs(&vec!["A", "A", "9", "T"]).into();
        assert_eq!(cards, Hand::Hard(HandTotal::Value(21)));
    }

    #[test]
    fn test_total_value_with_ace_adjustment() {
        let cards: Hand = Cards::new_from_strs(&vec!["A", "8", "T"]).into();
        assert_eq!(cards, Hand::Hard(HandTotal::Value(19)));
    }

    #[test]
    fn test_total_value_with_blackjack() {
        let cards: Hand = Cards::new_from_strs(&vec!["A", "T"]).into();
        assert_eq!(cards, Hand::Soft(HandTotal::BlackJack));
        assert!(cards.is_blackjack());
    }

    #[test]
    fn test_total_value_burst() {
        let cards: Hand = Cards::new_from_strs(&vec!["T", "T", "7"]).into();
        assert_eq!(cards, Hand::Hard(HandTotal::Burst));
        assert!(cards.is_burst());
    }

    #[test]
    fn test_to_hand_12() {
        let cards: Hand = Cards::new_from_strs(&vec!["A", "2"]).into();
        assert_eq!(cards, Hand::Soft(HandTotal::Value(13)));
    }

    #[test]
    fn test_eq() {
        assert_eq!(HandTotal::Value(10), HandTotal::Value(10));
        assert_ne!(HandTotal::Value(10), HandTotal::Value(11));
        assert_eq!(HandTotal::BlackJack, HandTotal::BlackJack);
        assert_ne!(HandTotal::BlackJack, HandTotal::Value(21));
    }

    #[test]
    fn test_ord() {
        assert!(HandTotal::Value(10) < HandTotal::Value(11));
        assert!(HandTotal::Value(11) > HandTotal::Value(10));
        assert!(HandTotal::Value(10) <= HandTotal::Value(10));
        assert!(HandTotal::Value(10) >= HandTotal::Value(10));
        assert!(HandTotal::Value(21) < HandTotal::BlackJack);
        assert!(HandTotal::BlackJack > HandTotal::Value(21));
    }

    #[test]
    fn test_hand_add_mut1() {
        let mut hand = Hand::Hard(HandTotal::Value(10));
        hand.add_mut(Card::Ace);
        assert_eq!(hand, Hand::Hard(HandTotal::Value(21)));
    }

    #[test]
    fn test_hand_add_mut2() {
        let mut hand = Hand::Pair(Card::Ace);
        hand.add_mut(Card::Ace);
        assert_eq!(hand, Hand::Soft(HandTotal::Value(13)));
    }

    #[test]
    fn test_hand_add_mut3() {
        let mut hand = Hand::Pair(Card::Ace);
        hand.add_mut(Card::N6);
        assert_eq!(hand, Hand::Soft(HandTotal::Value(18)));
    }

    #[test]
    fn test_hand_add_mut4() {
        let mut hand = Hand::Pair(Card::Ace);
        hand.add_mut(Card::Face);
        assert_eq!(hand, Hand::Hard(HandTotal::Value(12)));
    }

    #[test]
    fn test_cards_to_hand1() {
        let cards: Hand = Cards::new_from_strs(&vec!["A", "2", "3", "T"]).into();
        assert_eq!(cards, Hand::Hard(HandTotal::Value(16)));
    }

    #[test]
    fn test_cards_to_hand2() {
        let cards: Hand = Cards::new_from_strs(&vec!["A", "T"]).into();
        assert_eq!(cards, Hand::Soft(HandTotal::BlackJack));
    }

    #[test]
    fn test_cards_to_hand3() {
        let cards: Hand = Cards::new_from_strs(&vec!["A", "A", "A"]).into();
        assert_eq!(cards, Hand::Soft(HandTotal::Value(13)));
    }
}
