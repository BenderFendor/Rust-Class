use std::error::Error;
use std::fmt::Display;
use std::io;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameStatus {
    Win,
    Lose,
    Continue,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoardError {
    InvalidCharacter(char),
    InvalidSize,
    NoMinotaur,
    NoTheseus,
    NoGoal,
    MultipleMinotaur,
    MultipleTheseus,
    MultipleGoal,
}
impl Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardError::InvalidCharacter(c) => write!(f, "Invalid character: {}", c),
            BoardError::InvalidSize => write!(f, "Invalid size"),
            BoardError::NoMinotaur => write!(f, "No minotaur"),
            BoardError::NoTheseus => write!(f, "No theseus"),
            BoardError::NoGoal => write!(f, "No goal"),
            BoardError::MultipleMinotaur => write!(f, "Multiple minotaur"),
            BoardError::MultipleTheseus => write!(f, "Multiple theseus"),
            BoardError::MultipleGoal => write!(f, "Multiple goal"),
        }
    }
}
impl Error for BoardError {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GamePiece
{
    pub x: usize,
    pub y: usize,
}
impl GamePiece {
    pub fn move_to(&mut self, new_x: usize, new_y: usize) {
        self.x = new_x;
        self.y = new_y;
    }
}

#[derive(Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub walls: Vec<GamePiece>,
    pub minotaur: GamePiece,
    pub theseus: GamePiece,
    pub goal: GamePiece,
}
impl Grid {
}

#[derive(Clone)]
pub struct Game {
    pub grid: Grid,
}

// So I need to get the height and width of the maze so the number of rows and columns one then get the x t and m and get their index so like. Get the location of thesues and its x and y position get the location of minotaur and get its x and y postion then make a grid that has like empty space which is any postion that is not covered by the x's

// So I need an object for x that takes x and x has its position its x and y postions then you need
// to make an array of this x's

impl Game {
    pub fn from_board(board: &str) -> Result<Game, BoardError> {      
        let lines: Vec<&str> = board.lines().collect();
        let numofrows = lines.len();

        if numofrows == 0 {
            return Err(BoardError::InvalidSize)
        }
        let numofcolumns = lines[0].len();

        let mut wallstoadd = Vec::new();
        let mut theseus_pos: Option<GamePiece> = None;
        let mut minotaur_pos: Option<GamePiece> = None;
        let mut goal_pos: Option<GamePiece> = None;
        for y in 0..numofrows
        {
            for x in 0..numofcolumns
            {
                match lines[y].chars().nth(x) {
                    Some('X') => wallstoadd.push(GamePiece { x, y }),
                    Some('T') => {
                        if theseus_pos.is_some() {
                           return Err(BoardError::MultipleTheseus);
                        }
                        theseus_pos = Some(GamePiece { x, y });
                    }
                    Some('M') => {
                    if minotaur_pos.is_some() {
                        return Err(BoardError::MultipleMinotaur);
                    }
                        minotaur_pos = Some(GamePiece { x, y });
                    }
                    Some('G') => {
                        if goal_pos.is_some() {
                            return Err(BoardError::MultipleGoal);
                        }
                    goal_pos = Some(GamePiece{x,y})
                    }
                
                Some(' ') => {},
                Some(c) => return Err(BoardError::InvalidCharacter(c)), 
                None => return Err(BoardError::InvalidSize), 
                }     
            }
        }

        let grid = Grid {
            width: numofcolumns,
            height: numofrows,
            walls: wallstoadd, 
            theseus: theseus_pos.ok_or(BoardError::NoTheseus)?,
            minotaur: minotaur_pos.ok_or(BoardError::NoMinotaur)?, 
            goal: goal_pos.ok_or(BoardError::NoGoal)?,
        };

        Ok(Game {
            grid,
        })
    }

