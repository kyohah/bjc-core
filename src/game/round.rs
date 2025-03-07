use crate::models::{Card, Cards, Deck, Hand, Rule};
use num_rational::Ratio;
use serde::{Deserialize, Serialize};

use num::BigInt;
use rayon::prelude::*;
use smallvec::smallvec;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

#[cfg(feature = "onnx")]
use ndarray::{Array2, ArrayBase, Dim, OwnedRepr};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BlackJackGame {
    pub rule: Arc<Rule>,
    pub dealer_card: Card,
    pub player_hand: Hand,
    pub deck_cards: Deck,
    pub player_card_count: usize,
}

use std::{
    collections::BTreeMap,
    sync::{LazyLock, RwLock},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionEV {
    pub stand: Ratio<BigInt>,
    pub hit: Option<Ratio<BigInt>>,
    pub double: Option<Ratio<BigInt>>,
    pub split: Option<Ratio<BigInt>>,
}

impl ActionEV {
    pub fn new(stand: Ratio<BigInt>) -> Self {
        ActionEV {
            stand,
            hit: None,
            double: None,
            split: None,
        }
    }

    fn max_ev(&self) -> Ratio<BigInt> {
        let mut max_ev = self.stand.clone();

        for ev in [self.hit.clone(), self.double.clone(), self.split.clone()]
            .iter()
            .flatten()
        {
            if *ev > max_ev {
                max_ev = ev.clone();
            }
        }

        max_ev
    }
}

pub static BLACK_JACK_GAME_STAND_REPOSITORY: LazyLock<
    RwLock<BTreeMap<BlackJackGame, Ratio<BigInt>>>,
> = LazyLock::new(|| RwLock::new(BTreeMap::new()));
pub static BLACK_JACK_GAME_HIT_REPOSITORY: LazyLock<
    RwLock<BTreeMap<BlackJackGame, Ratio<BigInt>>>,
> = LazyLock::new(|| RwLock::new(BTreeMap::new()));

impl BlackJackGame {
    pub fn new(
        rule: Rule,
        dealer_card: Card,
        player_hand: Hand,
        deck_cards: Deck,
        player_card_count: usize,
    ) -> Self {
        BlackJackGame {
            rule: Arc::new(rule),
            dealer_card,
            player_hand: player_hand.clone(),
            deck_cards: deck_cards.clone(),
            player_card_count,
        }
    }

    pub fn clear_cache() {
        if let Ok(mut map) = BLACK_JACK_GAME_STAND_REPOSITORY.write() {
            map.clear();
        }

        if let Ok(mut map) = BLACK_JACK_GAME_HIT_REPOSITORY.write() {
            map.clear();
        }

        Deck::clear_cache();
    }

    pub fn insert_stand_ev(&self, ev: &Ratio<BigInt>) {
        let game_clone = self.clone();
        let ev_clone = ev.clone();

        if let Ok(mut map) = BLACK_JACK_GAME_STAND_REPOSITORY.write() {
            map.insert(game_clone, ev_clone);
        } else {
            eprintln!("Failed to acquire write lock for BLACK_JACK_GAME_STAND_REPOSITORY");
        }
    }

    pub fn insert_hit_ev(&self, ev: &Ratio<BigInt>) {
        let game_clone = self.clone();
        let ev_clone = ev.clone();

        if let Ok(mut map) = BLACK_JACK_GAME_HIT_REPOSITORY.write() {
            map.insert(game_clone, ev_clone);
        } else {
            eprintln!("Failed to acquire write lock for BLACK_JACK_GAME_HIT_REPOSITORY");
        }
    }

    pub fn get_stand_ev(
        &self,
        cancellation_token: &CancellationToken,
    ) -> Result<Ratio<BigInt>, ()> {
        if self.player_hand.is_lteq_11() || self.player_hand.is_burst() {
            return Ok(Ratio::from_integer(BigInt::from(-1)));
        }

        if let Ok(map) = BLACK_JACK_GAME_STAND_REPOSITORY.read() {
            if let Some(ev) = map.get(self) {
                return Ok(ev.clone());
            }
        }

        let ev = self.stand_ev(cancellation_token)?;
        self.insert_stand_ev(&ev);
        Ok(ev)
    }

    pub fn get_hit_ev(
        &self,
        cancellation_token: &CancellationToken,
    ) -> Result<Option<Ratio<BigInt>>, ()> {
        if self.player_hand.is_21()
            || self.player_hand.is_blackjack()
            || self.player_hand.is_burst()
        {
            return Ok(None);
        }

        if let Ok(map) = BLACK_JACK_GAME_HIT_REPOSITORY.read() {
            if let Some(ev) = map.get(self) {
                return Ok(Some(ev.clone()));
            }
        }

        let ev = self.hit_ev(cancellation_token)?;
        self.insert_hit_ev(&ev);
        Ok(Some(ev))
    }

    pub fn action_ev(&self, cancellation_token: &CancellationToken) -> Result<ActionEV, ()> {
        if cancellation_token.is_cancelled() {
            return Err(()); // キャンセルされている場合は即座にエラーを返す
        }

        // stand, hit, double, splitのうち、1つでもErrが返ってきた場合は、その時点でErrを返す
        // それ以外の場合は、ActionEVを返す
        let stand = self.get_stand_ev(cancellation_token)?;
        let hit = self.get_hit_ev(cancellation_token)?;
        let double = self.double_ev(cancellation_token)?;
        let split = self.split_ev(cancellation_token)?;

        Ok(ActionEV {
            stand,
            hit,
            double,
            split,
        })
    }

    pub fn hit_or_stand_ev(
        &self,
        cancellation_token: &CancellationToken,
    ) -> Result<Ratio<BigInt>, ()> {
        let stand_ev = self.get_stand_ev(cancellation_token)?;

        if let Some(hit_ev) = self.get_hit_ev(cancellation_token)? {
            if hit_ev > stand_ev {
                return Ok(hit_ev);
            }
        }
        Ok(stand_ev)
    }

    pub fn ev(&self) -> Ratio<BigInt> {
        let cancellation_token = CancellationToken::new();
        self.ev_with_cancellation_token(&cancellation_token)
            .unwrap()
    }

    pub fn ev_with_cancellation_token(
        &self,
        cancellation_token: &CancellationToken,
    ) -> Result<Ratio<BigInt>, ()> {
        let action_ev = self.action_ev(cancellation_token)?;
        Ok(action_ev.max_ev())
    }

    pub fn stand_ev(&self, cancellation_token: &CancellationToken) -> Result<Ratio<BigInt>, ()> {
        if self.player_hand.is_burst() {
            return Ok(Ratio::from_integer(BigInt::from(-1)));
        }

        if self.rule.six_card_charlie && self.player_card_count >= 6 {
            return Ok(Ratio::from_integer(BigInt::from(1)));
        }

        let user_total_value = self.player_hand.hand_total();

        let mut black_jack_prob = Ratio::from_integer(BigInt::from(0));
        let mut win_prob = Ratio::from_integer(BigInt::from(0));
        let mut lose_prob = Ratio::from_integer(BigInt::from(0));

        let binding = self.deck_cards.dealer_probs();

        let probs = binding.get(self.dealer_card);

        for (dealer_total_value, ratio) in probs.iter() {
            if cancellation_token.is_cancelled() {
                return Err(());
            }

            let bigint_ratio =
                Ratio::new(BigInt::from(*ratio.numer()), BigInt::from(*ratio.denom()));
            if self.player_hand.is_blackjack() {
                if !dealer_total_value.is_blackjack() {
                    black_jack_prob += bigint_ratio
                }
                // playerがブラックジャックでかつ、dealerがブラックジャックの場合は、pushのため、何もしない
            } else if dealer_total_value.is_blackjack() {
                lose_prob += bigint_ratio;
            } else if dealer_total_value.is_burst() {
                win_prob += bigint_ratio;
            } else if dealer_total_value < &user_total_value {
                win_prob += bigint_ratio;
            } else if dealer_total_value > &user_total_value {
                lose_prob += bigint_ratio;
            }
        }

        Ok(Ratio::from_float(1.5).unwrap() * black_jack_prob + win_prob - lose_prob)
    }

    pub fn hit_ev(&self, cancellation_token: &CancellationToken) -> Result<Ratio<BigInt>, ()> {
        if cancellation_token.is_cancelled() {
            return Err(());
        }

        self.deck_cards
            .remaining_ranks()
            .par_iter() // 並列イテレーションに変更
            .map(|&rank| {
                if cancellation_token.is_cancelled() {
                    return Err(());
                }

                let next_deck_cards = self.deck_cards.remove(rank);

                let round = BlackJackGame {
                    rule: Arc::clone(&self.rule),
                    dealer_card: self.dealer_card,
                    player_hand: self.player_hand.add(rank),
                    deck_cards: next_deck_cards,
                    player_card_count: self.player_card_count + 1,
                };

                let draw_prob = self.deck_cards.draw_probability(rank);
                let hit_or_stand_ev = round.hit_or_stand_ev(cancellation_token)?;

                Ok(hit_or_stand_ev
                    * Ratio::new(
                        BigInt::from(*draw_prob.numer()),
                        BigInt::from(*draw_prob.denom()),
                    ))
            })
            .collect::<Result<Vec<Ratio<BigInt>>, ()>>()
            .map(|evs| evs.iter().sum())
    }

    pub fn double_ev(
        &self,
        cancellation_token: &CancellationToken,
    ) -> Result<Option<Ratio<BigInt>>, ()> {
        if self.player_hand.is_21()
            || self.player_hand.is_blackjack()
            || self.player_hand.is_burst()
        {
            return Ok(None);
        }

        Ok(Some(
            self.deck_cards
                .remaining_ranks()
                .par_iter()
                .map(|&rank| {
                    if cancellation_token.is_cancelled() {
                        return Err(());
                    }

                    let next_deck_cards = self.deck_cards.remove(rank);

                    let round = BlackJackGame {
                        rule: Arc::clone(&self.rule),
                        dealer_card: self.dealer_card,
                        player_hand: self.player_hand.add(rank),
                        deck_cards: next_deck_cards,
                        player_card_count: self.player_card_count + 1,
                    };
                    let draw_prob = self.deck_cards.draw_probability(rank);

                    // 合計が9、10、11のツーカードハンドでのフリーダブル
                    let ev = round.stand_ev(cancellation_token)?
                        * Ratio::new(
                            BigInt::from(*draw_prob.numer()),
                            BigInt::from(*draw_prob.denom()),
                        )
                        * BigInt::from(2);

                    Ok(ev)
                })
                .collect::<Result<Vec<Ratio<BigInt>>, ()>>()?
                .iter()
                .sum::<Ratio<BigInt>>(),
        ))
    }

    pub fn split_ev(
        &self,
        cancellation_token: &CancellationToken,
    ) -> Result<Option<Ratio<BigInt>>, ()> {
        let player_card: Card;
        match self.player_hand {
            Hand::Pair(c) => {
                player_card = c;
            }
            _ => return Ok(None),
        }

        Ok(Some(
            self.deck_cards
                .remaining_ranks()
                .par_iter() // 並列イテレーションに変更
                .map(|&rank| {
                    if cancellation_token.is_cancelled() {
                        return Err(());
                    }

                    let next_deck_cards = self.deck_cards.remove(rank);

                    let round = BlackJackGame {
                        rule: Arc::clone(&self.rule),
                        dealer_card: self.dealer_card,
                        player_hand: Cards::from_smallvec(smallvec![player_card, rank]).into(),
                        deck_cards: next_deck_cards,
                        player_card_count: 1,
                    };

                    let draw_prob = self.deck_cards.draw_probability(rank);
                    if player_card == Card::Ace {
                        Ok(round.get_stand_ev(cancellation_token)?
                            * BigInt::from(2)
                            * Ratio::new(
                                BigInt::from(*draw_prob.numer()),
                                BigInt::from(*draw_prob.denom()),
                            ))
                    } else {
                        Ok(round.hit_or_stand_ev(cancellation_token)?
                            * BigInt::from(2)
                            * Ratio::new(
                                BigInt::from(*draw_prob.numer()),
                                BigInt::from(*draw_prob.denom()),
                            ))
                    }
                })
                .collect::<Result<Vec<Ratio<BigInt>>, ()>>()?
                .iter()
                .sum::<Ratio<BigInt>>(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::HandTotal;
    use smallvec::smallvec;

    #[test]
    fn test_stand_ev1() {
        let dealer_card = Card::Face;

        #[rustfmt::skip]
        let player_hands: Vec<(Hand, (i64, i64))> = vec![
            (Hand::Hard(HandTotal::Value(12)), (-21829933, 38172225)),
            (Hand::Hard(HandTotal::Value(13)), (-21829933, 38172225)),
            (Hand::Hard(HandTotal::Value(14)), (-21829933, 38172225)),
            (Hand::Hard(HandTotal::Value(15)), (-21829933, 38172225)),
            (Hand::Hard(HandTotal::Value(16)), (-21829933, 38172225)),
            (Hand::Hard(HandTotal::Value(17)), (-23435103, 50896300)),
            (Hand::Hard(HandTotal::Value(18)), (-9125656, 38172225)),
            (Hand::Hard(HandTotal::Value(19)), (-2664967, 152688900)),
            (Hand::Hard(HandTotal::Value(20)), (2215119, 5089630)),
            (Hand::Hard(HandTotal::Value(21)), (8847179, 10906350)),
            (Hand::Soft(HandTotal::Value(13)), (-21829933, 38172225)),
            (Hand::Soft(HandTotal::Value(14)), (-21829933, 38172225)),
            (Hand::Soft(HandTotal::Value(15)), (-21829933, 38172225)),
            (Hand::Soft(HandTotal::Value(16)), (-21829933, 38172225)),
            (Hand::Soft(HandTotal::Value(17)), (-23435103, 50896300)),
            (Hand::Soft(HandTotal::Value(18)), (-9125656, 38172225)),
            (Hand::Soft(HandTotal::Value(19)), (-2664967, 152688900)),
            (Hand::Soft(HandTotal::Value(20)), (2215119, 5089630)),
            (Hand::Soft(HandTotal::Value(21)), (8847179, 10906350)),
            (Hand::Soft(HandTotal::BlackJack), (18, 13)),
            (Hand::Hard(HandTotal::Burst), (-1, 1)),
            (Hand::Pair(Card::N2), (-21829933, 38172225)),
            (Hand::Pair(Card::N3), (-21829933, 38172225)),
            (Hand::Pair(Card::N4), (-21829933, 38172225)),
            (Hand::Pair(Card::N5), (-21829933, 38172225)),
            (Hand::Pair(Card::N6), (-21829933, 38172225)),
            (Hand::Pair(Card::N7), (-21829933, 38172225)),
            (Hand::Pair(Card::N8), (-21829933, 38172225)),
            (Hand::Pair(Card::N9), (-9125656, 38172225)),
            (Hand::Pair(Card::Face), (2215119, 5089630)),
            (Hand::Pair(Card::Ace), (-21829933, 38172225)),
        ];
        player_hands.iter().for_each(|(hand, prob)| {
            let deck_cards = Deck::new(1);
            let ratio = Ratio::new(BigInt::from(prob.0), BigInt::from(prob.1));
            let rule = Rule::evolution_classic();
            let round = BlackJackGame::new(rule, dealer_card, hand.clone(), deck_cards, 1);
            let token = CancellationToken::new();
            assert_eq!(
                round.get_stand_ev(&token).unwrap(),
                ratio,
                "hand: {:?}",
                hand
            );
        });
    }

    #[test]
    fn test_stand_ev_blackjack() {
        let dealer_card = Card::N6;
        let user_cards = Cards::from_smallvec(smallvec![Card::Ace, Card::Face]).into();
        let deck_cards = Deck::new_from_strs(&vec!["T", "T"]);
        let rule = Rule::evolution_classic();
        let round = BlackJackGame::new(rule, dealer_card, user_cards, deck_cards, 1);
        let token = CancellationToken::new();
        assert_eq!(
            round.get_stand_ev(&token).unwrap(),
            Ratio::from_float(1.5).unwrap()
        );
    }

    #[test]
    fn test_stand_ev_burst() {
        let dealer_card = Card::N2;
        let user_cards = Cards::from_smallvec(smallvec![Card::Face, Card::Face, Card::Face]).into();
        let deck_cards = Deck::new(1);
        let rule = Rule::evolution_classic();
        let round = BlackJackGame::new(rule, dealer_card, user_cards, deck_cards, 1);
        let token = CancellationToken::new();

        assert_eq!(
            round.get_stand_ev(&token).unwrap(),
            Ratio::from_integer(BigInt::from(-1))
        );
    }

    #[test]
    fn test_stand_ev_dealer_burst() {
        let dealer_card = Card::Face;
        let user_cards = Cards::from_smallvec(smallvec![Card::Face, Card::N2]).into();
        let deck_cards = Deck::new_from_strs(&vec!["6", "6"]);
        let rule = Rule::evolution_classic();
        let round = BlackJackGame::new(rule, dealer_card, user_cards, deck_cards, 1);
        let token = CancellationToken::new();

        assert_eq!(
            round.get_stand_ev(&token).unwrap(),
            Ratio::from_integer(BigInt::from(1))
        );
    }

    #[test]
    fn test_stand_ev_dealer_blackjack() {
        let dealer_card = Card::Face;
        let user_cards = Cards::from_smallvec(smallvec![Card::Face, Card::N2]).into();
        let deck_cards = Deck::new_from_strs(&vec!["A"]);
        let rule = Rule::evolution_classic();
        let round = BlackJackGame::new(rule, dealer_card, user_cards, deck_cards, 1);
        let token = CancellationToken::new();

        assert_eq!(
            round.get_stand_ev(&token).unwrap(),
            Ratio::from_integer(BigInt::from(-1))
        );
    }

    #[test]
    fn test_hit_ev1() {
        let dealer_card = Card::Face;

        #[rustfmt::skip]
        let player_hands: Vec<(Hand, (i128, i128))> = vec![
            (Hand::Hard(HandTotal::Value(4)), (-13689957489251, 37374807219750)),
            (Hand::Hard(HandTotal::Value(5)), (-3771395696811, 9689764834750)),
            (Hand::Hard(HandTotal::Value(6)), (-70135504890469, 174415767025500)),
            (Hand::Hard(HandTotal::Value(7)), (-13043991377, 34897112250)),
            (Hand::Hard(HandTotal::Value(8)), (-3236262234713, 10678516348500)),
            (Hand::Hard(HandTotal::Value(9)), (-1441653790309, 6795419494500)),
            (Hand::Hard(HandTotal::Value(10)), (-5743817, 110372262)),
            (Hand::Hard(HandTotal::Value(11)), (2127284281, 79016505750)),
            (Hand::Hard(HandTotal::Value(12)), (-9743496797, 22576144500)),
            (Hand::Hard(HandTotal::Value(13)), (-8273979149, 17559223500)),
            (Hand::Hard(HandTotal::Value(14)), (-535699399, 1053553410)),
            (Hand::Hard(HandTotal::Value(15)), (-5722583939, 10535534100)),
            (Hand::Hard(HandTotal::Value(16)), (-1517105488, 2633883525)),
            (Hand::Hard(HandTotal::Value(17)), (-6497749943, 10535534100)),
            (Hand::Hard(HandTotal::Value(18)), (-78892169, 117061490)),
            (Hand::Hard(HandTotal::Value(19)), (-131398262, 175592235)),
            (Hand::Hard(HandTotal::Value(20)), (-32750363, 38172225)),
            (Hand::Soft(HandTotal::Value(13)), (-91803515069723, 523247301076500)),
            (Hand::Soft(HandTotal::Value(14)), (-21449284016533, 104649460215300)),
            (Hand::Soft(HandTotal::Value(15)), (-8343882969077, 34883153405100)),
            (Hand::Soft(HandTotal::Value(16)), (-20080657174343, 74749614439500)),
            (Hand::Soft(HandTotal::Value(17)), (-1618906065863, 6229134536625)),
            (Hand::Soft(HandTotal::Value(18)), (-19931581951, 96575729250)),
            (Hand::Soft(HandTotal::Value(19)), (-13239131611, 86918156325)),
            (Hand::Soft(HandTotal::Value(20)), (-5743817, 110372262)),
            (Hand::Pair(Card::N2), (-914224158773, 2669629087125)),
            (Hand::Pair(Card::N3), (-202229317862381, 523247301076500)),
            (Hand::Pair(Card::N4), (-3236262234713, 10678516348500)),
            (Hand::Pair(Card::N5), (-5743817, 110372262)),
            (Hand::Pair(Card::N6), (-9743496797, 22576144500)),
            (Hand::Pair(Card::N7), (-535699399, 1053553410)),
            (Hand::Pair(Card::N8), (-1517105488, 2633883525)),
            (Hand::Pair(Card::N9), (-78892169, 117061490)),
            (Hand::Pair(Card::Face), (-32750363, 38172225)),
            (Hand::Pair(Card::Ace), (-6614100976199, 46738865673500)),
        ];
        player_hands.iter().for_each(|(hand, prob)| {
            let deck_cards = Deck::new(1);
            let ratio = Ratio::new(BigInt::from(prob.0), BigInt::from(prob.1));
            let rule = Rule::evolution_classic();
            let round = BlackJackGame::new(rule, dealer_card, hand.clone(), deck_cards, 1);
            let token = CancellationToken::new();

            assert_eq!(
                round.get_hit_ev(&token).unwrap(),
                Some(ratio),
                "hand: {:?}",
                hand
            );
        });
    }

    #[test]
    fn test_hit_ev_burst() {
        let dealer_card = Card::Face;
        let user_cards = Cards::from_smallvec(smallvec![Card::Face, Card::Face]).into();
        let deck_cards = Deck::new_from_strs(&vec!["3"]);
        let rule = Rule::evolution_classic();
        let round = BlackJackGame::new(rule, dealer_card, user_cards, deck_cards, 1);
        let token = CancellationToken::new();

        assert_eq!(
            round.get_hit_ev(&token).unwrap(),
            Some(Ratio::from_integer(BigInt::from(-1)))
        );
    }

    #[test]
    fn test_hit_ev_21() {
        let dealer_card = Card::N6;
        let user_cards = Cards::from_smallvec(smallvec![Card::N6, Card::N4]).into();
        let deck_cards = Deck::new_from_strs(&vec!["A", "A"]);
        let rule = Rule::evolution_classic();
        let round = BlackJackGame::new(rule, dealer_card, user_cards, deck_cards, 1);
        let token = CancellationToken::new();

        assert_eq!(
            round.get_hit_ev(&token).unwrap(),
            Some(Ratio::from_integer(BigInt::from(1)))
        );
    }

    #[test]
    fn test_stand_ev_blackjack2() {
        let dealer_card = Card::Face;
        let user_cards = Cards::from_smallvec(smallvec![Card::Face, Card::Ace]).into();
        let deck_cards = Deck::new_from_strs(&vec!["6", "6"]);
        let rule = Rule::evolution_classic();
        let round = BlackJackGame::new(rule, dealer_card, user_cards, deck_cards, 1);
        let token = CancellationToken::new();

        assert_eq!(
            round.get_stand_ev(&token).unwrap(),
            Ratio::from_float(1.5).unwrap()
        );
    }

    #[test]
    fn test_double_ev1() {
        let dealer_card = Card::Face;
        let user_cards = Cards::from_smallvec(smallvec![Card::N6, Card::N6]).into();
        let deck_cards = Deck::new_from_strs(&vec![
            "A", "A", "A", "A", "A", "A", "A", "A", "A", "A", "A", "A", "A", "A", "A", "A",
        ]);
        let rule = Rule::evolution_classic();
        let round = BlackJackGame::new(rule, dealer_card, user_cards, deck_cards, 1);
        let token = CancellationToken::new();

        assert_eq!(
            round.double_ev(&token).unwrap(),
            Some(Ratio::from_integer(BigInt::from(-2)))
        );
    }

    #[test]
    fn test_double_ev2() {
        let dealer_card = Card::N6;
        let user_cards = Cards::from_smallvec(smallvec![Card::N5, Card::N5]).into();
        let deck_cards = Deck::new_from_strs(&vec!["A", "A", "A", "A"]);
        let rule = Rule::evolution_classic();
        let round = BlackJackGame::new(rule, dealer_card, user_cards, deck_cards, 1);
        let token = CancellationToken::new();

        assert_eq!(
            round.double_ev(&token).unwrap(),
            Some(Ratio::from_integer(BigInt::from(2)))
        );
    }

    #[test]
    #[ignore]
    fn test_double_ev3() {
        let dealer_card = Card::Face;

        #[rustfmt::skip]
        let player_hands: Vec<(Hand, (i128, i128))> = vec![
            (Hand::Hard(HandTotal::Value(4)), (-13689957489251, 18687403609875)),
            (Hand::Hard(HandTotal::Value(5)), (-3771395696811, 4844882417375)),
            (Hand::Hard(HandTotal::Value(6)), (-70135504890469, 87207883512750)),
            (Hand::Hard(HandTotal::Value(7)), (-13043991377, 17448556125)),
            (Hand::Hard(HandTotal::Value(8)), (-3236262234713, 5339258174250)),
            (Hand::Hard(HandTotal::Value(9)), (-1441653790309, 3397709747250)),
            (Hand::Hard(HandTotal::Value(10)), (-5743817, 55186131)),
            (Hand::Hard(HandTotal::Value(11)), (2127284281, 39508252875)),
            (Hand::Hard(HandTotal::Value(12)), (-9743496797, 11288072250)),
            (Hand::Hard(HandTotal::Value(13)), (-8273979149, 8779611750)),
            (Hand::Hard(HandTotal::Value(14)), (-535699399, 526776705)),
            (Hand::Hard(HandTotal::Value(15)), (-5722583939, 5267767050)),
            (Hand::Hard(HandTotal::Value(16)), (-3034210976, 2633883525)),
            (Hand::Hard(HandTotal::Value(17)), (-6497749943, 5267767050)),
            (Hand::Hard(HandTotal::Value(18)), (-78892169, 58530745)),
            (Hand::Hard(HandTotal::Value(19)), (-262796524, 175592235)),
            (Hand::Hard(HandTotal::Value(20)), (-65500726, 38172225)),
            (Hand::Soft(HandTotal::Value(13)), (-91803515069723, 261623650538250)),
            (Hand::Soft(HandTotal::Value(14)), (-21449284016533, 52324730107650)),
            (Hand::Soft(HandTotal::Value(15)), (-8343882969077, 17441576702550)),
            (Hand::Soft(HandTotal::Value(16)), (-20080657174343, 37374807219750)),
            (Hand::Soft(HandTotal::Value(17)), (-3237812131726, 6229134536625)),
            (Hand::Soft(HandTotal::Value(18)), (-19931581951, 48287864625)),
            (Hand::Soft(HandTotal::Value(19)), (-26478263222, 86918156325)),
            (Hand::Soft(HandTotal::Value(20)), (-5743817, 55186131)),
            (Hand::Pair(Card::N2), (-1828448317546, 2669629087125)),
            (Hand::Pair(Card::N3), (-202229317862381, 261623650538250)),
            (Hand::Pair(Card::N4), (-3236262234713, 5339258174250)),
            (Hand::Pair(Card::N5), (-5743817, 55186131)),
            (Hand::Pair(Card::N6), (-9743496797, 11288072250)),
            (Hand::Pair(Card::N7), (-535699399, 526776705)),
            (Hand::Pair(Card::N8), (-3034210976, 2633883525)),
            (Hand::Pair(Card::N9), (-78892169, 58530745)),
            (Hand::Pair(Card::Face), (-65500726, 38172225)),
            (Hand::Pair(Card::Ace), (-6614100976199, 23369432836750)),
        ];
        player_hands.iter().for_each(|(hand, prob)| {
            let deck_cards = Deck::new(1);
            let rule = Rule::evolution_classic();
            let round = BlackJackGame::new(rule, dealer_card, hand.clone(), deck_cards, 1);
            let token = CancellationToken::new();
            assert_eq!(
                round.double_ev(&token).unwrap(),
                Some(Ratio::new(BigInt::from(prob.0), BigInt::from(prob.1)))
            );
            // println!("hand: {:?}, ev: {:?}", hand, round.double_ev());
        });
    }

    #[test]
    #[ignore]
    fn test_double_ev4() {
        let dealer_card = Card::N6;

        #[rustfmt::skip]
        let player_hands: Vec<(Hand, (i128, i128))> = vec![
            (Hand::Hard(HandTotal::Value(4)), (-5698721544979, 261623650538250)),
            (Hand::Hard(HandTotal::Value(5)), (-49279404379, 1137494132775)),
            (Hand::Hard(HandTotal::Value(6)), (-2201966416976, 43603941756375)),
            (Hand::Hard(HandTotal::Value(7)), (1867482761, 37752330525)),
            (Hand::Hard(HandTotal::Value(8)), (4236328099568, 18687403609875)),
            (Hand::Hard(HandTotal::Value(9)), (1465070099684, 3737480721975)),
            (Hand::Hard(HandTotal::Value(10)), (50253401537, 86918156325)),
            (Hand::Hard(HandTotal::Value(11)), (98089871951, 144863593875)),
            (Hand::Hard(HandTotal::Value(12)), (-8160470939, 24833758950)),
            (Hand::Hard(HandTotal::Value(13)), (-6121330657, 13372024050)),
            (Hand::Hard(HandTotal::Value(14)), (-569438479, 968987250)),
            (Hand::Hard(HandTotal::Value(15)), (-9049073083, 12596834250)),
            (Hand::Hard(HandTotal::Value(16)), (-2764856768, 3219190975)),
            (Hand::Hard(HandTotal::Value(17)), (-59286831221, 57945437550)),
            (Hand::Hard(HandTotal::Value(18)), (-1058888907779, 869181563250)),
            (Hand::Hard(HandTotal::Value(19)), (-46571321967, 32191909750)),
            (Hand::Hard(HandTotal::Value(20)), (-44979836743, 26338835250)),
            (Hand::Soft(HandTotal::Value(13)), (39055393177, 125841101750)),
            (Hand::Soft(HandTotal::Value(14)), (6034573799, 22651398315)),
            (Hand::Soft(HandTotal::Value(15)), (8417337298463, 37374807219750)),
            (Hand::Soft(HandTotal::Value(16)), (17425079239, 96575729250)),
            (Hand::Soft(HandTotal::Value(17)), (6234121471, 26338835250)),
            (Hand::Soft(HandTotal::Value(18)), (322250643059, 869181563250)),
            (Hand::Soft(HandTotal::Value(19)), (41367927214, 86918156325)),
            (Hand::Soft(HandTotal::Value(20)), (50253401537, 86918156325)),
            (Hand::Pair(Card::N2), (925172424887, 52324730107650)),
            (Hand::Pair(Card::N3), (-968454169499, 29069294504250)),
            (Hand::Pair(Card::N4), (4236328099568, 18687403609875)),
            (Hand::Pair(Card::N5), (50253401537, 86918156325)),
            (Hand::Pair(Card::N6), (-8160470939, 24833758950)),
            (Hand::Pair(Card::N7), (-569438479, 968987250)),
            (Hand::Pair(Card::N8), (-2764856768, 3219190975)),
            (Hand::Pair(Card::N9), (-1058888907779, 869181563250)),
            (Hand::Pair(Card::Face), (-44979836743, 26338835250)),
            (Hand::Pair(Card::Ace), (1476010744211, 3963994705125)),

        ];
        player_hands.iter().for_each(|(hand, prob)| {
            let deck_cards = Deck::new(1);
            let rule = Rule::evolution_classic();
            let round = BlackJackGame::new(rule, dealer_card, hand.clone(), deck_cards, 1);
            let token = CancellationToken::new();
            assert_eq!(
                round.double_ev(&token).unwrap(),
                Some(Ratio::new(BigInt::from(prob.0), BigInt::from(prob.1)))
            );
            // println!("hand: {:?}, ev: {:?}", hand, round.double_ev());
        });
    }

    #[test]
    fn test_split_ev1() {
        let dealer_card = Card::N6;
        let user_cards = Cards::from_smallvec(smallvec![Card::Face, Card::Face]).into();
        let deck_cards = Deck::new_from_strs(&vec!["T", "T", "T", "T"]);
        let rule = Rule::evolution_classic();
        let round = BlackJackGame::new(rule, dealer_card, user_cards, deck_cards, 1);
        let token = CancellationToken::new();

        assert_eq!(
            round.split_ev(&token).unwrap(),
            Some(Ratio::from_integer(BigInt::from(2)))
        );
    }

    #[test]
    fn test_split_ev2() {
        let dealer_card = Card::Face;
        let user_cards = Cards::from_smallvec(smallvec![Card::N9, Card::N9]).into();
        let deck_cards = Deck::new_from_strs(&vec![
            "A", "A", "A", "A", "A", "A", "A", "A", "A", "A", "A", "A", "A", "A", "A", "A", "A",
            "A", "A", "A", "A", "A", "A", "A",
        ]);
        let rule = Rule::evolution_classic();
        let round = BlackJackGame::new(rule, dealer_card, user_cards, deck_cards, 1);
        let token = CancellationToken::new();

        assert_eq!(
            round.split_ev(&token).unwrap(),
            Some(Ratio::from_integer(BigInt::from(-2)))
        );
    }
    #[test]
    fn test_ev1() {
        let dealer_card = Card::Face;
        let user_cards = Hand::Hard(HandTotal::Value(12));
        let deck_cards = Deck::new(1);
        let rule = Rule::evolution_classic();

        let round = BlackJackGame::new(rule, dealer_card, user_cards, deck_cards, 1);

        let numer: BigInt = "-9743496797".parse().unwrap();
        let denom: BigInt = "22576144500".parse().unwrap();

        assert_eq!(round.ev(), Ratio::<BigInt>::new(numer, denom),);
    }

    #[test]
    fn test_hand_none_ev() {
        let dealer_card = Card::Face;
        let user_cards = Hand::None;
        let deck_cards = Deck::new(1);
        let rule = Rule::evolution_classic();

        let round = BlackJackGame::new(rule, dealer_card, user_cards, deck_cards, 1);

        let numer: BigInt = "-391894135935091".parse().unwrap();
        let denom: BigInt = "1833601653345000".parse().unwrap();

        assert_eq!(round.ev(), Ratio::<BigInt>::new(numer, denom),);
    }

    // #[test]
    // fn test_preround_dealer_prob() {
    //     let deck = Deck::new(8);
    //     let rule = Rule::evolution_classic();
    //     let pre_round = PreBlackJackGame::new(rule, deck.clone());
    //     let probs = pre_round.dealer_prob();
    //     // probsをすべて足すと１になること
    //     assert_eq!(probs.iter().sum::<f64>(), 1.0);
    // }

    use std::time::Duration;
    use tokio::time::Instant;

    #[tokio::test]
    async fn test_ev_with_cancellation() {
        // Create a new game with 8 decks
        let deck = Deck::new(8);
        let rule = Rule::evolution_classic();
        let game = BlackJackGame::new(rule, Card::N2, Hand::Hard(HandTotal::Value(2)), deck, 1);

        // Create a cancellation token
        let cancel_token = CancellationToken::new();

        println!("Starting the task");

        // Use spawn_blocking for the EV calculation
        let handle = tokio::task::spawn_blocking({
            let cancel_token = cancel_token.clone();
            move || {
                let start = Instant::now();
                let ev = game.ev_with_cancellation_token(&cancel_token);
                let duration = start.elapsed();
                (ev, duration)
            }
        });

        // Wait 3 seconds before canceling the task
        tokio::time::sleep(Duration::from_secs(3)).await;
        println!("Cancelling the task");
        cancel_token.cancel();

        // Wait for the task to complete
        let (ev_result, duration) = handle.await.unwrap();

        // Check the result
        assert!(
            ev_result.is_err(),
            "Task should be cancelled and return an error"
        );
        assert!(
            duration >= Duration::from_secs(3),
            "Task should take at least 3 seconds to complete"
        );
    }

    // #[tokio::test]
    // async fn test_pre_ev_with_cancellation() {
    //     // Create a new game with 8 decks
    //     let deck = Deck::new(8);
    //     let rule = Rule::evolution_classic();
    //     let game = PreBlackJackGame::new(rule, deck);

    //     // Create a cancellation token
    //     let cancel_token = CancellationToken::new();

    //     println!("Starting the task");

    //     // Use spawn_blocking for the EV calculation
    //     let handle = tokio::task::spawn_blocking({
    //         let cancel_token = cancel_token.clone();
    //         move || {
    //             let start = Instant::now();
    //             let ev = game.ev_with_cancellation_token(&cancel_token);
    //             let duration = start.elapsed();
    //             (ev, duration)
    //         }
    //     });

    //     // Wait 3 seconds before canceling the task
    //     tokio::time::sleep(Duration::from_secs(10)).await;
    //     println!("Cancelling the task");
    //     cancel_token.cancel();

    //     // Wait for the task to complete
    //     let (ev_result, duration) = handle.await.unwrap();

    //     // Check the result
    //     assert!(
    //         ev_result.is_err(),
    //         "Task should be cancelled and return an error"
    //     );
    //     assert!(
    //         duration >= Duration::from_secs(3),
    //         "Task should take at least 3 seconds to complete"
    //     );
    // }
}
