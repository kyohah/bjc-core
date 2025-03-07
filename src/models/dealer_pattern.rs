use crate::models::{Card, Deck, HandTotal};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::LazyLock;
mod ace_patterns;
mod face_patterns;
mod n2_patterns;
mod n3_patterns;
mod n4_patterns;
mod n5_patterns;
mod n6_patterns;
mod n7_patterns;
mod n8_patterns;
mod n9_patterns;

// 一覧であり、キャッシュではない
static DEALER_HAND_PATTERNS: LazyLock<DealerHandPatterns> = LazyLock::new(|| DealerHandPatterns {
    ace: ace_patterns::ACE_PATTERNS.clone(),
    n2: n2_patterns::N2_PATTERNS.clone(),
    n3: n3_patterns::N3_PATTERNS.clone(),
    n4: n4_patterns::N4_PATTERNS.clone(),
    n5: n5_patterns::N5_PATTERNS.clone(),
    n6: n6_patterns::N6_PATTERNS.clone(),
    n7: n7_patterns::N7_PATTERNS.clone(),
    n8: n8_patterns::N8_PATTERNS.clone(),
    n9: n9_patterns::N9_PATTERNS.clone(),
    face: face_patterns::FACE_PATTERNS.clone(),
});

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DealerHandDeckMap {
    pub value_17: BTreeMap<Deck, usize>,
    pub value_18: BTreeMap<Deck, usize>,
    pub value_19: BTreeMap<Deck, usize>,
    pub value_20: BTreeMap<Deck, usize>,
    pub value_21: BTreeMap<Deck, usize>,
    pub burst: BTreeMap<Deck, usize>,
    pub blackjack: BTreeMap<Deck, usize>,
}

impl DealerHandDeckMap {
    // fn incr(&mut self, hand_total: &HandTotal, deck: &Deck) {
    //     let counter = match hand_total {
    //         HandTotal::Value(17) => &mut self.value_17,
    //         HandTotal::Value(18) => &mut self.value_18,
    //         HandTotal::Value(19) => &mut self.value_19,
    //         HandTotal::Value(20) => &mut self.value_20,
    //         HandTotal::Value(21) => &mut self.value_21,
    //         HandTotal::Burst => &mut self.burst,
    //         HandTotal::BlackJack => &mut self.blackjack,
    //         _ => panic!("Invalid HandTotal"),
    //     };
    //     let count = counter.entry(deck.clone()).or_insert(0);
    //     *count += 1;
    // }

    pub(crate) fn find(first_rank: Card, sum_number: &HandTotal) -> &'static BTreeMap<Deck, usize> {
        &DEALER_HAND_PATTERNS.get(first_rank, sum_number)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct DealerHandPatterns {
    pub ace: DealerHandDeckMap,
    pub n2: DealerHandDeckMap,
    pub n3: DealerHandDeckMap,
    pub n4: DealerHandDeckMap,
    pub n5: DealerHandDeckMap,
    pub n6: DealerHandDeckMap,
    pub n7: DealerHandDeckMap,
    pub n8: DealerHandDeckMap,
    pub n9: DealerHandDeckMap,
    pub face: DealerHandDeckMap,
}

impl DealerHandPatterns {
    // pub(crate) fn insert(&mut self, card: Card, hand_total: &HandTotal, deck: &Deck) {
    //     let dealer_hand_deck_map = match (card, hand_total) {
    //         (Card::Ace, _) => &mut self.ace,
    //         (Card::N2, _) => &mut self.n2,
    //         (Card::N3, _) => &mut self.n3,
    //         (Card::N4, _) => &mut self.n4,
    //         (Card::N5, _) => &mut self.n5,
    //         (Card::N6, _) => &mut self.n6,
    //         (Card::N7, _) => &mut self.n7,
    //         (Card::N8, _) => &mut self.n8,
    //         (Card::N9, _) => &mut self.n9,
    //         (Card::Face, _) => &mut self.face,
    //     };
    //     dealer_hand_deck_map.incr(hand_total, deck);
    // }