    pub fn show(&self) {
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                if self.is_theseus(y, x) {
                    print!("T");
                } else if self.is_minotaur(y, x) {
                    print!("M");
                } else if self.is_goal(y, x) {
                    print!("G");
                } else if self.is_wall(y, x) {
                    print!("X");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    pub fn minotaur_move(&mut self) {
        let tx = self.grid.theseus.x;
        let ty = self.grid.theseus.y;
        let mx = self.grid.minotaur.x;
        let my = self.grid.minotaur.y;
        let dist_x = (tx as isize - mx as isize).abs() as usize;
        let dist_y = (ty as isize - my as isize).abs() as usize;

        // Check left: if valid and decreases x-distance
        if mx > 0 && !self.is_wall(my, mx - 1) {
            let new_mx = mx - 1;
            let new_dist_x = (tx as isize - new_mx as isize).abs() as usize;
            if new_dist_x < dist_x {
                self.grid.minotaur.move_to(new_mx, my);
                return;
            }
        }

        // Check right: if valid and decreases x-distance
        if mx < self.grid.width - 1 && !self.is_wall(my, mx + 1) {
            let new_mx = mx + 1;
            let new_dist_x = (tx as isize - new_mx as isize).abs() as usize;
            if new_dist_x < dist_x {
                self.grid.minotaur.move_to(new_mx, my);
                return;
            }
        }

        // Check up: if valid and decreases y-distance
        if my > 0 && !self.is_wall(my - 1, mx) {
            let new_my = my - 1;
            let new_dist_y = (ty as isize - new_my as isize).abs() as usize;
            if new_dist_y < dist_y {
                self.grid.minotaur.move_to(mx, new_my);
                return;
            }
        }

        // Check down: if valid and decreases y-distance
        if my < self.grid.height - 1 && !self.is_wall(my + 1, mx) {
            let new_my = my + 1;
            let new_dist_y = (ty as isize - new_my as isize).abs() as usize;
            if new_dist_y < dist_y {
                self.grid.minotaur.move_to(mx, new_my);
            }
        }
    }
    pub fn theseus_move(&mut self, command: Command) {
        let current_pos = self.grid.theseus;
        let mut new_pos = current_pos;

        match command {
            Command::Up => {
                if new_pos.y > 0 {
                    new_pos.y -= 1;
                }
            }
            Command::Down => {
                if new_pos.y < self.grid.height - 1 {
                    new_pos.y += 1;
                }
            }
            Command::Left => {
                if new_pos.x > 0 {
                    new_pos.x -= 1;
                }
            }
            Command::Right => {
                if new_pos.x < self.grid.width - 1 {
                    new_pos.x += 1;
                }
            }
            Command::Skip => {
            }
        }

        // Only move Theseus if the new position is not a wall.
        if !self.is_wall(new_pos.y, new_pos.x) {
            self.grid.theseus.move_to(new_pos.x, new_pos.y);
        }
    }

    pub fn status(&self) -> GameStatus {
        if self.grid.theseus == self.grid.goal {
            GameStatus::Win
        } else if self.grid.theseus == self.grid.minotaur {
            GameStatus::Lose
        } else {
            GameStatus::Continue
        }
    }
}
impl Game {
    pub fn is_theseus(&self, row: usize, col: usize) -> bool {
        self.grid.theseus.y == row && self.grid.theseus.x == col
    }
    pub fn is_minotaur(&self, row: usize, col: usize) -> bool {
        self.grid.minotaur.y == row && self.grid.minotaur.x == col
    }
    pub fn is_wall(&self, row: usize, col: usize) -> bool {
        self.grid.walls.iter().any(|w| w.y == row && w.x == col)
    }
    pub fn is_goal(&self, row: usize, col: usize) -> bool {
        self.grid.goal.y == row && self.grid.goal.x == col
    }
    pub fn is_empty(&self, row: usize, col: usize) -> bool {
        !self.is_theseus(row, col)
            && !self.is_minotaur(row, col)
            && !self.is_goal(row, col)
            && !self.is_wall(row, col)
    }
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    /// Move one tile up
    Up,
    /// Move one tile down
    Down,
    /// Move one tile left
    Left,
    /// Move one tile right
    Right,
    /// Don't move at all
    Skip,
}

//  To get a command from the user, you can use the following code:
//  ```
//  let line = stdin.lines().next().unwrap().unwrap();
//  ```
//  This will read a line from the user and store it in the `buffer` string.
//  
//  Unfortunately, since stdin is line-buffered, everytime you enter a command while playing the
//  game you will have to press "enter" afterwards to send a new line.
//
//  While using the arrow keys to take inputs would be natural, it can be difficult to handle arrow
//  keys in a way that works on all devices. Therefore, it's recommended that you either use "w",
//  "a", "s", and "d" to take input, or else the words "up", "down", "left", "right". You can take
//  input however you like, so long as you document it here in a comment and it is reasonable to
//  use as a player.
pub fn input(mut stdin: impl io::Read + io::BufRead) -> Option<Command> {
    let mut buffer = String::new();
    if stdin.read_line(&mut buffer).is_err() {
        return None;
    }

    match buffer.trim() {
        "w" | "up" => Some(Command::Up),
        "s" | "down" => Some(Command::Down),
        "a" | "left" => Some(Command::Left),
        "d" | "right" => Some(Command::Right),
        " " | "skip" => Some(Command::Skip),
        _ => None,
    }
}
