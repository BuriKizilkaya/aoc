use std::{fs::{File}, path::PathBuf, io::{self, BufRead}};

fn main() -> io::Result<()>{
    let file_path = get_file_path("src/advent1/calibrationfile2.txt")?;
    let reader = get_reader(&file_path)?;
    let total = process_file(reader)?;
    
    println!("Total: {}", total);
    Ok(())
}

fn process_file(reader: io::BufReader<File>) -> io::Result<u32> {
    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let number = get_digite_per_line(&line);
        total += number;
    }

    Ok(total)
}

fn get_reader(file_path: &PathBuf) -> io::Result<io::BufReader<File>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    Ok(reader)
}

fn get_file_path(file_name: &str) -> io::Result<PathBuf> {
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push(file_name);
    println!("In file {}", file_path.display());

    Ok(file_path)
}

fn get_digite_per_line(line: &str) -> u32 {
    parse_line_by_part2(line)
}

fn parse_line_by_part2(line: &str) -> u32 {    
    // moving replacement by 1 char
    let mut index = 0;
    let line_iter = std::iter::from_fn(move || {
        let reduced_line = &line[index..];

        let result = if reduced_line.starts_with("one") {
            Some('1')
        } else if reduced_line.starts_with("two") {
            Some('2')
        } else if reduced_line.starts_with("three") {
            Some('3')
        } else if reduced_line.starts_with("four") {
            Some('4')
        } else if reduced_line.starts_with("five") {
            Some('5')
        } else if reduced_line.starts_with("six") {
            Some('6')
        } else if reduced_line.starts_with("seven") {
            Some('7')
        } else if reduced_line.starts_with("eight") {
            Some('8')
        } else if reduced_line.starts_with("nine") {
            Some('9')
        } else {
            let result = reduced_line.chars().next();
            result
        };

        index += 1;
        result
    });

    let modified_line = line_iter.collect::<String>();
    parse_line_by_part1(&modified_line)
}

#[allow(dead_code)]
fn parse_line_by_part1(line: &str) -> u32 {
    let mut first_digit = None;
    let mut last_digit = None;

    for c in line.chars() {
        if !c.is_numeric() {
            continue;
        }

        last_digit = Some(c.to_digit(10).unwrap());
        if first_digit.is_none() {
            first_digit = last_digit;
        }
    }

    let digits = format!("{}{}", first_digit.unwrap(), last_digit.unwrap());
    let number = digits.parse::<u32>().unwrap();
    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_by_part1() {
        for _ in 0..10 {
            let line = "zoneight234";
            let number = parse_line_by_part2(line);
            assert_eq!(number, 14);
        }
    }
}
