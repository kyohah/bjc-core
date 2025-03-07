use smallvec::SmallVec;
use std::cmp::Ordering;

use crate::models::{Card, Deck};

#[derive(PartialEq, Debug, Eq, Hash, Clone)]
pub struct Cards {
    pub cards: SmallVec<[Card; 21]>,
}

impl Ord for Cards {
    fn cmp(&self, other: &Self) -> Ordering {
        let min_len = self.cards.len().min(other.cards.len());

        for i in 0..min_len {
            match self.cards[i].cmp(&other.cards[i]) {
                Ordering::Equal => continue,
                non_eq => return non_eq,
            }
        }

        // If all compared cards are equal up to the shortest length, compare by length
        self.cards.len().cmp(&other.cards.len())
    }
}

impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Cards {
    pub fn new() -> Self {
        Cards {
            cards: SmallVec::new(),
        }
    }

    pub(crate) fn from_smallvec(cards: SmallVec<[Card; 21]>) -> Self {
        Cards { cards }
    }

    pub fn new_from_strs(strs: &Vec<&str>) -> Self {
        let mut cards = SmallVec::new();
        for s in strs {
            if let Some(card) = Card::from_str(s) {
                cards.push(card);
            }
        }
        Cards { cards }
    }

    pub fn add_mut(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn add(&self, card: Card) -> Self {
        let mut new_cards = self.clone();
        new_cards.add_mut(card);
        new_cards
    }
}

// Implement FromIterator for Cards using SmallVec's FromIterator
impl FromIterator<Card> for Cards {
    fn from_iter<I: IntoIterator<Item = Card>>(iter: I) -> Self {
        Cards {
            cards: iter.into_iter().collect(), // SmallVecのFromIteratorを利用
        }
    }
}

impl From<Deck> for Cards {
    fn from(deck: Deck) -> Self {
        let mut cards = Cards::new();
        for _ in 0..deck.ace {
            cards.add_mut(Card::Ace);
        }
        for _ in 0..deck.n2 {
            cards.add_mut(Card::N2);
        }
        for _ in 0..deck.n3 {
            cards.add_mut(Card::N3);
        }
        for _ in 0..deck.n4 {
            cards.add_mut(Card::N4);
        }
        for _ in 0..deck.n5 {
            cards.add_mut(Card::N5);
        }
        for _ in 0..deck.n6 {
            cards.add_mut(Card::N6);
        }
        for _ in 0..deck.n7 {
            cards.add_mut(Card::N7);
        }
        for _ in 0..deck.n8 {
            cards.add_mut(Card::N8);
        }
        for _ in 0..deck.n9 {
            cards.add_mut(Card::N9);
        }
        for _ in 0..deck.face {
            cards.add_mut(Card::Face);
        }
        cards
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smallvec::smallvec;
    use std::collections::BTreeSet;
    use std::vec;

    #[test]
    fn test_from_iter_empty() {
        let card_iter = vec![];
        let cards: Cards = card_iter.into_iter().collect();
        assert_eq!(cards.cards.len(), 0);
    }

    #[test]
    fn test_from_iter_single() {
        let card_iter = vec![Card::Ace];
        let cards: Cards = card_iter.into_iter().collect();
        assert_eq!(cards.cards.len(), 1);
        assert_eq!(cards.cards[0], Card::Ace);
    }

    #[test]
    fn test_from_iter_multiple() {
        let card_iter = vec![Card::N2, Card::N3, Card::Face, Card::Ace];
        let cards: Cards = card_iter.into_iter().collect();
        assert_eq!(cards.cards.len(), 4);
        assert_eq!(cards.cards[0], Card::N2);
        assert_eq!(cards.cards[1], Card::N3);
        assert_eq!(cards.cards[2], Card::Face);
        assert_eq!(cards.cards[3], Card::Ace);
    }

    #[test]
    fn test_from_smallvec_iter_multiple() {
        let card_iter: SmallVec<[Card; 4]> = smallvec![Card::N2, Card::N3, Card::Face, Card::Ace];
        let cards: Cards = card_iter.into_iter().collect();
        assert_eq!(cards.cards.len(), 4);
        assert_eq!(cards.cards[0], Card::N2);
        assert_eq!(cards.cards[1], Card::N3);
        assert_eq!(cards.cards[2], Card::Face);
        assert_eq!(cards.cards[3], Card::Ace);
    }

    #[test]
    fn test_from_iter_with_duplicates() {
        let card_iter = vec![Card::N5, Card::N5, Card::Face, Card::Face, Card::Ace];
        let cards: Cards = card_iter.into_iter().collect();
        assert_eq!(cards.cards.len(), 5);
        assert_eq!(cards.cards[0], Card::N5);
        assert_eq!(cards.cards[1], Card::N5);
        assert_eq!(cards.cards[2], Card::Face);
        assert_eq!(cards.cards[3], Card::Face);
        assert_eq!(cards.cards[4], Card::Ace);
    }

    #[test]
    fn card_order() {
        let card1 = Card::Ace;
        let card2 = Card::N2;
        assert!(card1 > card2);
    }

    #[test]
    fn test_cards_ordering() {
        let cards1 = Cards {
            cards: smallvec![Card::Face, Card::N2],
        };
        let cards2 = Cards {
            cards: smallvec![Card::N9, Card::N3],
        };
        let cards3 = Cards {
            cards: smallvec![Card::N9, Card::N2, Card::Ace],
        };
        let cards4 = Cards {
            cards: smallvec![Card::N8, Card::N4],
        };
        let cards5 = Cards {
            cards: smallvec![Card::N8, Card::N3, Card::Ace],
        };
        let cards6 = Cards {
            cards: smallvec![Card::N8, Card::N2, Card::N2, Card::Ace],
        };
        let cards7 = Cards {
            cards: smallvec![Card::N7, Card::N2, Card::Ace, Card::Ace, Card::Ace],
        };

        assert!(cards1 > cards2);
        assert!(cards2 > cards3);
        assert!(cards3 > cards4);
        assert!(cards4 > cards5);
        assert!(cards5 > cards6);
        assert!(cards6 > cards7);

        // Test for equality
        let cards8 = Cards {
            cards: smallvec![Card::N9, Card::N3],
        };
        assert!(cards2 == cards8);
        assert!(cards2 != cards3);
    }

    #[test]
    fn test_cards_btree_set() {
        let mut set = BTreeSet::new();
        let cards1 = Cards {
            cards: smallvec![Card::Face, Card::N2],
        };
        let cards2 = Cards {
            cards: smallvec![Card::N9, Card::N3],
        };
        let cards3 = Cards {
            cards: smallvec![Card::N9, Card::N2, Card::Ace],
        };
        let cards4 = Cards {
            cards: smallvec![Card::N8, Card::N4],
        };
        let cards5 = Cards {
            cards: smallvec![Card::N8, Card::N3, Card::Ace],
        };
        let cards6 = Cards {
            cards: smallvec![Card::N8, Card::N2, Card::N2, Card::Ace],
        };
        let cards7 = Cards {
            cards: smallvec![Card::N7, Card::N2, Card::Ace, Card::Ace, Card::Ace],
        };

        set.insert(cards1.clone());
        set.insert(cards2.clone());
        set.insert(cards3.clone());
        set.insert(cards4.clone());
        set.insert(cards5.clone());
        set.insert(cards6.clone());
        set.insert(cards7.clone());

        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(&cards7));
        assert_eq!(iter.next(), Some(&cards6));
        assert_eq!(iter.next(), Some(&cards5));
        assert_eq!(iter.next(), Some(&cards4));
        assert_eq!(iter.next(), Some(&cards3));
        assert_eq!(iter.next(), Some(&cards2));
        assert_eq!(iter.next(), Some(&cards1));
        assert_eq!(iter.next(), None);
        println!("{:?}", set);
    }

    #[test]
    fn test_cards_btree_set1() {
        let mut set = BTreeSet::new();
        let cards1 = Cards {
            cards: smallvec![Card::Ace],
        };
        let cards2 = Cards {
            cards: smallvec![Card::Ace, Card::Ace],
        };

        let cards3 = Cards {
            cards: smallvec![Card::Ace, Card::Ace, Card::Ace],
        };

        let cards4 = Cards {
            cards: smallvec![Card::Ace, Card::Ace, Card::Ace, Card::Ace],
        };

        let cards5 = Cards {
            cards: smallvec![Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace],
        };

        let cards6 = Cards {
            cards: smallvec![
                Card::Ace,
                Card::Ace,
                Card::Ace,
                Card::Ace,
                Card::Ace,
                Card::Ace,
            ],
        };

        let cards7 = Cards {
            cards: smallvec![
                Card::Ace,
                Card::Ace,
                Card::Ace,
                Card::Ace,
                Card::Ace,
                Card::Ace,
                Card::Ace,
            ],
        };

        set.insert(cards1.clone());
        set.insert(cards2.clone());
        set.insert(cards3.clone());
        set.insert(cards4.clone());
        set.insert(cards5.clone());
        set.insert(cards6.clone());
        set.insert(cards7.clone());

        let mut iter = set.iter();
        assert_eq!(iter.next(), Some(&cards1));
        assert_eq!(iter.next(), Some(&cards2));
        assert_eq!(iter.next(), Some(&cards3));
        assert_eq!(iter.next(), Some(&cards4));
        assert_eq!(iter.next(), Some(&cards5));
        assert_eq!(iter.next(), Some(&cards6));
        assert_eq!(iter.next(), Some(&cards7));

        assert_eq!(iter.next(), None);
        assert_eq!(set.len(), 7);
    }
}
