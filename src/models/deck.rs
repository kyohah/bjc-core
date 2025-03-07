use crate::models::{Card, Cards, DealerHandDeckMap, HandTotal};

use num::rational::Ratio;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::collections::BTreeMap;
use std::sync::{Arc, LazyLock, RwLock};
use std::vec;

// dealerのdeckごとの 16,
static DECK_DEALER_PROBS_REPOSITORY: LazyLock<RwLock<BTreeMap<Deck, Arc<DealerHandProb>>>> =
    LazyLock::new(|| RwLock::new(BTreeMap::new()));

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct Deck {
    pub ace: usize,
    pub n2: usize,
    pub n3: usize,
    pub n4: usize,
    pub n5: usize,
    pub n6: usize,
    pub n7: usize,
    pub n8: usize,
    pub n9: usize,
    pub face: usize, // 10, J, Q, K are considered as "face"
}

impl Default for Deck {
    fn default() -> Self {
        Self {
            ace: 4,
            n2: 4,
            n3: 4,
            n4: 4,
            n5: 4,
            n6: 4,
            n7: 4,
            n8: 4,
            n9: 4,
            face: 16,
        }
    }
}

impl Deck {
    pub fn new(num_decks: usize) -> Self {
        Deck {
            ace: 4 * num_decks,
            n2: 4 * num_decks,
            n3: 4 * num_decks,
            n4: 4 * num_decks,
            n5: 4 * num_decks,
            n6: 4 * num_decks,
            n7: 4 * num_decks,
            n8: 4 * num_decks,
            n9: 4 * num_decks,
            face: 16 * num_decks,
        }
    }

    pub fn zero() -> Self {
        Deck {
            ace: 0,
            n2: 0,
            n3: 0,
            n4: 0,
            n5: 0,
            n6: 0,
            n7: 0,
            n8: 0,
            n9: 0,
            face: 0,
        }
    }

    pub fn new_from_cards(cards: &Cards) -> Self {
        let mut card_counts = Deck::zero();

        for card in cards.cards.iter() {
            match card {
                Card::Ace => card_counts.ace += 1,
                Card::N2 => card_counts.n2 += 1,
                Card::N3 => card_counts.n3 += 1,
                Card::N4 => card_counts.n4 += 1,
                Card::N5 => card_counts.n5 += 1,
                Card::N6 => card_counts.n6 += 1,
                Card::N7 => card_counts.n7 += 1,
                Card::N8 => card_counts.n8 += 1,
                Card::N9 => card_counts.n9 += 1,
                Card::Face => card_counts.face += 1,
            }
        }
        card_counts
    }

    pub fn new_from_strs(strs: &Vec<&str>) -> Self {
        let mut card_counts = Deck::zero();

        for s in strs {
            if let Some(card) = Card::from_str(s) {
                match card {
                    Card::Ace => card_counts.ace += 1,
                    Card::N2 => card_counts.n2 += 1,
                    Card::N3 => card_counts.n3 += 1,
                    Card::N4 => card_counts.n4 += 1,
                    Card::N5 => card_counts.n5 += 1,
                    Card::N6 => card_counts.n6 += 1,
                    Card::N7 => card_counts.n7 += 1,
                    Card::N8 => card_counts.n8 += 1,
                    Card::N9 => card_counts.n9 += 1,
                    Card::Face => card_counts.face += 1,
                }
            }
        }
        card_counts
    }

    pub(crate) fn clear_cache() {
        let mut deck_dealer_probs = DECK_DEALER_PROBS_REPOSITORY.write().unwrap();
        deck_dealer_probs.clear();
    }

    pub(crate) fn dealer_probs(&self) -> Arc<DealerHandProb> {
        {
            let deck_dealer_probs = DECK_DEALER_PROBS_REPOSITORY.read().unwrap();
            if let Some(dealer_probs) = deck_dealer_probs.get(&self) {
                return Arc::clone(dealer_probs);
            }
        }

        let dealer_probs = Arc::new(self.calc_dealer_hand_prob());
        let key_clone = self.clone();

        {
            let mut deck_dealer_probs = DECK_DEALER_PROBS_REPOSITORY.write().unwrap();
            deck_dealer_probs.insert(key_clone, Arc::clone(&dealer_probs));
        }

        dealer_probs
    }

