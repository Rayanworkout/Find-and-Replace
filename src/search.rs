// use crate::utils::PathExt;
use anyhow::{Context, Result};
use ignore::Walk;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// Function to get the matches in a file
/// We enumerate over the lines of the file
/// and filter the lines that contain the pattern
/// We return a vector of tuples containing the line number
/// and the content of the line
fn get_matches(file: BufReader<File>, pattern: &str) -> Vec<(usize, String)> {
    file.lines()
        .enumerate() // Add line numbers starting from 0
        .filter_map(|(line_number, content)| {
            match content {
                Ok(content) if content.contains(pattern) => Some((line_number + 1, content)), // Line numbers start from 1
                _ => None,
            }
        })
        .collect()
}

pub fn search_in_file(path: &PathBuf, pattern: &str) -> Result<()> {
    let file = File::open(&path).with_context(|| format!("could not read file `{:?}`", &path))?;

    let reader = BufReader::new(file);

    let matches = get_matches(reader, &pattern);

    if !matches.is_empty() {
        println!("\n{:?}", path);
        for (line_number, content) in matches {
            println!("Line {}: {}\n", line_number, content);
        }
    }

    Ok(())
}

pub fn search_in_folder(
    path: &PathBuf,
    _omit: &Option<Vec<PathBuf>>,
    _pattern: &str,
) -> Result<()> {
    // Here, we collect all the paths in the folder
    // with a Ok() result

    // Then we keep only those who aren't omitted
    // if let Some(to_omit) = omit {
    //     folder.retain(|path| !path.should_omit(&to_omit));
    // }

    for entry in Walk::new(&path) {
        match entry {
            Ok(data) => println!("{:?}", data.path().is_file()),
            Err(error) => eprintln!("Error: {}", error),
        }
    }

    // match entry {
    //     Ok(entry) => match entry.into_path().is_directory() {
    //         Some(result) => match result {
    //             true => search_in_folder(&path, &omit, &pattern)?,
    //             false => search_in_file(&path, &pattern)?,
    //         },
    //         None => eprintln!("Failed to read the following path: {:?}", path),
    //     },
    //     Err(err) => eprintln!("ERROR: {}", err),
    // }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_search_in_file() {
        let old_pattern = "old".to_string();
        let path = PathBuf::from("tests/assets/classic.txt");

        assert!(search_in_file(&path, &old_pattern).is_ok());
    }

    #[test]
    fn test_search_in_folder() {
        let old_pattern = "old".to_string();
        let path = PathBuf::from("tests/assets");

        assert!(search_in_folder(&path, &None, &old_pattern).is_ok());
    }

    #[test]
    fn test_get_matches_classic() {
        let file = BufReader::new(File::open("tests/assets/classic.txt").unwrap());
        let matches = get_matches(file, "more");

        assert_eq!(matches.len(), 1);
    }

    #[test]
    fn test_get_matches_empty() {
        let file = BufReader::new(File::open("tests/assets/empty.txt").unwrap());
        let matches = get_matches(file, "more");

        assert!(matches.is_empty());
    }

    #[test]
    fn test_get_matches_multiple() {
        let file = BufReader::new(File::open("tests/assets/classic.txt").unwrap());
        let matches = get_matches(file, "world");

        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_get_matches_no_match() {
        let file = BufReader::new(File::open("tests/assets/classic.txt").unwrap());
        let matches = get_matches(file, "foo");

        assert!(matches.is_empty());
    }
}
