use num::One;
use num_rational::Ratio;
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Rule {
    pub decks: usize,              // デッキ数
    pub without_9_t: bool,         // 9と10を除くか
    pub free_split_9_10_11: bool,  // 9, 10, 11でフリースプリット
    pub free_double_9_10_11: bool, // 9, 10, 11でフリーダブル
    pub triple_double: bool,       // 3倍のダブル
    pub quad_double: bool,         // 4倍のダブル
    pub double_after_split: bool,  // スプリット後にダブルできるか
    pub hit_split_aces: bool,      // スプリットエースにヒットできるか
    pub six_card_charlie: bool,    // 6枚のカードで勝てるか
    //4-17, 18, 19, 20, 21, BJごとに倍率がある
    pub multiplier_black_jack: Ratio<usize>,
    pub multiplier_lteq_17: Ratio<usize>,
    pub multiplier_18: Ratio<usize>,
    pub multiplier_19: Ratio<usize>,
    pub multiplier_20: Ratio<usize>,
    pub multiplier_21: Ratio<usize>,
}
