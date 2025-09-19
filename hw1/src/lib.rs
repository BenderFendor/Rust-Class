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

#[derive(Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub walls: Vec<GamePiece>,
    pub minotaur: GamePiece,
    pub theseus: GamePiece,

    // TODO: Implement the Grid struct
}
impl Grid {
    fn AddWall(&mut self,WallPiece: GamePiece)
    {
        self.walls.push(WallPiece);
    }
 }

#[derive(Clone)]
pub struct Game(&self) {
    grid: Grid,
    // TODO: Implement the Game struct
}

// So I need to get the height and width of the maze so the number of rows and columns one then get the x t and m and get their index so like. Get the location of thesues and its x and y position get the location of minotaur and get its x and y postion then make a grid that has like empty space which is any postion that is not covered by the x's

// So I need an object for x that takes x and x has its position its x and y postions then you need
// to make an array of this x's

impl Game {
    // TODO: replace the function body with your implementation
    pub fn from_board(board: &str) -> Result<Game, BoardError> {
        
        println!("{}",board);
      
        let lines: Vec<&str> = boards.lines().collect()
        let numofrows = lens.len();

        if numofrows == 0 {
            return Err(BoardError::InvalidSize)
        }
        let numofcolumns = lines[0].len();

        let mut walls = Vec::new()
        let mut theseus_pos: Option<GamePiece> = None;
        let mut minotaur_pos: Option<GamePiece>= None;

        for y in 0..numofrows
        {
            for x in 0..numofcolumns
            {
                let character = lines.[y].chars().nth(x); // I may change this to a match statement
                if character == 'X'
                {
                    walls.push(GamePiece {x,y});
                }
                else if character == 'T' {
                    thesus_pos = Some(GamePiece {x,y});
                }
                else if character == 'T' {
                    minotaur_pos = Some(GamePiece{x, });
                }
                else if character == ' ' {
                    
                }
                else
                {
                    other => return Err(BoardError::InvalidCharacter(other));
                }
            }
        }

        let grid = Grid {
            width: numofcolumns,
            height: numofrows,
            walls: walls,
            thesus: thesus_pos,
            minotaur: minotaur_pos,
        }

        return Ok(Game {
            grid: Grid {},
        });
    }

    // TODO
    pub fn show(&self) {
    }

    // TODO
    pub fn minotaur_move(&mut self) {
    }

    // TODO
    pub fn theseus_move(&mut self, command: Command) {
    }

    // TODO: replace the function body with your implementation
    pub fn status(&self) -> GameStatus {
        GameStatus::Continue
    }
}

impl Game {
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is Theseus
    pub fn is_theseus(&self, row: usize, col: usize) -> bool {
        false
    }
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is Minotaur
    pub fn is_minotaur(&self, row: usize, col: usize) -> bool {
        false
    }
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is a wall
    pub fn is_wall(&self, row: usize, col: usize) -> bool {
        false
    }
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is the goal
    pub fn is_goal(&self, row: usize, col: usize) -> bool {
        false
    }
    // TODO: replace the function body with your implementation
    /// Returns true if the given position is empty
    pub fn is_empty(&self, row: usize, col: usize) -> bool {
        false
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
pub fn input(stdin: impl io::Read + io::BufRead) -> Option<Command> {
    // TODO: replace this loop with your code
    loop {}
}
