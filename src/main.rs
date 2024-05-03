use crate::game::{Game, Location, Player};

mod game;

fn main() {
    let mut game = Game::new();

    game.place_move(Player::X, Location::TopLeft, Location::Middle);
    game.place_move(Player::O, Location::TopLeft, Location::TopRight);
    game.place_move(Player::X, Location::TopLeft, Location::BottomRight);
    game.place_move(Player::O, Location::TopLeft, Location::TopLeft);
    game.place_move(Player::X, Location::TopLeft, Location::Top);
    game.place_move(Player::O, Location::TopLeft, Location::BottomLeft);
    game.place_move(Player::X, Location::TopLeft, Location::Bottom);

    game.print();
}
