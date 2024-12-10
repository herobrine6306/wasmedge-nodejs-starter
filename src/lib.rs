use rand::Rng;
use std::io;

// Game state
struct GameState {
    player_health: u8,
    player_position: (u8, u8),
    treasure_position: (u8, u8),
    maze: Vec<Vec<u8>>,
}

impl GameState {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let treasure_x = rng.gen_range(0..10);
        let treasure_y = rng.gen_range(0..10);
        let mut maze = vec![vec![0; 10]; 10];
        maze[treasure_x as usize][treasure_y as usize] = 2; // Treasure
        GameState {
            player_health: 100,
            player_position: (0, 0),
            treasure_position: (treasure_x, treasure_y),
            maze,
        }
    }

    fn print_maze(&self) {
        for y in 0..10 {
            for x in 0..10 {
                match self.maze[x as usize][y as usize] {
                    0 => print!(" "),
                    1 => print!("P"), // Player
                    2 => print!("T"), // Treasure
                    _ => print!("X"), // Obstacle
                }
            }
            println!();
        }
    }

    fn update(&mut self, direction: &str) {
        let (x, y) = self.player_position;
        match direction {
            "up" => {
                if y > 0 {
                    self.player_position.1 -= 1;
                }
            }
            "down" => {
                if y < 9 {
                    self.player_position.1 += 1;
                }
            }
            "left" => {
                if x > 0 {
                    self.player_position.0 -= 1;
                }
            }
            "right" => {
                if x < 9 {
                    self.player_position.0 += 1;
                }
            }
            _ => println!("Invalid direction"),
        }
        if self.player_position == self.treasure_position {
            println!(" Congratulations, you found the Raspberry Pi treasure!");
            std::process::exit(0);
        }
    }
}

fn main() {
    let mut game = GameState::new();
    loop {
        game.print_maze();
        println!("Health: {}", game.player_health);
        print!("Enter direction (up/down/left/right): ");
        let mut direction = String::new();
        io::stdin().read_line(&mut direction).expect("Failed to read line");
        game.update(&direction.trim());
    }
}
