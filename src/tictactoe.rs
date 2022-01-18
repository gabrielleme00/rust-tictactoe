const BOARD_N: usize = 3;
const BOARD_SIZE: usize = BOARD_N * BOARD_N;

pub struct Game {
    board: [u8; BOARD_SIZE],
    turn: usize,
    last_input: usize,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: [0; BOARD_SIZE],
            turn: 1,
            last_input: 255,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.print_board();
            self.make_move();
            if self.is_gameover() {
                break;
            }
            self.turn += 1;
        }
    }

    fn print_board(&self) {
        let mut i: usize = 0;

        println!("\n-----------");

        while i < BOARD_SIZE {
            // print position value
            match self.board[i] {
                0 => print!(" {} ", i),
                1 => print!(" X "),
                2 => print!(" O "),
                _ => break,
            }

            // print decorators
            if i > 0 && i < 8 && (i + 1) % 3 == 0 {
                println!("\n━━━╋━━━╋━━━");
            } else if i != 8 {
                print!("┃");
            }

            i += 1;
        }

        println!("\n-----------");
    }

    fn make_move(&mut self) {
        let input = self.get_input();
        self.board[input] = self.get_player();
        self.last_input = input;
    }

    fn get_input(&self) -> usize {
        println!("Player {}'s turn: ", self.get_player());

        loop {
            // read user input
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            // convert input to a valid number
            let input = match input.trim().parse::<usize>() {
                Ok(num) => num,
                Err(_) => continue,
            };

            // check if move is valid
            let is_valid_move = input > 8 || self.board[input] != 0;
            if is_valid_move {
                continue;
            }

            return input;
        }
    }

    fn get_player(&self) -> u8 {
        if self.turn % 2 == 1 {
            return 1;
        } else {
            return 2;
        };
    }

    fn is_gameover(&self) -> bool {
        if self.has_player_won() {
            self.print_board();
            println!("Player {} has won the game!", self.get_player());
            return true;
        } else if self.turn > BOARD_SIZE {
            self.print_board();
            println!("It's a draw!");
            return true;
        }

        false
    }

    fn has_player_won(&self) -> bool {
        let row = self.last_input / BOARD_N;
        let col = self.last_input % BOARD_N;
        let player = self.get_player();

        // check column
        println!("FOR COLUMN");
        for i in 0..BOARD_N {
            let element = self.board[i * BOARD_N + col];
            println!("  i({}) = {}", i, element);
            if element != player {
                println!("  K - {} = {}", element, player);
                break;
            }
            if i == BOARD_N - 1 { return true }
        }

        // check row
        for i in 0..BOARD_N {
            let element = self.board[row * BOARD_N + i];
            if element != player {
                break;
            }
            if i == BOARD_N - 1 { return true }
        }

        // check diagonal
        if row == col {
            for i in 0..BOARD_N {
                let element = self.board[i * BOARD_N + i];
                if element != player {
                    break;
                }
                if i == BOARD_N - 1 { return true }
            }
        }

        // check anti-diagonal
        if row + col == BOARD_N - 1 {
            for i in 0..BOARD_N {
                let element = self.board[((BOARD_N - 1) - i) * BOARD_N + i];
                if element != player {
                    break;
                }
                if i == BOARD_N - 1 { return true }
            }
        };

        false
    }
}
