use crate::models::{Card, Cards, Deck, Hand};
use serde::{Deserialize, Serialize};
use smallvec::smallvec;
use std::sync::LazyLock;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PreRoundPattern {
    pub dealer_card: Card,
    pub player_pattern: Deck,
    pub player_hand: Hand,
    pub all_deck: Deck,
    pub weight: usize,
}

static PRE_ROUND_PATTERN_REPOSITORY: LazyLock<Vec<PreRoundPattern>> =
    LazyLock::new(PreRoundPattern::generate);

impl PreRoundPattern {
    fn new(
        dealer_card: Card,
        player_pattern: Deck,
        player_hand: Hand,
        all_deck: Deck,
        weight: usize,
    ) -> Self {
        PreRoundPattern {
            dealer_card,
            player_pattern,
            player_hand,
            all_deck,
            weight,
        }
    }

    fn generate_player_patterns() -> Vec<Deck> {
        Card::ALL
            .iter()
            .enumerate()
            .flat_map(|(i, &rank1)| {
                Card::ALL.iter().skip(i).map(move |&rank2| {
                    let cards = Cards::from_smallvec(smallvec![rank1, rank2]);
                    Deck::new_from_cards(&cards)
                })
            })
            .collect()
    }

    fn generate() -> Vec<PreRoundPattern> {
        let player_patterns = Self::generate_player_patterns();
        Card::ALL
            .iter()
            .flat_map(|&dealer_card| {
                player_patterns.iter().map(move |player_pattern| {
                    let player_hand: Hand = (*player_pattern).clone().into();
                    let weight = if player_hand.is_pair() { 1 } else { 2 };
                    let all_deck = player_pattern.add(dealer_card);

                    PreRoundPattern::new(
                        dealer_card,
                        player_pattern.clone(),
                        player_hand,
                        all_deck,
                        weight,
                    )
                })
            })
            .collect()
    }

    pub(crate) fn all() -> &'static Vec<PreRoundPattern> {
        &*PRE_ROUND_PATTERN_REPOSITORY
    }
}
#[cfg(test)]
mod tests {
    use num_rational::Ratio;

    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_generate_pre_round_patterns() {
        let patterns = PreRoundPattern::generate();

        // 全てのPreRoundPatternが生成されているかを確認する
        // 10種類のCardがあるため、(10 * (10 + 1)) / 2 = 55通りのplayer_patternと、10種類のdealer_cardの組み合わせを確認
        let expected_len = 55 * Card::ALL.len(); // 55 * 10 = 550
        assert_eq!(patterns.len(), expected_len);

        // 重複がないか確認する
        let mut seen = HashSet::new();
        for pattern in &patterns {
            let entry = (
                pattern.dealer_card,
                pattern.player_pattern.clone(),
                pattern.player_hand.clone(),
                pattern.all_deck.clone(),
                pattern.weight,
            );
            assert!(seen.insert(entry), "Duplicate pattern found!");
        }
    }

    #[test]
    fn test_pre_round_pattern_properties() {
        let patterns = PreRoundPattern::generate();

        for pattern in patterns {
            // `player_pattern` にディーラーカードが追加されていることを確認
            let expected_deck = pattern.player_pattern.add(pattern.dealer_card);
            assert_eq!(
                pattern.all_deck, expected_deck,
                "All deck does not match expected."
            );

            // `weight` が正しく設定されているか確認
            if pattern.player_hand.is_pair() {
                assert_eq!(pattern.weight, 1, "Weight should be 1 for pairs.");
            } else {
                assert_eq!(pattern.weight, 2, "Weight should be 2 for non-pairs.");
            }
        }
    }

    #[test]
    fn test_all_function() {
        // `all()` が正しいパターンを返すか確認
        let patterns = PreRoundPattern::all();
        let generated_patterns = PreRoundPattern::generate();

        assert_eq!(patterns.len(), generated_patterns.len());
        assert_eq!(patterns, &generated_patterns);
    }

    #[test]
    // deckから、確率の合計が1になるかを確認する
    fn test_probability_sum() {
        let patterns = PreRoundPattern::all();

        let deck = Deck::new(8);
        let s: Ratio<u128> = patterns
            .iter()
            .map(|pattern| deck.deck_draw_probability(&pattern.all_deck) * pattern.weight as u128)
            .sum();
        assert_eq!(s, Ratio::new(1, 1));
    }
}
