use std::{io::{self, BufRead}, path::PathBuf, fs::File};

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

struct Games {
    games: Vec<Game>
}

impl Games {
    fn total_valid_games(&self) -> usize {
        self.games.iter().filter(|game| game.is_valid()).count()
    }

    fn sum_id_valid_games(&self) -> u32 {
        self.games.iter().filter(|game| game.is_valid()).map(|game| game.id).sum::<u32>()
    }

    fn sum_power_of_fewest_number_of_cube(&self) -> u32 {
        let mut sum_powers = 0;
        for game in &self.games {
            let cube = game.get_fewest_number_of_cube();
            let power = cube.red * cube.green * cube.blue;
            sum_powers += power;
        }
        sum_powers
    }
}

struct Game {
    id: u32,
    cubes: Vec<Cube>
}

impl Game {
    fn is_valid(&self) -> bool {
        self.cubes.iter().all(|cube| cube.is_valid())
    }

    fn get_fewest_number_of_cube(&self) -> Cube {
        let max_red = self.cubes.iter().map(|cube| cube.red).max().unwrap();
        let max_green = self.cubes.iter().map(|cube| cube.green).max().unwrap();
        let max_blue = self.cubes.iter().map(|cube| cube.blue).max().unwrap();

        let cube = Cube {
            red: max_red,
            green: max_green,
            blue: max_blue
        };
        cube
    }
}

struct Cube {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cube {
    fn is_valid(&self) -> bool {
        self.red <= MAX_RED && self.green <= MAX_GREEN && self.blue <= MAX_BLUE
    }
}

fn main() -> io::Result<()>{
    let file_path = get_file_path("src/advent2/input2.txt")?;
    let reader = get_reader(&file_path)?;
    let games = process_file(reader)?;

    println!("Total valid games: {}", games.total_valid_games());
    println!("Sum of id valid games: {}", games.sum_id_valid_games());
    println!("Sum of power of fewest number of cube: {}", games.sum_power_of_fewest_number_of_cube());
    Ok(())
}

fn get_file_path(file_name: &str) -> io::Result<PathBuf> {
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push(file_name);
    println!("In file {}", file_path.display());

    Ok(file_path)
}

fn get_reader(file_path: &PathBuf) -> io::Result<io::BufReader<File>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    Ok(reader)
}

fn process_file(reader: io::BufReader<File>) -> io::Result<Games> {
    let mut games = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let game = parse_line_to_game(&line);
        games.push(game);
    }

    let games = Games {
        games: games
    };

    Ok(games)
}

fn parse_line_to_game(line: &str) -> Game {
    let modified_line = line.replace(" ", "");
    let mut splitted_line = modified_line.split(":");

    let game_str = splitted_line.next().unwrap();
    let game_id = game_str.replace("Game", "").parse::<u32>().unwrap();

    let cubes_str = splitted_line.next().unwrap();
    let cubes_str = cubes_str.split(";");
    let mut cubes = Vec::new();
    for cube_str in cubes_str {
        let cube = parse_cube(cube_str);
        cubes.push(cube);
    }

    let game = Game {
        id: game_id,
        cubes: cubes
    };
    game
}

fn parse_cube(cube_str: &str) -> Cube {
    let cube_str = cube_str.split(",");

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for cube in cube_str {
        if cube.contains("red") {
            red = cube.replace("red", "").parse::<u32>().unwrap();
        } else if cube.contains("green") {
            green = cube.replace("green", "").parse::<u32>().unwrap();
        } else if cube.contains("blue") {
            blue = cube.replace("blue", "").parse::<u32>().unwrap();
        }
    }

    let cube = Cube {
        red: red,
        green: green,
        blue: blue
    };
    cube
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_to_game() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = parse_line_to_game(line);

        assert_eq!(game.id, 1);
        assert_eq!(game.cubes.len(), 3);
    }

    #[test]
    fn test_input_file() {
        let file_path = get_file_path("src/advent2/inputexample.txt").unwrap();
        let reader = get_reader(&file_path).unwrap();
        let games = process_file(reader).unwrap();

        let total_valid_games = games.total_valid_games();
        let sum_id_valid_games = games.sum_id_valid_games();
        assert_eq!(total_valid_games, 3);
        assert_eq!(sum_id_valid_games, 8);
    }

    #[test]
    fn test_input1() {
        let file_path = get_file_path("src/advent2/input1.txt").unwrap();
        let reader = get_reader(&file_path).unwrap();
        let games = process_file(reader).unwrap();

        let sum_id_valid_games = games.sum_id_valid_games();
        assert_eq!(sum_id_valid_games, 2369);
    }

    #[test]
    fn test_fewest_number_of_cube_of_example() {
        let file_path = get_file_path("src/advent2/inputexample.txt").unwrap();
        let reader = get_reader(&file_path).unwrap();
        let games = process_file(reader).unwrap();

        let game = games.games.first().unwrap();
        let cube = game.get_fewest_number_of_cube();
        let sum_powers = games.sum_power_of_fewest_number_of_cube();

        assert_eq!(cube.red, 4);
        assert_eq!(cube.green, 2);
        assert_eq!(cube.blue, 6);
        assert_eq!(sum_powers, 2286);
    }
}