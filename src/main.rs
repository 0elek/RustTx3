use rand::Rng;

#[derive(PartialEq, Copy, Clone, Debug)]
enum State {
    White,
    Black,
    Unoccupied,
}
#[derive(Clone, Copy)]
enum Winner {
    White,
    Black,
    Draw,
    InProgress,
}
#[derive(Clone, Copy)]
struct Board {
    board: [State; 9],
    turn: State,
    winner: Winner,
}

impl From<State> for Winner {
    fn from(state: State) -> Self {
        match state {
            State::White => Winner::White,
            State::Black => Winner::Black,
            State::Unoccupied => Winner::InProgress,
        }
    }
}

fn check_for_winner(board: Board) -> Winner {
    const WINNING_POSITIONS: [[u8; 3]; 8] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
        [1, 5, 9],
        [3, 5, 7],
        [1, 4, 7],
        [2, 5, 8],
        [3, 6, 9],
    ];

    let mut winner: Winner = Winner::InProgress;
    for ways in WINNING_POSITIONS {
        let mut count: i32 = 0;
        let mut played: i32 = 0;
        for way in ways {
            if board.board[(way - 1) as usize] == board.turn {
                count += 1;
            }
            if board.board[(way - 1) as usize] != State::Unoccupied {
                played += 1;
            }
        }
        if played == 9 {
            winner = Winner::Draw
        };
        if count == 3 {
            winner = Winner::from(board.turn);
        }
    }

    winner
}

fn turn(mut board: Board, step: usize) -> Option<Board> {
    if board.board[step] == State::Unoccupied {
        board.board[step] = board.turn;
        Some(board)
    } else {
        None
    }
}

fn print_board(board: Board) {
    fn x_or_o(state: State) -> String {
        match state {
            State::White => String::from("X"),
            State::Black => String::from("O"),
            State::Unoccupied => String::from("#"),
        }
    }
    println!(
        "{}|{}|{}   0 , 1 , 2",
        x_or_o(board.board[0]),
        x_or_o(board.board[1]),
        x_or_o(board.board[2])
    );
    println!(
        "{}|{}|{}   3 , 4 , 5",
        x_or_o(board.board[3]),
        x_or_o(board.board[4]),
        x_or_o(board.board[5])
    );
    println!(
        "{}|{}|{}   6 , 7 , 8",
        x_or_o(board.board[6]),
        x_or_o(board.board[7]),
        x_or_o(board.board[8])
    );
}

fn main() {
    let mut starter: State = State::Unoccupied;
    if starter == State::Unoccupied {
        let mut rng = rand::thread_rng();
        let random_number: i32 = rng.gen_range(0..2);
        if random_number == 0 {
            starter = State::White;
        } else {
            starter = State::Black;
        }
    }

    let mut game_board: Board = Board {
        board: [State::Unoccupied; 9],
        turn: starter,
        winner: Winner::InProgress,
    };

    loop {
        print_board(game_board);

        let mut step: usize = 0;
        let mut input: String = String::new();
        println!("Please enter your step");
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<usize>() {
                Ok(n) => {
                    step = n;
                    if step > 8 {
                        println!("Please enter a valid number");
                        continue;
                    }
                }
                Err(_) => println!("Please enter a valid number"),
            },
            Err(_) => println!("Please enter a valid number"),
        }

        let new_board = turn(game_board, step);
        match new_board {
            Some(mut board) => {
                board.winner = check_for_winner(board);
                match board.winner {
                    Winner::InProgress => {}
                    Winner::Draw => {
                        println!("Draw");
                        break;
                    }
                    Winner::White => {
                        println!("X wins");
                        break;
                    }
                    Winner::Black => {
                        println!("O wins");
                        break;
                    }
                }
                board.turn = match board.turn {
                    State::White => State::Black,
                    State::Black => State::White,
                    _ => State::Unoccupied,
                };
                game_board = board;
            }
            None => println!("Please enter a valid number"),
        }
    }
}
