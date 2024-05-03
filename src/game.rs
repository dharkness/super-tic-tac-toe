#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Player {
    X,
    O,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Location {
    TopLeft,
    Top,
    TopRight,
    Left,
    Middle,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
pub struct Cell(Option<Player>);

impl Cell {
    pub fn player(&self) -> Option<Player> {
        self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn place_move(&mut self, player: Player) -> bool {
        if self.is_empty() {
            self.0 = Some(player);
            true
        } else {
            false
        }
    }

    pub fn label(&self) -> char {
        match self.0 {
            None => ' ',
            Some(Player::X) => 'X',
            Some(Player::O) => 'O',
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Game {
    boards: [Board; 9],
}

impl Game {
    pub fn new() -> Game {
        Game { boards: [Board::new(); 9] }
    }

    pub fn place_move(&mut self, player: Player, board: Location, cell: Location) {
        self.boards[board as usize].place_move(player, cell);
    }

    pub fn lines(&self) -> [String; 23] {
        let boards = self.boards.iter().map(|board| board.lines()).collect::<Vec<_>>();
        [
            "       ║       ║       ".to_string(),
            format!(" {} ║ {} ║ {} ", boards[0][0], boards[1][0], boards[2][0]),
            format!(" {} ║ {} ║ {} ", boards[0][1], boards[1][1], boards[2][1]),
            format!(" {} ║ {} ║ {} ", boards[0][2], boards[1][2], boards[2][2]),
            format!(" {} ║ {} ║ {} ", boards[0][3], boards[1][3], boards[2][3]),
            format!(" {} ║ {} ║ {} ", boards[0][4], boards[1][4], boards[2][4]),
            "       ║       ║       ".to_string(),
            "═══════╬═══════╬═══════".to_string(),
            "       ║       ║       ".to_string(),
            format!(" {} ║ {} ║ {} ", boards[3][0], boards[4][0], boards[5][0]),
            format!(" {} ║ {} ║ {} ", boards[3][1], boards[4][1], boards[5][1]),
            format!(" {} ║ {} ║ {} ", boards[3][2], boards[4][2], boards[5][2]),
            format!(" {} ║ {} ║ {} ", boards[3][3], boards[4][3], boards[5][3]),
            format!(" {} ║ {} ║ {} ", boards[3][4], boards[4][4], boards[5][4]),
            "       ║       ║       ".to_string(),
            "═══════╬═══════╬═══════".to_string(),
            "       ║       ║       ".to_string(),
            format!(" {} ║ {} ║ {} ", boards[6][0], boards[7][0], boards[8][0]),
            format!(" {} ║ {} ║ {} ", boards[6][1], boards[7][1], boards[8][1]),
            format!(" {} ║ {} ║ {} ", boards[6][2], boards[7][2], boards[8][2]),
            format!(" {} ║ {} ║ {} ", boards[6][3], boards[7][3], boards[8][3]),
            format!(" {} ║ {} ║ {} ", boards[6][4], boards[7][4], boards[8][4]),
            "       ║       ║       ".to_string(),
        ]
    }

    pub fn print(&self) {
        for line in self.lines().iter() {
            println!("{}", line);
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Board {
    cells: [Cell; 9],
    winner: Option<Player>,
}

impl Board {
    pub fn new() -> Board {
        Board { cells: [Cell::default(); 9], winner: None }
    }

    pub fn is_empty(&self, cell: Location) -> bool {
        self.cells[cell as usize].is_empty()
    }

    pub fn place_move(&mut self, player: Player, cell: Location) -> bool {
        if self.winner.is_none() && self.cells[cell as usize].place_move(player) {
            self.winner = self.find_winner();
            true
        } else {
            false
        }
    }

    pub fn find_winner(&self) -> Option<Player> {
        let winning_combinations = [
            [Location::TopLeft, Location::Top, Location::TopRight],
            [Location::Left, Location::Middle, Location::Right],
            [Location::BottomLeft, Location::Bottom, Location::BottomRight],
            [Location::TopLeft, Location::Left, Location::BottomLeft],
            [Location::Top, Location::Middle, Location::Bottom],
            [Location::TopRight, Location::Right, Location::BottomRight],
            [Location::TopLeft, Location::Middle, Location::BottomRight],
            [Location::TopRight, Location::Middle, Location::BottomLeft],
        ];

        for combination in winning_combinations.iter() {
            let cells = [
                self.cells[combination[0] as usize],
                self.cells[combination[1] as usize],
                self.cells[combination[2] as usize],
            ];

            if !cells[0].is_empty() && cells[0] == cells[1] && cells[1] == cells[2] {
                return cells[0].player();
            }
        }

        None
    }

    pub fn lines(&self) -> [String; 5] {
        match self.winner {
            None => [
                format!("{}│{}│{}", self.cells[0].label(), self.cells[1].label(), self.cells[2].label()),
                "─┼─┼─".to_string(),
                format!("{}│{}│{}", self.cells[3].label(), self.cells[4].label(), self.cells[5].label()),
                "─┼─┼─".to_string(),
                format!("{}│{}│{}", self.cells[6].label(), self.cells[7].label(), self.cells[8].label()),
            ],
            Some(Player::X) => [
                "X   X".to_string(),
                " X X ".to_string(),
                "  X  ".to_string(),
                " X X ".to_string(),
                "X   X".to_string(),
            ],
            Some(Player::O) => [
                " OOO ".to_string(),
                "O   O".to_string(),
                "O   O".to_string(),
                "O   O".to_string(),
                " OOO ".to_string(),
            ],
        }
    }

    pub fn print(&self) {
        println!("{}|{}|{}", self.cells[0].label(), self.cells[1].label(), self.cells[2].label());
        println!("-+-+-");
        println!("{}|{}|{}", self.cells[3].label(), self.cells[4].label(), self.cells[5].label());
        println!("-+-+-");
        println!("{}|{}|{}", self.cells[6].label(), self.cells[7].label(), self.cells[8].label());
    }
}
