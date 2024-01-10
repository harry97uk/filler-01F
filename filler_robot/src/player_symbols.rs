pub struct GameSymbols {
    pub my_recent_symbol: char,
    pub my_territory_symbol: char,
    pub opponent_recent_symbol: char,
    pub opponent_territory_symbol: char,
}

impl GameSymbols {
    pub fn new(player_number: usize) -> Self {
        if player_number == 1 {
            return GameSymbols {
                my_recent_symbol: 'a',
                my_territory_symbol: '@',
                opponent_recent_symbol: 's',
                opponent_territory_symbol: '$',
            };
        } else {
            return GameSymbols {
                my_recent_symbol: 's',
                my_territory_symbol: '$',
                opponent_recent_symbol: 'a',
                opponent_territory_symbol: '@',
            };
        }
    }
}
