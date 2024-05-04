#![allow(
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::too_many_arguments,
    dead_code
)]

use std::io;
use crate::game::{Game, Location, Player};

mod game;

fn main() {
    let mut game = Game::new();
    let mut player = Player::X;
    let mut board: Option<Location> = None;

    while game.winner().is_none() {
        println!();
        game.print();
        println!("\n{}'s turn\n", player.to_string());

        if board.is_none() {
            println!("Enter the board location:");
            let mut input_text = String::new();
            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");

            board = Location::from_input(input_text.trim().to_lowercase().as_str());
            if board.is_none() {
                println!("\nInvalid board location");
                continue;
            }
        }

        println!("Enter the {} cell location:", board.unwrap().to_string());
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let cell = Location::from_input(input_text.trim().to_lowercase().as_str());
        if cell.is_none() {
            println!("\nInvalid cell location");
            continue;
        }

        if game.place_move(player, board.unwrap(), cell.unwrap()) {
            player = player.opponent();
        } else {
            println!("\nInvalid move");
            continue;
        }

        if game.board_is_won(cell.unwrap()) {
            board = None;
        } else {
            board = Some(cell.unwrap());
        }
    }

    println!("\n{} wins!\n", game.winner().unwrap().to_string());

    game.print();
}
