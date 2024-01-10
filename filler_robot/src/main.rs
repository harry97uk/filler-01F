mod player_symbols;

use std::{ io::{ self, Read, BufRead }, collections::VecDeque };

use player_symbols::GameSymbols;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap());

    // Read player information
    let player_info = lines.next().unwrap();
    let player_number: usize = player_info
        .split_whitespace()
        .nth(3)
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as usize;

    // Define player symbols
    let symbols = GameSymbols::new(player_number);

    // Process game turns
    loop {
        // Read Anfield and piece information
        let anfield_size_info = lines.next().unwrap();
        let vertical_anfield_size = anfield_size_info
            .split_whitespace()
            .nth(2)
            .unwrap()
            .parse()
            .unwrap();
        let mut anfield: VecDeque<String> = (0..=vertical_anfield_size)
            .map(|_| lines.next().unwrap())
            .collect();

        //remove x axis number labels
        anfield.pop_front();

        let piece_info = lines.next().unwrap();
        //let piece_x_size: usize = piece_info.split_whitespace().nth(1).unwrap().parse().unwrap();
        let piece_y_size: usize = piece_info.split_whitespace().nth(2).unwrap().parse().unwrap();

        let piece: VecDeque<String> = (0..piece_y_size).map(|_| lines.next().unwrap()).collect();

        // Your game logic goes here to determine the next move
        let (next_x, next_y) = find_next_move(&anfield, &piece, &symbols);

        // Output the move
        println!("{} {}", next_x, next_y);
    }
}

// Example logic to find the next move
fn find_next_move(
    anfield: &VecDeque<String>,
    piece: &VecDeque<String>,
    symbols: &GameSymbols
) -> (usize, usize) {
    // Your logic to determine the next move based on the Anfield goes here
    // This is a simple example that always places the piece in the center
    for y in 0..anfield.len() {
        for x in 0..anfield[0].len() {
            // Check if the piece can be placed at (x, y)
            if can_place_piece(&anfield, &piece, x, y, &symbols) {
                return (x, y);
            }
        }
    }

    let center_x = anfield[0].len() / 2;
    let center_y = anfield.len() / 2;

    (center_x, center_y)
}

// Function to check if the piece can be placed at a specific position
fn can_place_piece(
    anfield: &VecDeque<String>,
    piece: &VecDeque<String>,
    x: usize,
    y: usize,
    symbols: &GameSymbols
) -> bool {
    // Check if the piece can be placed at (x, y) without overlapping opponent territory
    for (i, row) in piece.iter().enumerate() {
        for (j, piece_cell) in row.chars().enumerate() {
            let anfield_cell = anfield.get(y + i).and_then(|r| r.chars().nth(x + j));
            if piece_cell == '0' {
                // Check if the piece cell is on a valid position (within bounds, not overlapping opponent territory)
                if let Some(a) = anfield_cell {
                    if
                        a != symbols.opponent_recent_symbol &&
                        a != symbols.opponent_territory_symbol
                    {
                        continue;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
    }

    true
}