    pub(crate) fn calc_dealer_hand_prob(&self) -> DealerHandProb {
        // key: card, value: total_value_prob
        let mut dealer_hand_prob = DealerHandProb::new();

        // key: total_value, value: prob

        let cards = [
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
        for card in cards {
            let mut dealer_total_prob = DealerTotalProb::new();
            let card_values = [
                HandTotal::BlackJack,
                HandTotal::Value(17),
                HandTotal::Value(18),
                HandTotal::Value(19),
                HandTotal::Value(20),
                HandTotal::Value(21),
                HandTotal::Burst,
            ];
            for sum_number in card_values.iter() {
                let patterns = DealerHandDeckMap::find(card, sum_number);
                // let patterns = dealer_pattern.patterns;
                let prob = patterns
                    .iter()
                    .map(|(pattern, count)| {
                        let prob = self.deck_draw_probability(&pattern);
                        prob * *count as u128
                    })
                    .sum::<Ratio<u128>>();

                dealer_total_prob.insert(sum_number, prob);
            }

            dealer_hand_prob.insert(card, dealer_total_prob);
        }
        dealer_hand_prob
    }

    // 1つ以上のカードが残っているカードの種類を返す
    pub(crate) fn remaining_ranks(&self) -> SmallVec<[Card; 10]> {
        let mut ranks = SmallVec::<[Card; 10]>::new();

        if self.ace > 0 {
            ranks.push(Card::Ace);
        }
        if self.n2 > 0 {
            ranks.push(Card::N2);
        }
        if self.n3 > 0 {
            ranks.push(Card::N3);
        }
        if self.n4 > 0 {
            ranks.push(Card::N4);
        }
        if self.n5 > 0 {
            ranks.push(Card::N5);
        }
        if self.n6 > 0 {
            ranks.push(Card::N6);
        }
        if self.n7 > 0 {
            ranks.push(Card::N7);
        }
        if self.n8 > 0 {
            ranks.push(Card::N8);
        }
        if self.n9 > 0 {
            ranks.push(Card::N9);
        }
        if self.face > 0 {
            ranks.push(Card::Face);
        }

        ranks
    }

    pub fn add_mut(&mut self, card: Card) {
        match card {
            Card::Ace => self.ace += 1,
            Card::N2 => self.n2 += 1,
            Card::N3 => self.n3 += 1,
            Card::N4 => self.n4 += 1,
            Card::N5 => self.n5 += 1,
            Card::N6 => self.n6 += 1,
            Card::N7 => self.n7 += 1,
            Card::N8 => self.n8 += 1,
            Card::N9 => self.n9 += 1,
            Card::Face => self.face += 1,
        }
    }

    pub fn remove_mut(&mut self, card: Card) {
        match card {
            Card::Ace => {
                self.ace -= 1;
            }
            Card::N2 => {
                self.n2 -= 1;
            }
            Card::N3 => {
                self.n3 -= 1;
            }
            Card::N4 => {
                self.n4 -= 1;
            }
            Card::N5 => {
                self.n5 -= 1;
            }
            Card::N6 => {
                self.n6 -= 1;
            }
            Card::N7 => {
                self.n7 -= 1;
            }
            Card::N8 => {
                self.n8 -= 1;
            }
            Card::N9 => {
                self.n9 -= 1;
            }
            Card::Face => {
                self.face -= 1;
            }
        }
    }
    pub fn add(&self, card: Card) -> Self {
        let mut new_cards = self.clone();
        new_cards.add_mut(card);
        new_cards
    }

    pub fn remove(&self, card: Card) -> Self {
        let mut new_cards = self.clone();
        new_cards.remove_mut(card);
        new_cards
    }

    pub fn remove_mut_deck(&mut self, deck: &Deck) {
        self.ace -= deck.ace;
        self.n2 -= deck.n2;
        self.n3 -= deck.n3;
        self.n4 -= deck.n4;
        self.n5 -= deck.n5;
        self.n6 -= deck.n6;
        self.n7 -= deck.n7;
        self.n8 -= deck.n8;
        self.n9 -= deck.n9;
        self.face -= deck.face;
    }

    pub fn remove_deck(&self, deck: &Deck) -> Self {
        let mut new_deck = self.clone();
        new_deck.remove_mut_deck(deck);
        new_deck
    }

    pub(crate) fn count(&self, card: Card) -> usize {
        match card {
            Card::Ace => self.ace,
            Card::N2 => self.n2,
            Card::N3 => self.n3,
            Card::N4 => self.n4,
            Card::N5 => self.n5,
            Card::N6 => self.n6,
            Card::N7 => self.n7,
            Card::N8 => self.n8,
            Card::N9 => self.n9,
            Card::Face => self.face,
        }
    }

    pub fn total_cards(&self) -> usize {
        self.ace
            + self.n2
            + self.n3
            + self.n4
            + self.n5
            + self.n6
            + self.n7
            + self.n8
            + self.n9
            + self.face
    }

    pub(crate) fn draw_probability(&self, target: Card) -> Ratio<usize> {
        let target_count = self.count(target);
        let total_cards = self.total_cards();

        Ratio::new(target_count, total_cards)
    }

    pub(crate) fn deck_draw_probability(&self, draw_deck: &Deck) -> Ratio<u128> {
        let mut total_cards_count = self.total_cards() as u128;
        let self_array = self.to_array();
        let mut return_prob = Ratio::new(1, 1);

        for (i, &draw_count) in draw_deck.to_array().iter().enumerate() {
            if draw_count == 0 {
                continue;
            }

            if self_array[i] < draw_count {
                return Ratio::new(0, 1); // Not enough cards, return zero probability
            }

            let available_count = self_array[i] as u128;

            // Directly calculate the product of probabilities for each card draw

            ((available_count - draw_count as u128 + 1)..=available_count).for_each(|c| {
                let prob = Ratio::new(c, total_cards_count);
                total_cards_count -= 1;
                return_prob *= prob;
            })
        }
        return_prob
        // Multiply all the individual probabilities together
    }

    pub(crate) fn to_array(&self) -> [usize; 10] {
        [
            self.ace, self.n2, self.n3, self.n4, self.n5, self.n6, self.n7, self.n8, self.n9,
            self.face,
        ]
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DealerTotalProb {
    s17: Ratio<u128>,
    s18: Ratio<u128>,
    s19: Ratio<u128>,
    s20: Ratio<u128>,
    s21: Ratio<u128>,
    black_jack: Ratio<u128>,
    bust: Ratio<u128>,
}

impl DealerTotalProb {
    pub(crate) fn new() -> Self {
        DealerTotalProb {
            s17: Ratio::new(0, 1),
            s18: Ratio::new(0, 1),
            s19: Ratio::new(0, 1),
            s20: Ratio::new(0, 1),
            s21: Ratio::new(0, 1),
            black_jack: Ratio::new(0, 1),
            bust: Ratio::new(0, 1),
        }
    }

    pub(crate) fn insert(&mut self, total: &HandTotal, prob: Ratio<u128>) {
        match total {
            HandTotal::Value(17) => self.s17 = prob,
            HandTotal::Value(18) => self.s18 = prob,
            HandTotal::Value(19) => self.s19 = prob,
            HandTotal::Value(20) => self.s20 = prob,
            HandTotal::Value(21) => self.s21 = prob,
            HandTotal::BlackJack => self.black_jack = prob,
            HandTotal::Burst => self.bust = prob,
            _ => panic!("Invalid total"),
        }
    }

    pub(crate) fn iter(&self) -> vec::IntoIter<(&HandTotal, &Ratio<u128>)> {
        vec![
            (&HandTotal::Value(17), &self.s17),
            (&HandTotal::Value(18), &self.s18),
            (&HandTotal::Value(19), &self.s19),
            (&HandTotal::Value(20), &self.s20),
            (&HandTotal::Value(21), &self.s21),
            (&HandTotal::BlackJack, &self.black_jack),
            (&HandTotal::Burst, &self.bust),
        ]
        .into_iter()
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DealerHandProb {
    ace: DealerTotalProb,
    n2: DealerTotalProb,
    n3: DealerTotalProb,
    n4: DealerTotalProb,
    n5: DealerTotalProb,
    n6: DealerTotalProb,
    n7: DealerTotalProb,
    n8: DealerTotalProb,
    n9: DealerTotalProb,
    face: DealerTotalProb,
}

impl DealerHandProb {
    pub(crate) fn new() -> Self {
        DealerHandProb {
            ace: DealerTotalProb::new(),
            n2: DealerTotalProb::new(),
            n3: DealerTotalProb::new(),
            n4: DealerTotalProb::new(),
            n5: DealerTotalProb::new(),
            n6: DealerTotalProb::new(),
            n7: DealerTotalProb::new(),
            n8: DealerTotalProb::new(),
            n9: DealerTotalProb::new(),
            face: DealerTotalProb::new(),
        }
    }

    pub(crate) fn insert(&mut self, card: Card, dealer_total_prob: DealerTotalProb) {
        match card {
            Card::Ace => self.ace = dealer_total_prob,
            Card::N2 => self.n2 = dealer_total_prob,
            Card::N3 => self.n3 = dealer_total_prob,
            Card::N4 => self.n4 = dealer_total_prob,
            Card::N5 => self.n5 = dealer_total_prob,
            Card::N6 => self.n6 = dealer_total_prob,
            Card::N7 => self.n7 = dealer_total_prob,
            Card::N8 => self.n8 = dealer_total_prob,
            Card::N9 => self.n9 = dealer_total_prob,
            Card::Face => self.face = dealer_total_prob,
        }
    }

    pub(crate) fn get(&self, card: Card) -> &DealerTotalProb {
        match card {
            Card::Ace => &self.ace,
            Card::N2 => &self.n2,
            Card::N3 => &self.n3,
            Card::N4 => &self.n4,
            Card::N5 => &self.n5,
            Card::N6 => &self.n6,
            Card::N7 => &self.n7,
            Card::N8 => &self.n8,
            Card::N9 => &self.n9,
            Card::Face => &self.face,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_rational::Ratio;
    use std::vec;

    #[test]
    fn test_draw_probability() {
        let deck_cards = Deck::new(1); // 1デッキ
        let ace_probability = deck_cards.draw_probability(Card::Ace);
        let expected_probability = Ratio::new(4, 52); // 1デッキには4枚のエースがある
        assert_eq!(ace_probability, expected_probability);
    }

    #[test]
    fn test_draw_probability1() {
        let deck_cards = Deck::new_from_strs(&vec!["2", "3", "4", "A"]);
        let ace_probability = deck_cards.draw_probability(Card::N5);
        let expected_probability = Ratio::new(0, 1); // 1デッキには4枚のエースがある
        assert_eq!(ace_probability, expected_probability);
    }

    #[test]
    fn test_card_to_vec() {
        let deck = Deck::new_from_strs(&vec!["A", "2", "3", "T"]);
        let cards: Cards = deck.into();
        assert_eq!(cards.cards.contains(&Card::Ace), true);
    }

    #[test]
    fn test_deck_count() {
        let deck_cards = Deck::new(1); // 1デッキ
        let count = deck_cards.count(Card::Ace);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_cards_draw_probability() {
        let deck_cards = Deck::new(1); // 1デッキ
        let cards = Deck::new_from_strs(&vec!["A"]);
        let prob = deck_cards.deck_draw_probability(&cards);
        let expected_prob = Ratio::new(4, 52);
        assert_eq!(prob, expected_prob);
    }

    #[test]
    fn test_cards_draw_probability1() {
        let deck_cards = Deck::new(1); // 1デッキ
        let cards = Deck::new_from_strs(&vec!["A", "2", "3", "4"]);
        let prob = deck_cards.deck_draw_probability(&cards);
        let expected_prob =
            Ratio::new(4, 52) * Ratio::new(4, 51) * Ratio::new(4, 50) * Ratio::new(4, 49);
        assert_eq!(prob, expected_prob);
    }

    #[test]
    fn test_cards_draw_probability2() {
        let deck_cards = Deck::new(1); // 1デッキ
        let cards = Deck::new_from_strs(&vec!["A", "2", "3", "T"]);
        let prob = deck_cards.deck_draw_probability(&cards);
        let expected_prob =
            Ratio::new(4, 52) * Ratio::new(4, 51) * Ratio::new(4, 50) * Ratio::new(16, 49);
        assert_eq!(prob, expected_prob);
    }

    #[test]
    fn test_cards_draw_probability3() {
        let deck_cards = Deck::new(1); // 1デッキ
        let cards = Deck::new_from_strs(&vec!["A", "A", "A", "A", "A"]);
        let prob = deck_cards.deck_draw_probability(&cards);
        let expected_prob = Ratio::new(0, 1);
        assert_eq!(prob, expected_prob);
    }

    #[test]
    fn test_cards_draw_probability4() {
        let deck_cards = Deck::new(1); // 1デッキ
        let cards = Deck::new_from_strs(&vec!["A", "A"]);
        let prob = deck_cards.deck_draw_probability(&cards);
        let expected_prob = Ratio::new(4, 52) * Ratio::new(3, 51);
        assert_eq!(prob, expected_prob);
    }

    #[test]
    fn test_cards_draw_probability5() {
        let deck_cards = Deck::new(6);
        let cards = Deck::new_from_strs(&vec!["A", "A", "A", "A", "A", "A"]);
        let prob = deck_cards.deck_draw_probability(&cards);
        let expected_prob = Ratio::new(4 * 6, 52 * 6)
            * Ratio::new(4 * 6 - 1, 52 * 6 - 1)
            * Ratio::new(4 * 6 - 2, 52 * 6 - 2)
            * Ratio::new(4 * 6 - 3, 52 * 6 - 3)
            * Ratio::new(4 * 6 - 4, 52 * 6 - 4)
            * Ratio::new(4 * 6 - 5, 52 * 6 - 5);
        assert_eq!(prob, expected_prob);
    }

    #[test]
    fn test_generate_prob2() {
        let deck_cards = Deck::new(1); // 1デッキ
        let prob = deck_cards.dealer_probs();

        // prob.iter().for_each(|(card, total_value_prob)| {
        //     total_value_prob.iter().for_each(|(total_value, prob)| {
        //         println!("{:?} {:?} {:?}", card, total_value, prob);
        //     });
        // });

        // 合計が1.0になるか
        for card in [
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
        ] {
            let total_prob = prob.get(card);

            let total = total_prob.black_jack
                + total_prob.s17
                + total_prob.s18
                + total_prob.s19
                + total_prob.s20
                + total_prob.s21
                + total_prob.bust;

            assert_eq!(total, Ratio::new(1, 1));
        }

        // assert_eq!(
        //     prob.get(&Card::Ace).unwrap().get(&17).unwrap(),
        //     &Ratio::new(85180877, 675354750)
        // );
    }

    #[test]
    fn test_generate_prob3() {
        let deck_cards = Deck::new(6); // 1デッキ
        let prob = deck_cards.dealer_probs();

        // 合計が1.0になるか
        for card in [
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
        ] {
            let total_prob = prob.get(card);
            println!("{:?}", total_prob);

            // let total = total_prob.black_jack
            //     + total_prob.s17
            //     + total_prob.s18
            //     + total_prob.s19
            //     + total_prob.s20
            //     + total_prob.s21
            //     + total_prob.bust;
            // assert_eq!(total, Ratio::new(1, 1));

            // assert_abs_diff_eq!(total_prob, 1.0, epsilon = 1e-9);
        }

        // assert_eq!(
        //     prob.get(&Card::Ace).unwrap().get(&17).unwrap(),
        //     &Ratio::new(85180877, 675354750)
        // );
    }

    #[test]
    fn test_deck_eq1() {
        let cards = vec!["A", "2", "3", "4", "2", "2", "A"];
        let deck1 = Deck::new_from_strs(&cards);
        let deck2 = Deck::new_from_cards(&&Cards::new_from_strs(&cards));
        assert_eq!(deck1, deck2);
    }
}
