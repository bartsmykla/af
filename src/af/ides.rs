use crate::consts::*;
use phf::ordered_map::OrderedMap;
use phf::phf_ordered_map;
use std::collections::BTreeSet;

static LANGS_IDES_MAP: OrderedMap<&str, &str> = phf_ordered_map! {
    "c" => CLION,
    "c++" => CLION,
    "go" => GOLAND,
    "javascript" => WEBSTORM,
    "ruby" => RUBYMINE,
    "rust" => RUSTROVER,
};

pub fn get(language: &str) -> Option<&'static str> {
    LANGS_IDES_MAP.get(language).copied()
}

pub fn list() -> Vec<&'static str> {
    let ides = LANGS_IDES_MAP.values().copied();
    let unique = BTreeSet::from_iter(ides);
    unique.into_iter().collect()
}

#[test]
fn test_values() {
    assert_eq!(list(), vec![CLION, GOLAND, RUBYMINE, RUSTROVER, WEBSTORM]);
}
