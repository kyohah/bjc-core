use bjc::game::PreBlackJackGame;
use bjc::models::{Deck, Rule};
use criterion::{criterion_group, criterion_main, Criterion};
use num::rational::Ratio;

fn bench() {
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
}

fn criterion_bench(c: &mut Criterion) {
    c.bench_function("evaluate_hand (133,784,560 hands)", |b| b.iter(|| bench()));
}

criterion_group!(benches, criterion_bench);
criterion_main!(benches);