    pub(crate) fn get(&self, card: Card, hand_total: &HandTotal) -> &BTreeMap<Deck, usize> {
        let dealer_hand_deck_map = match (card, hand_total) {
            (Card::Ace, _) => &self.ace,
            (Card::N2, _) => &self.n2,
            (Card::N3, _) => &self.n3,
            (Card::N4, _) => &self.n4,
            (Card::N5, _) => &self.n5,
            (Card::N6, _) => &self.n6,
            (Card::N7, _) => &self.n7,
            (Card::N8, _) => &self.n8,
            (Card::N9, _) => &self.n9,
            (Card::Face, _) => &self.face,
        };
        match hand_total {
            HandTotal::Value(17) => &dealer_hand_deck_map.value_17,
            HandTotal::Value(18) => &dealer_hand_deck_map.value_18,
            HandTotal::Value(19) => &dealer_hand_deck_map.value_19,
            HandTotal::Value(20) => &dealer_hand_deck_map.value_20,
            HandTotal::Value(21) => &dealer_hand_deck_map.value_21,
            HandTotal::Burst => &dealer_hand_deck_map.burst,
            HandTotal::BlackJack => &dealer_hand_deck_map.blackjack,
            _ => panic!("Invalid HandTotal"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_generate_all_patterns_len() {
        let cases = vec![
            (Card::Ace, HandTotal::Value(17), 99),
            (Card::Ace, HandTotal::Value(18), 147),
            (Card::Ace, HandTotal::Value(19), 174),
            (Card::Ace, HandTotal::Value(20), 186),
            (Card::Ace, HandTotal::Value(21), 175),
            (Card::Ace, HandTotal::Burst, 577),
            (Card::Ace, HandTotal::BlackJack, 1),
            (Card::Face, HandTotal::Value(17), 14),
            (Card::Face, HandTotal::Value(18), 15),
            (Card::Face, HandTotal::Value(19), 19),
            (Card::Face, HandTotal::Value(20), 22),
            (Card::Face, HandTotal::Value(21), 22),
            (Card::Face, HandTotal::Burst, 91),
            (Card::Face, HandTotal::BlackJack, 1),
            (Card::N9, HandTotal::Value(17), 21),
            (Card::N9, HandTotal::Value(18), 22),
            (Card::N9, HandTotal::Value(19), 29),
            (Card::N9, HandTotal::Value(20), 33),
            (Card::N9, HandTotal::Value(21), 35),
            (Card::N9, HandTotal::Burst, 137),
            (Card::N8, HandTotal::Value(17), 28),
            (Card::N8, HandTotal::Value(18), 32),
            (Card::N8, HandTotal::Value(19), 36),
            (Card::N8, HandTotal::Value(20), 43),
            (Card::N8, HandTotal::Value(21), 47),
            (Card::N8, HandTotal::Burst, 189),
            (Card::N7, HandTotal::Value(17), 39),
            (Card::N7, HandTotal::Value(18), 43),
            (Card::N7, HandTotal::Value(19), 52),
            (Card::N7, HandTotal::Value(20), 57),
            (Card::N7, HandTotal::Value(21), 64),
            (Card::N7, HandTotal::Burst, 259),
            (Card::N6, HandTotal::Value(17), 51),
            (Card::N6, HandTotal::Value(18), 58),
            (Card::N6, HandTotal::Value(19), 66),
            (Card::N6, HandTotal::Value(20), 77),
            (Card::N6, HandTotal::Value(21), 82),
            (Card::N6, HandTotal::Burst, 345),
            (Card::N5, HandTotal::Value(17), 69),
            (Card::N5, HandTotal::Value(18), 76),
            (Card::N5, HandTotal::Value(19), 91),
            (Card::N5, HandTotal::Value(20), 102),
            (Card::N5, HandTotal::Value(21), 113),
            (Card::N5, HandTotal::Burst, 458),
            (Card::N4, HandTotal::Value(17), 89),
            (Card::N4, HandTotal::Value(18), 104),
            (Card::N4, HandTotal::Value(19), 119),
            (Card::N4, HandTotal::Value(20), 134),
            (Card::N4, HandTotal::Value(21), 145),
            (Card::N4, HandTotal::Burst, 606),
            (Card::N3, HandTotal::Value(17), 118),
            (Card::N3, HandTotal::Value(18), 137),
            (Card::N3, HandTotal::Value(19), 161),
            (Card::N3, HandTotal::Value(20), 178),
            (Card::N3, HandTotal::Value(21), 194),
            (Card::N3, HandTotal::Burst, 795),
            (Card::N2, HandTotal::Value(17), 151),
            (Card::N2, HandTotal::Value(18), 178),
            (Card::N2, HandTotal::Value(19), 206),
            (Card::N2, HandTotal::Value(20), 232),
            (Card::N2, HandTotal::Value(21), 247),
            (Card::N2, HandTotal::Burst, 1023),
        ];

        for (card, sum, len) in cases {
            // 空だったら、&Vec::new で初期化する
            let patterns = DealerHandDeckMap::find(card, &sum);
            // println!("{:?} {:?} {:?}", card, sum, patterns.len());
            assert_eq!(
                patterns.len(),
                len,
                "Failed for card {:?} and sum {:?}",
                card,
                sum
            );
        }
    }
}
