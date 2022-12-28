use std::io;
use std::io::Write;

/// Holds the two possible values for the player.
enum Player {
    X,
    O,
}

impl Player {
    /// Display the character ("X" or "O") associated with the player.
    /// TODO: There is probably a better way of doing this?
    fn to_string(&self) -> &'static str {
        match self {
            Player::X => "X",
            Player::O => "O"
        }
    }
}

/// Run the Tic-Tac-Toe Game!
fn main() {
    clear_console();

    let mut map = [["-"; 3]; 3];
    let mut current_player: Player = Player::X;
    let mut game_over = false;

    while !game_over {
        println!("\tRUST TIC-TAC-TOE! Current Player: {}\n", current_player.to_string());

        print_field(&map);

        print!("\tPlease enter the X coordinate: ");
        io::stdout().flush().unwrap();
        let x_coordinate = get_coordinate_input_from_user();

        print!("\tPlease enter the Y coordinate: ");
        io::stdout().flush().unwrap();
        let y_coordinate = get_coordinate_input_from_user();

        if map[y_coordinate][x_coordinate] != "-" {
            clear_console();
            io::stdout().flush().unwrap();

            println!("\tThis field is already taken! Please choose another one!\n");
            continue;
        }

        map[y_coordinate][x_coordinate] = current_player.to_string();

        if game_is_over(&map, &current_player) {
            game_over = true;
        } else {
            current_player = match current_player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
            clear_console();
        }
    }

    clear_console();
    println!("\tRUST TIC-TAC-TOE! GAME OVER!\n");
    print_field(&map);
    println!("\tYou won! Congratulations Player {}!", current_player.to_string());
}

/// Clears the console with the unicode sign for clearing the console.
/// Taken from: https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed
fn clear_console() {
    print!("{}[2J", 27 as char);
}

/// Check if the current player has won the game by having three pieces either
/// horizontally, vertically or diagonally linked.
fn game_is_over(map: &[[&str; 3]; 3], player: &Player) -> bool {
    // Horizontal wins
    for x in 0..3 {
        let mut counter = 0;
        for y in 0..3 {
            if map[x][y] == player.to_string() {
                counter += 1;
            }
        }
        if counter == 3 {
            return true;
        }
    }

    // Vertical wins
    for y in 0..3 {
        let mut counter = 0;
        for x in 0..3 {
            if map[x][y] == player.to_string() {
                counter += 1;
            }
        }
        if counter == 3 {
            return true;
        }
    }

    // Diagonal wins
    if map[0][0] == player.to_string() && map[1][1] == player.to_string() && map[2][2] == player.to_string() {
        return true;
    }

    if map[0][2] == player.to_string() && map[1][1] == player.to_string() && map[2][0] == player.to_string() {
        return true;
    }

    return false;
}

/// Get user input and ensure that it is a number and is one of the values 0, 1 or 2.
fn get_coordinate_input_from_user() -> usize {
    let mut coordinate = String::new();

    io::stdin()
        .read_line(&mut coordinate)
        .expect("Failed to read line");

    let coordinate = match coordinate.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            print!("\tPlease enter a number between 0 and 2: ");
            io::stdout().flush().unwrap();
            get_coordinate_input_from_user()
        }
    };

    if coordinate < 0 || coordinate > 2 {
        print!("\tPlease enter a valid coordinate (0, 1, 2): ");
        io::stdout().flush().unwrap();
        return get_coordinate_input_from_user();
    }

    return coordinate;
}

/// Prints each space of the map in a unicode representation.
fn print_field(map: &[[&str; 3]; 3]) {
    println!("
           0     1     2
              |     |
        0  {}  |  {}  |  {}
         _____|_____|_____
              |     |
        1  {}  |  {}  |  {}
         _____|_____|_____
              |     |
        2  {}  |  {}  |  {}
              |     |
      ",
             map[0][0],
             map[0][1],
             map[0][2],
             map[1][0],
             map[1][1],
             map[1][2],
             map[2][0],
             map[2][1],
             map[2][2],
    );
}