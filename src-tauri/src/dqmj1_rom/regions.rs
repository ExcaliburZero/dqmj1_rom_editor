use std::collections::BTreeMap;

#[derive(Clone, Copy)]
pub enum Region {
    NorthAmerica,
    Japan,
    Europe,
}

impl Region {
    pub fn from_game_code(game_code: &str) -> Option<Region> {
        //let game_code_to_region = BTreeMap::from([
        //    ([0x41, 0x4A, 0x52, 0x45], Region::NorthAmerica), // AJRE
        //    ([0x41, 0x4A, 0x52, 0x4A], Region::Japan),        // AJRJ
        //    ([0x41, 0x4A, 0x52, 0x50], Region::Europe),       // AJRP
        //]);
        let game_code_to_region = BTreeMap::from([
            ("AJRE", Region::NorthAmerica),
            ("AJRJ", Region::Japan),
            ("AJRP", Region::Europe),
        ]);

        game_code_to_region.get(game_code).cloned()
    }
}
