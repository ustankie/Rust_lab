use std::{fmt::Display, io};

const EMPTY_FIELD: char = ' ';
const PLAYER_ONE: char = 'o';
const PLAYER_TWO: char = 'x';

fn main() {
    let mut user_input = String::new();

    let players = [PLAYER_ONE, PLAYER_TWO];
    let mut board = [[EMPTY_FIELD; 3]; 3];

    let mut current_player_num = 0;
    let mut current_player = players[current_player_num];
    let mut success = false;
    let mut draw = 1;

    print_board(board);

    while !is_winning(board, current_player) && draw != board.len() * board.len() {
        if success {
            draw += 1;
            current_player_num += 1;
            current_player = players[current_player_num % players.len()];
        }

        println!("\n Currently moving: {}\nType your move:", current_player);
        user_input.clear();
        let _ = io::stdin().read_line(&mut user_input);
        let mut numbers = user_input.trim().split_whitespace();

        let x: i32 = numbers.next().unwrap_or("-1").parse().expect("x");
        let y: i32 = numbers.next().unwrap_or("-1").parse().expect("y");

        (board, success) = make_move(board, current_player, (x, y));
        print_board(board);
    }

    if is_winning(board, current_player) {
        println!("{} wins!", current_player);
    } else {
        println!("It's a draw!");
    }
}

fn print_board(board: [[char; 3]; 3]) {
    println!();
    print!("  ");
    for i in 0..board[0].len() {
        print!(" {}  ", i);
    }
    println!();

    let mut i = 0;
    for row in board {
        print!("{}", i);
        for cell in row {
            print!("| {} ", cell);
        }
        println!("|");
        i += 1;
    }
    println!();
}

fn is_winning(board: [[char; 3]; 3], current_player: char) -> bool {
    let mut win_counter_horizontal = 0;
    let mut win_counter_vertical = 0;
    let mut win_counter_diagonal = 0;
    for i in 0..board.len() {
        win_counter_horizontal = 0;
        win_counter_vertical = 0;
        for j in 0..board[0].len() {
            let field = board[i][j];
            if field == current_player {
                win_counter_horizontal = win_counter_horizontal + 1;
            }
            let field = board[j][i];
            if field == current_player {
                win_counter_vertical = win_counter_vertical + 1;
            }
            if i == j && field == current_player {
                win_counter_diagonal += 1;
            }
        }
        if win_counter_horizontal == board.len()
            || win_counter_vertical == board.len()
            || win_counter_diagonal == board.len()
        {
            return true;
        }
    }

    false
}

fn make_move(
    mut board: [[char; 3]; 3],
    current_player: char,
    position: (i32, i32),
) -> ([[char; 3]; 3], bool) {
    if position.0 < 0
        || position.0 >= board.len() as i32
        || position.1 < 0
        || position.1 >= board[0].len() as i32
    {
        println!();
        println!("Invalid move!");
        return (board, false);
    }

    let x = position.0 as usize;
    let y = position.1 as usize;

    if board[x][y] != EMPTY_FIELD {
        println!();
        println!("The field occupied!");
        return (board, false);
    }
    board[x][y] = current_player;
    (board, true)
}
