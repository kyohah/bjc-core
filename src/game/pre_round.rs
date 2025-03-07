use crate::game::BlackJackGame;
use crate::models::{Card, Deck, HandTotal, PreRoundPattern, Rule};

use num::bigint::BigInt;
use num::rational::Ratio;
use num::ToPrimitive;
use num::Zero;
use std::collections::HashMap;

use rayon::prelude::*;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

pub struct PreBlackJackGame {
    rule: Arc<Rule>,
    deck: Deck,
}

impl PreBlackJackGame {
    pub fn new(rule: Rule, deck: Deck) -> Self {
        PreBlackJackGame {
            rule: Arc::new(rule),
            deck,
        }
    }

    pub fn calc_ev(&self) -> Ratio<BigInt> {
        let token = CancellationToken::new();
        self.calc_ev_with_cancellation_token(&token).unwrap()
    }

    pub fn ev(&self) -> Ratio<BigInt> {
        let token = CancellationToken::new();
        self.ev_with_cancellation_token(&token).unwrap()
    }

    pub fn ev_with_cancellation_token(
        &self,
        cancellation_token: &CancellationToken,
    ) -> Result<Ratio<BigInt>, ()> {
        let ev = self.calc_ev_with_cancellation_token(cancellation_token)?;
        Ok(ev)
    }

    #[cfg(feature = "onnx")]
    pub fn ml_ev(&self) -> Result<f32, ()> {
        let input_data = self.input_onnx();

        let model_path = include_bytes!("../bjc.onnx");

        let inputs = ort::inputs!["features" => input_data].unwrap();
        // println!("inputs: {:?}", self.input_onnx());

        let session = ort::Session::builder()
            .unwrap()
            .commit_from_memory(model_path)
            .unwrap();

        let outputs = session.run(inputs).unwrap();
        // モデルのセッションを作成
        let scaled_ev = outputs["predictions"]
            .try_extract_tensor::<f32>()
            .unwrap()
            .to_owned()[[0, 0]];

        Ok(PreBlackJackGame::ev_unscaled(scaled_ev))
    }

    pub fn calc_ev_with_cancellation_token(
        &self,
        cancellation_token: &CancellationToken,
    ) -> Result<Ratio<BigInt>, ()> {
        PreRoundPattern::all()
            .par_iter()
            .map(|pattern| {
                if cancellation_token.is_cancelled() {
                    return Err(());
                }
                let ev = self.pre_round_ev_with_cancellation_token(&pattern, cancellation_token)?;
                Ok(ev)
            })
            .collect::<Result<Vec<Ratio<BigInt>>, ()>>()
            .map(|evs| evs.iter().sum())
    }

    // dealerのそれになる確率を返す
    //  HandTotal::BlackJack,
    // HandTotal::Value(17),
    // HandTotal::Value(18),
    // HandTotal::Value(19),
    // HandTotal::Value(20),
    // HandTotal::Value(21),
    // HandTotal::Burst,
    // pub dealer_prob(&self)
    pub fn dealer_prob(&self) -> [f64; 7] {
        let binding = self.deck.dealer_probs();
        let mut tmp_probs = HashMap::new();

        for card in Card::ALL {
            let card_probs = binding.get(card);
            for (total, prob) in card_probs.iter() {
                // le tmp_probs.entry(total).or_insert(0.0)
                let entry = tmp_probs.entry(total).or_insert(Ratio::zero());
                let draw_prob = self.deck.draw_probability(card);
                *entry += prob * Ratio::new(*draw_prob.numer() as u128, *draw_prob.denom() as u128);
            }
        }

        // 指定したハンドの確率を配列に格納
        let totals = [
            HandTotal::BlackJack,
            HandTotal::Value(17),
            HandTotal::Value(18),
            HandTotal::Value(19),
            HandTotal::Value(20),
            HandTotal::Value(21),
            HandTotal::Burst,
        ];

        let mut probs = [0.0; 7];
        for (i, total) in totals.iter().enumerate() {
            probs[i] = tmp_probs
                .get(&total)
                .map_or(0.0, |prob| prob.to_f64().unwrap_or(0.0));
        }
        probs
    }

    #[cfg(feature = "onnx")]
    pub fn input_onnx(&self) -> ArrayBase<OwnedRepr<f32>, Dim<[usize; 2]>> {
        let mut output = Vec::new();
        let deck_counts = self.deck.total_cards();
        let deck_maisu = self
            .deck
            .to_array()
            .into_iter()
            .map(|count| count as f32 / deck_counts as f32)
            .collect::<Vec<f32>>();

        let deck_counts = self.deck.total_cards() as f32 / 416 as f32;
        let dealer_probs = self.dealer_prob();

        output.extend(deck_maisu.into_iter().map(|count| count));
        output.push(deck_counts as f32);
        output.extend(dealer_probs.into_iter().map(|prob| prob as f32));

        // vecをArrayBaseに変換

        let input_data: ArrayBase<OwnedRepr<f32>, Dim<[usize; 2]>> =
            Array2::from_shape_vec((1, 18), output).unwrap();

        input_data
    }

    pub fn pre_round_ev_with_cancellation_token(
        &self,
        pre_round_pattern: &PreRoundPattern,
        cancellation_token: &CancellationToken,
    ) -> Result<Ratio<BigInt>, ()> {
        let prob = self.deck.deck_draw_probability(&pre_round_pattern.all_deck)
            * pre_round_pattern.weight as u128;

        let deck_cards = self.deck.remove_deck(&pre_round_pattern.all_deck);

        let black_jack_game = BlackJackGame {
            rule: Arc::clone(&self.rule),
            dealer_card: pre_round_pattern.dealer_card,
            player_hand: pre_round_pattern.player_hand.clone(),
            deck_cards: deck_cards.clone(),
            player_card_count: 2,
        };
        let ev = black_jack_game.ev_with_cancellation_token(cancellation_token)?;

        Ok(ev * Ratio::new(BigInt::from(*prob.numer()), BigInt::from(*prob.denom())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Hand;
    use std::time::Duration;
    use tokio::time::Instant;

    #[test]
    fn test_pre_round() {
        let deck = Deck::new(8);
        let rule = Rule::evolution_classic();
        let pre_round = PreBlackJackGame::new(rule, deck.clone());
        assert_eq!(
            pre_round.ev(),
            Ratio::new(
                "-1243148184227145034346117698114444483224258977"
                    .parse()
                    .unwrap(),
                "221172350277495373076549362381539684887036067525"
                    .parse()
                    .unwrap()
            )
        );

        // 並列化時
        //  numer: 1016350091974324226323438723196054028, denom: 93112157693523724162642086744350091091
    }
    #[test]
    fn test_preround_dealer_prob() {
        let deck = Deck::new(8);
        let rule = Rule::evolution_classic();
        let pre_round = PreBlackJackGame::new(rule, deck.clone());
        let probs = pre_round.dealer_prob();
        // probsをすべて足すと１になること
        assert_eq!(probs.iter().sum::<f64>(), 1.0);
    }

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

    #[tokio::test]
    async fn test_pre_ev_with_cancellation() {
        // Create a new game with 8 decks
        let deck = Deck::new(8);
        let rule = Rule::evolution_classic();
        let game = PreBlackJackGame::new(rule, deck);

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
        tokio::time::sleep(Duration::from_secs(10)).await;
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
}
