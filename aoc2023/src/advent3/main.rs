use std::io::{self, BufRead};
use aoc2023::get_reader;


#[derive(Debug, Clone, PartialEq)]
struct PartNumber {
    characters: Vec<char>
}

impl PartNumber {
    fn new() -> PartNumber {
        PartNumber {
            characters: Vec::new()
        }
    }

    fn add_character(&mut self, character: char) {
        self.characters.push(character);
    }

    fn get_value(&self) -> u32 {
        let digits: String = self.characters.iter().collect();
        let value = digits.parse::<u32>().unwrap();
        value
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Dot,
    Symbol(char),
    Number(PartNumber)
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    start_x: i32,
    end_x: i32,
    start_y: i32,
    end_y: i32
}

#[derive(Debug, Clone)]
struct Cell {
    coordinate: Coordinate,
    value: Value
}

impl Cell {
    fn is_adjacent_to(&self, other: &Cell) -> bool {
        let coord1 = &self.coordinate;
        let coord2 = &other.coordinate;


        let is_left = self.is_in_range(coord1.start_x - 1, coord2.start_x, coord2.end_x) 
                            && self.is_in_range(coord1.start_y, coord2.start_y, coord2.end_y);

        let is_right = self.is_in_range(coord1.end_x + 1, coord2.start_x, coord2.end_x)
                            && self.is_in_range(coord1.start_y, coord2.start_y, coord2.end_y);

        let is_top = self.is_in_range(coord1.start_y - 1, coord2.start_y, coord2.end_y)
                           && self.is_in_range(coord1.start_x, coord2.start_x, coord2.end_x);
        
        let is_bottom = self.is_in_range(coord1.end_y + 1, coord2.start_y, coord2.end_y)
                            && self.is_in_range(coord1.start_x, coord2.start_x, coord2.end_x);

        let is_top_left = self.is_in_range(coord1.start_x - 1, coord2.start_x, coord2.end_x)
                                && self.is_in_range(coord1.start_y - 1, coord2.start_y, coord2.end_y);

        let is_top_right = self.is_in_range(coord1.end_x + 1, coord2.start_x, coord2.end_x)
                                && self.is_in_range(coord1.start_y - 1, coord2.start_y, coord2.end_y);

        let is_bottom_left = self.is_in_range(coord1.start_x - 1, coord2.start_x, coord2.end_x)
                                && self.is_in_range(coord1.end_y + 1, coord2.start_y, coord2.end_y);

        let is_bottom_right = self.is_in_range(coord1.end_x + 1, coord2.start_x, coord2.end_x)
                                && self.is_in_range(coord1.end_y + 1, coord2.start_y, coord2.end_y);

        is_left || is_right || is_top || is_bottom || is_top_left || is_top_right || is_bottom_left || is_bottom_right
    }

    fn is_in_range(&self, value: i32, start: i32, end: i32) -> bool {
        start <= value && value <= end
    }
}

#[derive(Debug)]
struct Engine {
    cells: Vec<Cell>
}

impl Engine {
    fn get_sum_of_all_valid_part_numbers(&self) -> u32 {
        let valid_cells = self.get_valid_cells();
        let mut sum = 0;
        for cell in valid_cells.iter() {
            match cell.value {
                Value::Number(ref part_number) => {
                    let value = part_number.get_value();
                    println!("{}", value);
                    sum += value;
                },
                _ => {}
            }
        }
        sum
    }

