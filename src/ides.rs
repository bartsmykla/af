use phf::phf_ordered_map;
use std::collections::BTreeSet;

static LANGS_IDES_MAP: phf::ordered_map::OrderedMap<&str, &str> = phf_ordered_map! {
    "c" => "clion",
    "c++" => "clion",
    "go" => "goland",
    "javascript" => "webstorm",
    "ruby" => "rubymine",
    "rust" => "rustrover",
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
    assert_eq!(list(), vec!["clion", "goland", "rubymine", "rustrover", "webstorm"]);
}