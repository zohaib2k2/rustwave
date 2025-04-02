use std::{fs::OpenOptions, io::BufRead, io::BufReader};

#[derive(Debug)]
pub enum WaveError {
    InvalidUrl,
    FileDoesNotExist,
    CouldNotOpenFile(std::io::Error),
    CouldNotParseLine(Box<dyn std::error::Error>),
    EmptyFile,
}

type Result<T> = std::result::Result<T, WaveError>;

pub fn read_to_lines(filename: &str) -> Result<Vec<String>> {
    let path = std::path::Path::new(filename);

    // Error 1: file doesn't exist
    if !path.exists() {
        return Err(WaveError::FileDoesNotExist);
    }

    let file_res = OpenOptions::new().read(true).open(path);
    match file_res {
        Ok(file) => {
            let lines: Vec<_> = BufReader::new(file).lines().collect();
            //Error3: lines couldn't be parsed
            lines
                .into_iter()
                .map(|line| line.map_err(|e| WaveError::CouldNotParseLine(Box::new(e))))
                .collect()
        }
        // Error 2: File couldn't be opend
        Err(e) => Err(WaveError::CouldNotOpenFile(e)),
    }
}
