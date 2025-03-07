use crate::models::{DealerHandDeckMap, Deck};
use maplit::btreemap;
use std::sync::LazyLock;

// This is an example static instance for the ace card patterns
pub(crate) static N8_PATTERNS: LazyLock<DealerHandDeckMap> = LazyLock::new(|| DealerHandDeckMap {
    value_17: btreemap! {
        Deck { ace: 3, n2: 3, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 1, n2: 0, n3: 0, n4: 2, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 1, n2: 2, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 7,
        Deck { ace: 1, n2: 0, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 2, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 0, n2: 1, n3: 0, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 2, n2: 1, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 0, n2: 3, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 0, n2: 0, n3: 3, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 1, n3: 1, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 6,
        Deck { ace: 3, n2: 0, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 2, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 12,
        Deck { ace: 4, n2: 0, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 3, n2: 1, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 5,
        Deck { ace: 3, n2: 0, n3: 2, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 5, n2: 2, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 0, n3: 1, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 0, n2: 0, n3: 0, n4: 1, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 0, n3: 1, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 2, n2: 0, n3: 1, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 6,
        Deck { ace: 1, n2: 1, n3: 2, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 8,
        Deck { ace: 2, n2: 0, n3: 0, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 6, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 5, n2: 0, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 4, n2: 1, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 6,
        Deck { ace: 1, n2: 4, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 1, n2: 1, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 0, n2: 0, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
    },
    value_18: btreemap! {
        Deck { ace: 2, n2: 2, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 5,
        Deck { ace: 5, n2: 1, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 3, n2: 2, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 6,
        Deck { ace: 0, n2: 0, n3: 0, n4: 1, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 2, n3: 2, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 6,
        Deck { ace: 0, n2: 1, n3: 0, n4: 2, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 1, n2: 0, n3: 1, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 3, n2: 0, n3: 1, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 1, n3: 1, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 6,
        Deck { ace: 1, n2: 0, n3: 0, n4: 1, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 1, n2: 1, n3: 0, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 1, n3: 2, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 7,
        Deck { ace: 0, n2: 0, n3: 0, n4: 0, n5: 2, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 0, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 3, n2: 1, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 4, n2: 1, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 1, n3: 1, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 10,
        Deck { ace: 0, n2: 2, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 2, n2: 0, n3: 0, n4: 2, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 1, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 2,
        Deck { ace: 4, n2: 0, n3: 2, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 5, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 3, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 0, n2: 0, n3: 2, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 1, n2: 3, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 9,
        Deck { ace: 1, n2: 2, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 4, n2: 3, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 1, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 4, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 2, n2: 0, n3: 1, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 1, n2: 0, n3: 3, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
    },
    value_19: btreemap! {
        Deck { ace: 0, n2: 0, n3: 1, n4: 2, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 0, n2: 0, n3: 2, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 0, n2: 1, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 1, n2: 2, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 3, n2: 1, n3: 2, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 5,
        Deck { ace: 0, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 2,
        Deck { ace: 1, n2: 0, n3: 2, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 6,
        Deck { ace: 0, n2: 0, n3: 0, n4: 0, n5: 1, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 2, n2: 1, n3: 1, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 8,
        Deck { ace: 3, n2: 0, n3: 0, n4: 2, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 0, n3: 0, n4: 1, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 1, n2: 0, n3: 0, n4: 1, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 1, n3: 3, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 1, n2: 3, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 2, n3: 0, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 3, n2: 0, n3: 1, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 2, n2: 2, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 1, n3: 1, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 6,
        Deck { ace: 0, n2: 1, n3: 1, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 0, n2: 3, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 4, n2: 0, n3: 1, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 1, n2: 0, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 2, n3: 2, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 7,
        Deck { ace: 1, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 1, n3: 0, n4: 1, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 2, n2: 3, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 0, n2: 4, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 3, n2: 2, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 5, n2: 0, n3: 2, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 0, n3: 1, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 4, n2: 2, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 0, n3: 0, n4: 1, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 2, n3: 1, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 6,
        Deck { ace: 1, n2: 1, n3: 0, n4: 2, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 2, n2: 0, n3: 3, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 1, n2: 0, n3: 0, n4: 0, n5: 2, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
    },
    value_20: btreemap! {
        Deck { ace: 1, n2: 0, n3: 2, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 0, n3: 0, n4: 3, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 5, n2: 0, n3: 1, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 4, n2: 0, n3: 1, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 4, n2: 0, n3: 0, n4: 2, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 1, n3: 2, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 2, n2: 0, n3: 0, n4: 1, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 3, n2: 0, n3: 0, n4: 1, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 3, n2: 1, n3: 1, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 5,
        Deck { ace: 2, n2: 0, n3: 2, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 1, n2: 0, n3: 0, n4: 1, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 1, n2: 2, n3: 0, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 1, n3: 1, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 0, n2: 3, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 0, n2: 4, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 1, n3: 0, n4: 1, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 6,
        Deck { ace: 2, n2: 3, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 1, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 0, n3: 0, n4: 0, n5: 2, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 1, n3: 0, n4: 2, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 0, n2: 0, n3: 1, n4: 1, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 0, n2: 2, n3: 1, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 0, n2: 1, n3: 0, n4: 1, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 0, n2: 1, n3: 1, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 0, n3: 0, n4: 0, n5: 1, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 0, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 2,
        Deck { ace: 4, n2: 2, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 2, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 1, n3: 1, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 1, n2: 3, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 1, n2: 0, n3: 1, n4: 2, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 0, n2: 2, n3: 0, n4: 2, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 0, n2: 0, n3: 2, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 2, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 0, n3: 0, n4: 0, n5: 1, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 1, n3: 0, n4: 0, n5: 2, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 1, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 0, n2: 0, n3: 0, n4: 0, n5: 0, n6: 2, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 3, n2: 2, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 2, n3: 1, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 7,
        Deck { ace: 3, n2: 0, n3: 1, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
    },
    value_21: btreemap! {
        Deck { ace: 1, n2: 1, n3: 0, n4: 0, n5: 2, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 0, n2: 1, n3: 0, n4: 1, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 2, n3: 0, n4: 1, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 0, n2: 0, n3: 0, n4: 2, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 0, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 0, n3: 0, n4: 0, n5: 1, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 1, n2: 0, n3: 0, n4: 0, n5: 0, n6: 2, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 0, n3: 2, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 2, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 0, n2: 1, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 2,
        Deck { ace: 2, n2: 2, n3: 0, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 1, n3: 0, n4: 1, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 1, n2: 2, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 3, n2: 0, n3: 0, n4: 1, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 3, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 4, n2: 2, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 1, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 1, n3: 1, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 1, n2: 0, n3: 1, n4: 1, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 0, n2: 0, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 2,
        Deck { ace: 4, n2: 0, n3: 1, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 0, n3: 1, n4: 1, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 1, n2: 2, n3: 1, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 7,
        Deck { ace: 0, n2: 4, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 0, n3: 2, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 0, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 0, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 0, n2: 1, n3: 2, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 1, n2: 3, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 2, n2: 0, n3: 0, n4: 1, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 2, n3: 1, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 2, n2: 3, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 0, n2: 0, n3: 1, n4: 0, n5: 2, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 3, n2: 0, n3: 0, n4: 0, n5: 2, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 0, n3: 0, n4: 0, n5: 1, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 1, n2: 1, n3: 1, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 3, n2: 1, n3: 1, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 5,
        Deck { ace: 4, n2: 0, n3: 0, n4: 1, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 1, n3: 0, n4: 1, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 0, n2: 0, n3: 0, n4: 0, n5: 0, n6: 1, n7: 1, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 5, n2: 0, n3: 1, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 3, n3: 0, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 0, n3: 2, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 3, n2: 2, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 1, n3: 0, n4: 0, n5: 1, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 4,
    },
    blackjack: btreemap! {},
    burst: btreemap! {
        Deck { ace: 4, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 3, n2: 0, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 0, n2: 1, n3: 0, n4: 0, n5: 1, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 2, n2: 3, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 0, n2: 0, n3: 1, n4: 0, n5: 1, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 3, n2: 2, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 0, n3: 0, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 2, n2: 0, n3: 2, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 3,
        Deck { ace: 0, n2: 0, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 0, n2: 0, n3: 0, n4: 2, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 0, n2: 0, n3: 1, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 2,
        Deck { ace: 0, n2: 4, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 4, n2: 2, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 4, n2: 0, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 0, n2: 2, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 1, n2: 2, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 7,
        Deck { ace: 0, n2: 0, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 4, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 1, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 3,
        Deck { ace: 1, n2: 1, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 3,
        Deck { ace: 3, n2: 1, n3: 1, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 5,
        Deck { ace: 1, n2: 0, n3: 0, n4: 0, n5: 0, n6: 1, n7: 1, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 2, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 3,
        Deck { ace: 0, n2: 1, n3: 2, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 3,
        Deck { ace: 1, n2: 0, n3: 1, n4: 1, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 1, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 0, n2: 2, n3: 0, n4: 1, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 0, n2: 1, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 4, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 2, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 5, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 3, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 0, n2: 2, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 3,
        Deck { ace: 4, n2: 0, n3: 0, n4: 1, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 5, n2: 0, n3: 1, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 0, n3: 2, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 2,
        Deck { ace: 1, n2: 0, n3: 0, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 2, n2: 0, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 0, n2: 3, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 4, n2: 2, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 3, n2: 0, n3: 0, n4: 0, n5: 1, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 2, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 7,
        Deck { ace: 2, n2: 0, n3: 2, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 0, n2: 1, n3: 0, n4: 0, n5: 0, n6: 2, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 1, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 1, face: 0 } => 2,
        Deck { ace: 0, n2: 1, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 2,
        Deck { ace: 2, n2: 0, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 0, n3: 1, n4: 1, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 3, n2: 2, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 0, n2: 1, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 1 } => 2,
        Deck { ace: 0, n2: 0, n3: 0, n4: 2, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 0, n2: 1, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 2,
        Deck { ace: 2, n2: 0, n3: 2, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 3,
        Deck { ace: 2, n2: 2, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 2, n2: 1, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 4,
        Deck { ace: 2, n2: 1, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 4,
        Deck { ace: 0, n2: 3, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 0, n3: 0, n4: 0, n5: 0, n6: 2, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 5, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 0, n2: 0, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 1, n9: 0, face: 0 } => 2,
        Deck { ace: 3, n2: 1, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 5,
        Deck { ace: 4, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 3, n2: 1, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 5,
        Deck { ace: 3, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 0, n2: 0, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 2, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 0, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 0, n2: 2, n3: 1, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 1, n2: 1, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 3,
        Deck { ace: 2, n2: 0, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 3, n2: 2, n3: 0, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 4, n2: 0, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 0, n3: 0, n4: 2, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 3, n2: 0, n3: 0, n4: 1, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 0, n3: 2, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 2, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 3, n2: 0, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 1, n2: 2, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 7,
        Deck { ace: 0, n2: 0, n3: 0, n4: 0, n5: 0, n6: 0, n7: 1, n8: 1, n9: 0, face: 0 } => 2,
        Deck { ace: 4, n2: 0, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 3, n2: 0, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 1, n2: 1, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 3,
        Deck { ace: 0, n2: 2, n3: 0, n4: 1, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 1, n2: 0, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 0, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 4, n2: 2, n3: 0, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 0, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 1 } => 1,
        Deck { ace: 0, n2: 1, n3: 2, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 1, n2: 3, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 2,
        Deck { ace: 4, n2: 2, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 0, n2: 4, n3: 0, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 1, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 1, n9: 0, face: 0 } => 2,
        Deck { ace: 2, n2: 1, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 4,
        Deck { ace: 1, n2: 3, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 2,
        Deck { ace: 2, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 0, n2: 0, n3: 0, n4: 2, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 5, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 0, n3: 0, n4: 2, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 0, n3: 2, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 1, n2: 1, n3: 0, n4: 0, n5: 1, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 0, n2: 0, n3: 1, n4: 0, n5: 1, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 2,
        Deck { ace: 3, n2: 0, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 3, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 3,
        Deck { ace: 1, n2: 2, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 2, n2: 2, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 0, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 2, n2: 1, n3: 0, n4: 1, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 0, n2: 4, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 1, n2: 1, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 3,
        Deck { ace: 1, n2: 0, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 0, n3: 2, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 1, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 2,
        Deck { ace: 3, n2: 0, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 1, n2: 0, n3: 0, n4: 0, n5: 0, n6: 0, n7: 2, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 0, n3: 2, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 1, n2: 0, n3: 0, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 0, n2: 1, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 0, n3: 1, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 2,
        Deck { ace: 1, n2: 2, n3: 1, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 7,
        Deck { ace: 1, n2: 0, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 1, n2: 0, n3: 1, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 4,
        Deck { ace: 0, n2: 0, n3: 1, n4: 0, n5: 1, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 2, n2: 0, n3: 0, n4: 0, n5: 0, n6: 1, n7: 1, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 0, n3: 2, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 3,
        Deck { ace: 2, n2: 1, n3: 0, n4: 1, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 0, n2: 0, n3: 1, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 2,
        Deck { ace: 1, n2: 0, n3: 1, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 4,
        Deck { ace: 0, n2: 0, n3: 2, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 0, n2: 1, n3: 2, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 3,
        Deck { ace: 0, n2: 1, n3: 0, n4: 0, n5: 0, n6: 1, n7: 1, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 1, n3: 2, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 3,
        Deck { ace: 1, n2: 1, n3: 0, n4: 0, n5: 1, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 1, n2: 1, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 3,
        Deck { ace: 4, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 3, n2: 0, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 3, n2: 2, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 0, n2: 0, n3: 0, n4: 0, n5: 0, n6: 0, n7: 2, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 4, n2: 0, n3: 0, n4: 1, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 1, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 4,
        Deck { ace: 0, n2: 1, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 2,
        Deck { ace: 2, n2: 0, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 1, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 4,
        Deck { ace: 0, n2: 0, n3: 1, n4: 1, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 2,
        Deck { ace: 1, n2: 0, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 2, n2: 3, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 3,
        Deck { ace: 2, n2: 0, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 0, n2: 0, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 1, n2: 3, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 2,
        Deck { ace: 0, n2: 2, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 3,
        Deck { ace: 2, n2: 1, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 4,
        Deck { ace: 1, n2: 0, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 0, n2: 0, n3: 1, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 2,
        Deck { ace: 2, n2: 1, n3: 1, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 4,
        Deck { ace: 1, n2: 1, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 3,
        Deck { ace: 0, n2: 2, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 3,
        Deck { ace: 2, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 2, n2: 0, n3: 2, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 3, n2: 1, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 5,
        Deck { ace: 2, n2: 0, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 0, n2: 1, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 2,
        Deck { ace: 0, n2: 2, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 3,
        Deck { ace: 0, n2: 0, n3: 1, n4: 1, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 0, n2: 1, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 2,
        Deck { ace: 0, n2: 1, n3: 2, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 2, n2: 0, n3: 0, n4: 0, n5: 1, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 0, n3: 0, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 0, n2: 3, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 3, n2: 0, n3: 0, n4: 0, n5: 1, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 3, n2: 1, n3: 1, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 5,
        Deck { ace: 1, n2: 0, n3: 2, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 2,
        Deck { ace: 0, n2: 4, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 2, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 3,
        Deck { ace: 2, n2: 0, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 1, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 3,
        Deck { ace: 1, n2: 1, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 3,
        Deck { ace: 1, n2: 1, n3: 0, n4: 1, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 1, n2: 0, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 3, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 0, n2: 0, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 1,
        Deck { ace: 2, n2: 0, n3: 0, n4: 1, n5: 0, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
        Deck { ace: 0, n2: 0, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 1, n9: 1, face: 0 } => 1,
        Deck { ace: 2, n2: 3, n3: 0, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 3,
        Deck { ace: 1, n2: 0, n3: 1, n4: 1, n5: 0, n6: 0, n7: 0, n8: 1, n9: 0, face: 0 } => 4,
        Deck { ace: 1, n2: 3, n3: 0, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 2,
        Deck { ace: 2, n2: 3, n3: 0, n4: 0, n5: 0, n6: 0, n7: 0, n8: 0, n9: 1, face: 0 } => 3,
        Deck { ace: 4, n2: 2, n3: 0, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 1, n2: 2, n3: 1, n4: 0, n5: 0, n6: 1, n7: 0, n8: 0, n9: 0, face: 0 } => 7,
        Deck { ace: 1, n2: 0, n3: 0, n4: 0, n5: 0, n6: 0, n7: 1, n8: 1, n9: 0, face: 0 } => 1,
        Deck { ace: 5, n2: 0, n3: 1, n4: 0, n5: 0, n6: 0, n7: 1, n8: 0, n9: 0, face: 0 } => 1,
        Deck { ace: 2, n2: 0, n3: 0, n4: 0, n5: 1, n6: 0, n7: 0, n8: 0, n9: 0, face: 1 } => 1,
    },
});
