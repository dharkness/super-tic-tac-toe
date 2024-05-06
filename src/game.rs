use std::str::FromStr;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Mark {
    X,
    O,
}

impl Mark {
    pub fn to_str(self) -> &'static str {
        match self {
            Mark::X => "X",
            Mark::O => "O",
        }
    }

    pub fn opponent(self) -> Mark {
        match self {
            Mark::X => Mark::O,
            Mark::O => Mark::X,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Coord {
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

impl Coord {
    pub fn from_index(index: usize) -> Option<Coord> {
        match index {
            0 => Some(Coord::TopLeft),
            1 => Some(Coord::Top),
            2 => Some(Coord::TopRight),
            3 => Some(Coord::Left),
            4 => Some(Coord::Middle),
            5 => Some(Coord::Right),
            6 => Some(Coord::BottomLeft),
            7 => Some(Coord::Bottom),
            8 => Some(Coord::BottomRight),
            _ => None,
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            Coord::TopLeft => "top-left",
            Coord::Top => "top",
            Coord::TopRight => "top-right",
            Coord::Left => "left",
            Coord::Middle => "middle",
            Coord::Right => "right",
            Coord::BottomLeft => "bottom-left",
            Coord::Bottom => "bottom",
            Coord::BottomRight => "bottom-right",
        }
    }
}

impl FromStr for Coord {
    type Err = ();

    fn from_str(input: &str) -> Result<Coord, ()> {
        match input {
            "tl" => Ok(Coord::TopLeft),
            "t" => Ok(Coord::Top),
            "tr" => Ok(Coord::TopRight),
            "l" => Ok(Coord::Left),
            "m" => Ok(Coord::Middle),
            "r" => Ok(Coord::Right),
            "bl" => Ok(Coord::BottomLeft),
            "b" => Ok(Coord::Bottom),
            "br" => Ok(Coord::BottomRight),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
pub struct Cell(Option<Mark>);

impl Cell {
    pub fn mark(&self) -> Option<Mark> {
        self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn place_move(&mut self, mark: Mark) -> bool {
        if self.is_empty() {
            self.0 = Some(mark);
            true
        } else {
            false
        }
    }

    pub fn to_str(self) -> &'static str{
        match self.0 {
            None => " ",
            Some(mark) => mark.to_str(),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Board {
    boards: [InnerBoard; 9],
    winner: Option<Mark>,
}

impl Board {
    pub fn new() -> Board {
        Board { boards: [InnerBoard::new(); 9], winner: None }
    }

    pub fn place_move(&mut self, mark: Mark, board: Coord, cell: Coord) -> bool {
        if self.boards[board as usize].place_move(mark, cell) {
            if self.boards[board as usize].winner().is_some() {
                self.winner = self.find_winner();
            }
            true
        } else {
            false
        }
    }

    pub fn winner(&self) -> Option<Mark> {
        self.winner
    }

    pub fn find_winner(&self) -> Option<Mark> {
        let winning_combinations = [
            [Coord::TopLeft, Coord::Top, Coord::TopRight],
            [Coord::Left, Coord::Middle, Coord::Right],
            [Coord::BottomLeft, Coord::Bottom, Coord::BottomRight],
            [Coord::TopLeft, Coord::Left, Coord::BottomLeft],
            [Coord::Top, Coord::Middle, Coord::Bottom],
            [Coord::TopRight, Coord::Right, Coord::BottomRight],
            [Coord::TopLeft, Coord::Middle, Coord::BottomRight],
            [Coord::TopRight, Coord::Middle, Coord::BottomLeft],
        ];

        for combination in winning_combinations.iter() {
            let winners = [
                self.boards[combination[0] as usize].winner(),
                self.boards[combination[1] as usize].winner(),
                self.boards[combination[2] as usize].winner(),
            ];

            if winners[0].is_some() && winners[0] == winners[1] && winners[1] == winners[2] {
                return winners[0];
            }
        }

        None
    }

    pub fn board_is_won(&self, board: Coord) -> bool {
        self.boards[board as usize].winner().is_some()
    }

    pub fn lines(&self) -> [String; 23] {
        let boards = self.boards.iter().map(|board| board.lines()).collect::<Vec<_>>();
        [
            "       ┃       ┃       ".to_string(),
            format!(" {} ┃ {} ┃ {} ", boards[0][0], boards[1][0], boards[2][0]),
            format!(" {} ┃ {} ┃ {} ", boards[0][1], boards[1][1], boards[2][1]),
            format!(" {} ┃ {} ┃ {} ", boards[0][2], boards[1][2], boards[2][2]),
            format!(" {} ┃ {} ┃ {} ", boards[0][3], boards[1][3], boards[2][3]),
            format!(" {} ┃ {} ┃ {} ", boards[0][4], boards[1][4], boards[2][4]),
            "       ┃       ┃       ".to_string(),
            "━━━━━━━╋━━━━━━━╋━━━━━━━".to_string(),
            "       ┃       ┃       ".to_string(),
            format!(" {} ┃ {} ┃ {} ", boards[3][0], boards[4][0], boards[5][0]),
            format!(" {} ┃ {} ┃ {} ", boards[3][1], boards[4][1], boards[5][1]),
            format!(" {} ┃ {} ┃ {} ", boards[3][2], boards[4][2], boards[5][2]),
            format!(" {} ┃ {} ┃ {} ", boards[3][3], boards[4][3], boards[5][3]),
            format!(" {} ┃ {} ┃ {} ", boards[3][4], boards[4][4], boards[5][4]),
            "       ┃       ┃       ".to_string(),
            "━━━━━━━╋━━━━━━━╋━━━━━━━".to_string(),
            "       ┃       ┃       ".to_string(),
            format!(" {} ┃ {} ┃ {} ", boards[6][0], boards[7][0], boards[8][0]),
            format!(" {} ┃ {} ┃ {} ", boards[6][1], boards[7][1], boards[8][1]),
            format!(" {} ┃ {} ┃ {} ", boards[6][2], boards[7][2], boards[8][2]),
            format!(" {} ┃ {} ┃ {} ", boards[6][3], boards[7][3], boards[8][3]),
            format!(" {} ┃ {} ┃ {} ", boards[6][4], boards[7][4], boards[8][4]),
            "       ┃       ┃       ".to_string(),
        ]
    }

    pub fn print(&self) {
        for line in self.lines().iter() {
            println!("{}", line);
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct InnerBoard {
    cells: [Cell; 9],
    winner: Option<Mark>,
}

impl InnerBoard {
    pub fn new() -> InnerBoard {
        InnerBoard { cells: [Cell::default(); 9], winner: None }
    }

    pub fn is_empty(&self, cell: Coord) -> bool {
        self.cells[cell as usize].is_empty()
    }

    pub fn place_move(&mut self, mark: Mark, cell: Coord) -> bool {
        if self.winner.is_none() && self.cells[cell as usize].place_move(mark) {
            self.winner = self.find_winner();
            true
        } else {
            false
        }
    }

    pub fn winner(&self) -> Option<Mark> {
        self.winner
    }

    pub fn find_winner(&self) -> Option<Mark> {
        let winning_combinations = [
            [Coord::TopLeft, Coord::Top, Coord::TopRight],
            [Coord::Left, Coord::Middle, Coord::Right],
            [Coord::BottomLeft, Coord::Bottom, Coord::BottomRight],
            [Coord::TopLeft, Coord::Left, Coord::BottomLeft],
            [Coord::Top, Coord::Middle, Coord::Bottom],
            [Coord::TopRight, Coord::Right, Coord::BottomRight],
            [Coord::TopLeft, Coord::Middle, Coord::BottomRight],
            [Coord::TopRight, Coord::Middle, Coord::BottomLeft],
        ];

        for combination in winning_combinations.iter() {
            let cells = [
                self.cells[combination[0] as usize],
                self.cells[combination[1] as usize],
                self.cells[combination[2] as usize],
            ];

            if !cells[0].is_empty() && cells[0] == cells[1] && cells[1] == cells[2] {
                return cells[0].mark();
            }
        }

        None
    }

    pub fn lines(&self) -> [String; 5] {
        match self.winner {
            None => [
                format!("{}│{}│{}", self.cells[0].to_str(), self.cells[1].to_str(), self.cells[2].to_str()),
                "─┼─┼─".to_string(),
                format!("{}│{}│{}", self.cells[3].to_str(), self.cells[4].to_str(), self.cells[5].to_str()),
                "─┼─┼─".to_string(),
                format!("{}│{}│{}", self.cells[6].to_str(), self.cells[7].to_str(), self.cells[8].to_str()),
            ],
            Some(Mark::X) => [
                "X   X".to_string(),
                " X X ".to_string(),
                "  X  ".to_string(),
                " X X ".to_string(),
                "X   X".to_string(),
            ],
            Some(Mark::O) => [
                " OOO ".to_string(),
                "O   O".to_string(),
                "O   O".to_string(),
                "O   O".to_string(),
                " OOO ".to_string(),
            ],
        }
    }
}
