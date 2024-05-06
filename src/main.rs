#![allow(
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::too_many_arguments,
    dead_code
)]

use std::io;
use std::str::FromStr;

use crate::game::{Board, Coord, Mark};

mod game;

fn main() {
    let mut game = Board::new();
    let mut mark = Mark::X;
    let mut outer: Option<Coord> = None;

    while game.winner().is_none() {
        println!();
        game.print();
        println!("\n{}'s turn\n", mark.to_str());

        if outer.is_none() {
            println!("Enter the board location:");
            let mut input_text = String::new();
            io::stdin()
                .read_line(&mut input_text)
                .expect("failed to read from stdin");

            outer = Coord::from_str(input_text.trim().to_lowercase().as_str()).ok();
            if outer.is_none() {
                println!("\nInvalid board location");
                continue;
            }
        }

        println!("Enter the {} cell location:", outer.unwrap().to_str());
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let inner = Coord::from_str(input_text.trim().to_lowercase().as_str()).ok();
        if inner.is_none() {
            println!("\nInvalid cell location");
            continue;
        }

        if game.place_move(mark, outer.unwrap(), inner.unwrap()) {
            mark = mark.opponent();
        } else {
            println!("\nInvalid move");
            continue;
        }

        if game.board_is_won(inner.unwrap()) {
            outer = None;
        } else {
            outer = Some(inner.unwrap());
        }
    }

    println!("\n{} wins!\n", game.winner().unwrap().to_str());

    game.print();
}
