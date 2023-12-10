use std::{path::PathBuf, io, fs::File};

fn get_file_path(file_name: &str) -> io::Result<PathBuf> {
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push(file_name);
    Ok(file_path)
}

pub fn get_reader(file_path: &str) -> io::Result<io::BufReader<File>> {
    let file_path = get_file_path(file_path)?;
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    Ok(reader)
}
