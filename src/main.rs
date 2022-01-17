use std::io;

const BOARD_N: usize = 3;
const BOARD_SIZE: usize = BOARD_N * BOARD_N;

fn main() {
    let mut board: [u8; BOARD_SIZE] = [0; BOARD_SIZE];
    let mut turn: usize = 1;

    loop {
        print_board(&board);

        let player = get_turn_player(turn);
        println!("Player {}'s turn: ", player);

        let input = get_user_input();
        if is_move_valid(&board, input) { continue }

        // update the board and pass the turn
        board[input] = player;
        turn += 1;

        if has_game_ended(&board, input, player, turn) { break }
    }
}

fn is_move_valid(board: &[u8], input: usize) -> bool {
    if input > 8 || board[input] != 0 { return true }
    false
}

fn get_turn_player(turn: usize) -> u8 {
    if turn % 2 == 1 { return 1 } else { return 2 };
}

fn get_user_input() -> usize {
    loop {
        // read user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        // convert input to a valid number
        let input = match input.trim().parse::<usize>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return input;
    }
}

fn has_game_ended(board: &[u8], input: usize, player: u8, turn: usize) -> bool {
    if has_player_won(&board, input, player) {
        print_board(&board);
        println!("Player {} has won the game!", player);
        return true;
    } else if turn > BOARD_SIZE {
        print_board(&board);
        println!("It's a draw!");
        return true;
    };

    false
}

fn has_player_won(board: &[u8], input: usize, player: u8) -> bool {
    let row = input / BOARD_N;
    let col = input % BOARD_N;

    // check column
    for i in 0..BOARD_N {
        let element = board[i * BOARD_N + col];
        if element != player { break }
        if i == BOARD_N - 1 { return true }
    }

    // check row
    for i in 0..BOARD_N {
        let element = board[row * BOARD_N + i];
        if element != player { break }
        if i == BOARD_N - 1 { return true }
    }

    // check diagonal
    if row == col {
        for i in 0..BOARD_N {
            let element = board[i * BOARD_N + i];
            if element != player { break }
            if i == BOARD_N - 1 { return true }
        }
    }

    // check anti-diagonal
    if row + col == BOARD_N - 1 {
        for i in 0..BOARD_N {
            let element = board[((BOARD_N - 1) - i) * BOARD_N + i];
            if element != player { break }
            if i == BOARD_N - 1 { return true }
        }
    }

    false
}

fn print_board(board: &[u8]) {
    let mut i: usize = 0;

    println!("\n-----------");

    while i < BOARD_SIZE {
        // print position value
        match board[i] {
            0 => print!(" {} ", i),
            1 => print!(" X "),
            2 => print!(" O "),
            _ => break,
        }

        // print decorators
        if i > 0 && i < 8 && (i+1) % 3 == 0 {
            println!("\n━━━╋━━━╋━━━");
        } else if i != 8 {
            print!("┃");
        }

        i += 1;
    }

    println!("\n-----------");
}