    fn get_valid_cells(&self) -> Vec<&Cell> {
        let numbered_cells = self.cells.iter().filter(|cell| match cell.value {
            Value::Number(_) => true,
            _ => false
        }).collect::<Vec<&Cell>>();

        let symbol_cells = self.cells.iter().filter(|cell| match cell.value {
            Value::Symbol(_) => true,
            _ => false
        }).collect::<Vec<&Cell>>();

        
        let mut valid_cells = Vec::new();
        for symbol_cell in symbol_cells.iter() {
            for numbered_cell in numbered_cells.iter() {
                if symbol_cell.is_adjacent_to(numbered_cell) {
                    valid_cells.push(*numbered_cell);
                }
            }
        }
        valid_cells
    }
}

fn process_file(file_path: &str) -> io::Result<Engine> {
    let reader = get_reader(file_path)?;

    let mut cells: Vec<Cell> = Vec::new();
    for (row, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut current_cell: Option<Cell> = None;
        let mut coordinate = Coordinate {
            start_x: 0,
            end_x: 0,
            start_y: row as i32,
            end_y: row as i32,
        };

        for (column, char) in line.chars().enumerate() {
            coordinate.start_x = column as i32;

            match char {
                '.' => {
                    if let Some(ref cell) = current_cell {
                        cells.push(cell.clone());
                        current_cell = None;
                    }

                    coordinate.end_x = coordinate.start_x;
                    let cell = Cell {
                        coordinate: coordinate.clone(),
                        value: Value::Dot
                    };
                    cells.push(cell);
                },
                c if c.is_ascii_digit() => {
                    match current_cell {
                        Some(ref mut cell) => {
                            cell.coordinate.end_x = coordinate.start_x;
                            match cell.value {
                                Value::Number(ref mut part_number) => {
                                    part_number.add_character(c);
                                },
                                _ => {
                                    let mut part_number = PartNumber::new();
                                    part_number.add_character(c);
                                    cell.value = Value::Number(part_number);
                                }
                            }
                        },
                        None => {
                            let mut part_number = PartNumber::new();
                            part_number.add_character(c);
                            let cell = Cell {
                                coordinate: coordinate.clone(),
                                value: Value::Number(part_number)
                            };
                            current_cell = Some(cell);
                        }
                    }
                }
                _ => {
                    if let Some(ref cell) = current_cell {
                        cells.push(cell.clone());
                        current_cell = None;
                    }

                    coordinate.end_x = coordinate.start_x;
                    let cell = Cell {
                        coordinate: coordinate.clone(),
                        value: Value::Symbol(char)
                    };
                    cells.push(cell);
                }
            }
        }

        // digit at the end of the line
        if let Some(ref cell) = current_cell {
            cells.push(cell.clone());
        }
    }

    let engine = Engine {
        cells
    };
    Ok(engine)
}



fn main() -> io::Result<()>{
    let engine = process_file("src/advent3/inputpart1.txt")?;
    let res = engine.get_sum_of_all_valid_part_numbers();
    println!("sum: {}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Cell, Coordinate, Value};
    use crate::process_file;

    #[test]
    fn test_adjacent() {
        let cell1 = Cell {
            coordinate: Coordinate {
                start_x: 0,
                end_x: 0,
                start_y: 0,
                end_y: 0
            },
            value: Value::Dot
        };

        let cell2 = Cell {
            coordinate: Coordinate {
                start_x: 1,
                end_x: 2,
                start_y: 0,
                end_y: 0
            },
            value: Value::Dot
        };

        let is_adjacent1 = cell2.is_adjacent_to(&cell1);
        let is_adjacent2 = cell1.is_adjacent_to(&cell2);

        assert_eq!(is_adjacent1, true);
        assert_eq!(is_adjacent2, true);
    }

    #[test]
    fn test_if_process_file_contains_correct_coordinates() {
        let res = process_file("src/advent3/inputtest1.txt").unwrap();

        let first = res.cells.first().unwrap();
        let last = res.cells.last().unwrap();

        assert_eq!(first.coordinate.start_x, 0);
        assert_eq!(first.coordinate.end_x, 2);
        assert_eq!(first.coordinate.start_y, 0);
        assert_eq!(first.coordinate.end_y, 0);
        
        assert_eq!(last.coordinate.start_x, 7);
        assert_eq!(last.coordinate.end_x, 9);
        assert_eq!(last.coordinate.start_y, 0);
        assert_eq!(last.coordinate.end_y, 0);
    }

    #[test]
    fn test_inputexample(){
        let res = process_file("src/advent3/inputexample.txt").unwrap();

        let res = res.get_sum_of_all_valid_part_numbers();
        println!("sum: {}", res);

        assert_eq!(res, 4361);
    }

    #[test]
    fn test_inputpart1(){
        let res = process_file("src/advent3/inputpart1.txt").unwrap();

        let res = res.get_sum_of_all_valid_part_numbers();
        println!("sum: {}", res);

        assert_eq!(res, 1102);
    }
}