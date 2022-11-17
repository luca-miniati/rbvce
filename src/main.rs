#![allow(dead_code)]

macro_rules! get_bit {
    ($bitboard:expr, $sq:expr) => {
        $bitboard & 1 << $sq > 0
    };
}

macro_rules! set_bit {
    ($bitboard:expr, $sq:expr) => {
        if !get_bit!($bitboard, $sq) {$bitboard ^= 1 << $sq};
    };
}

macro_rules! pop_bit {
    ($bitboard:expr, $sq:expr) => {
        $bitboard & !(1 << $sq)
    };
}

fn print_bitboard(bitboard: u128) {
    print!("\n");
    for rank in 0..11 {
        for file in 0..11 {
            let sq = rank * 11 + file;
            print!(" {}", if get_bit!(bitboard, sq) {1} else {0});
        }
        print!("\n");
    }
    print!("\n    BITBOARD: {}\n\n", bitboard);
}

fn print_position(position: [u128; 4]) {
    let mut position_str = String::new();
    position_str.push_str(&position[0].to_string());
    position_str.push(',');
    position_str.push_str(&position[1].to_string());
    position_str.push(',');
    position_str.push_str(&position[2].to_string());
    position_str.push(',');
    position_str.push_str(&position[3].to_string());
    println!("{position_str}");
}

fn get_moves(square: i8, occupancy: u128) -> u128 {
    let piece_rank:i8 = square as i8 / 11;
    let piece_file:i8 = square as i8 % 11;
    let mut moves: u128 = 0;

    let mut target_rank: i8 = piece_rank - 1;
    while target_rank >= 0 {
        if occupancy & 1 << target_rank*11 + piece_file > 0 {break};
        moves |= 1 << target_rank*11 + piece_file;
        target_rank -= 1;
    }
    target_rank = piece_rank + 1;
    while target_rank <= 10 {
        if occupancy & 1 << target_rank*11 + piece_file > 0 {break};
        moves |= 1 << target_rank*11 + piece_file;
        target_rank += 1;
    }
    let mut target_file: i8 = piece_file + 1;
    while target_file <= 10 {
        if occupancy & 1 << piece_rank*11 + target_file > 0 {break};
        moves |= 1 << piece_rank*11 + target_file;
        target_file += 1;
    }
    target_file = piece_file - 1;
    while target_file >= 0 {
        if occupancy & 1 << piece_rank*11 + target_file > 0 {break};
        moves |= 1 << piece_rank*11 + target_file;
        target_file -= 1;
    }
    
    return moves;
}

fn bit_count(mut bitboard: u128) -> u8 {
    let mut count = 0;
    while bitboard > 0 {
        bitboard &= bitboard - 1;
        count += 1;
    }
    count
}

fn get_eval(position: [u128; 4]) -> f32 {
    (1 + bit_count(position[1]) - bit_count(position[2])) as f32
}

#[inline(always)]
fn get_first_bit(bitboard: u128) -> u8 {
    let lz: u8 = bitboard.leading_zeros() as u8;
    let first_bit: u8 = 127 - lz;
    first_bit
}

fn get_positions(position: [u128; 4], white_to_move: bool) {
    let mut positions: Vec<[u128; 4]> = Vec::new();

    let color = if white_to_move {1} else {2};

    let num_pieces = bit_count(position[color]) + if white_to_move {1} else {0};
    // for each piece
    for _ in 0..num_pieces {
        let sq = get_first_bit(position[color]);
        let moves = get_moves(sq as i8, position[3]);
        let num_moves = bit_count(moves);
        // for each move
        for _ in 0..num_moves {
            let mv = get_first_bit(moves);
            let new_position = pop_bit!(position[color], sq) | 1 << mv;
            let mut new_new_position = position;
            new_new_position[color] = new_position;
            positions.push(new_new_position);
        }
    }
    // positions 
}

// fn minimax(position: [u128; 4], depth: i32, white_to_move: bool) -> f32 {
//     if depth == 0 {
//         return get_eval(position);
//     } else if white_to_move {
//         let max_eval = f32::MAX;

//         let eval = minimax(position, depth - 1, !white_to_move);
//     }
// }

fn main() {
    // position syntax:
    // [king, white, black, occupancy]
    let position: [u128; 4] = [
        1152921504606846976,
        4843975203994294389047296, 
        321942687793732272394500699397882104, 
        321942687798576248751416498393776376
    ];
    let corners: u128 = 1330526069999549579810939684362650625;
    let unmoveable_squares: u128 = 1998385887253891897645375523062287875;
    let moveable_squares: u128 = 338283981033684571565729231908705923580;
    // get_positions(position, true);


    print_position(position);
}
