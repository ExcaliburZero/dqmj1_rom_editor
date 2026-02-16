use std::collections::BTreeMap;

#[derive(Clone, Copy)]
pub enum Region {
    NorthAmerica,
    Japan,
    Europe,
}

impl Region {
    pub fn from_game_code(game_code: &str) -> Option<Region> {
        let game_code_to_region = BTreeMap::from([
            ("AJRE", Region::NorthAmerica),
            ("AJRJ", Region::Japan),
            ("AJRP", Region::Europe),
        ]);

        game_code_to_region.get(game_code).cloned()
    }
}
