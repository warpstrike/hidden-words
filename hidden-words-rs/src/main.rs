use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufRead;

// Struct to represent grid dimensions
struct Grid {
    data: Vec<Vec<char>>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.data {
            for &ch in row {
                write!(f, "{} ", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// Function to read the grid from a file
fn read_grid_from_file(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let mut grid = Vec::new();

    // Open the file
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    // Read lines and convert to Vec<Vec<char>>
    for line in reader.lines() {
        if let Ok(line_str) = line {
            // Parse the line into a Vec<char> and push it to the grid
            let char_row: Vec<char> = line_str.chars().collect();
            grid.push(char_row);
        }
    }

    Ok(grid)
}

// Print the grid
fn print_grid(grid: &Grid) {
    println!("{}", grid); // fmt implementation for Grid
}

// Parse the grid and word list from provided text
// fn parse_grid_and_words(text: &str) -> (Vec<Vec<char>>, Vec<String>) {
//     let mut grid = Vec::new();
//     let mut word_list = Vec::new();

//     for line in text.lines() {
//         let cleaned_line = line.trim();
//         if cleaned_line.chars().all(char::is_alphabetic) && cleaned_line.len() > 1 {
//             grid.push(cleaned_line.chars().collect());
//         } else if !cleaned_line.is_empty() {
//             word_list.push(cleaned_line.to_string());
//         }
//     }

//     (grid, word_list)
// }

// Function to validate the parsed grid
fn validate_grid(grid: &Vec<Vec<char>>) -> bool {
    if grid.is_empty() {
        println!("Error: Grid is empty.");
        return false;
    }

    let row_length = grid[0].len();

    if row_length == 0 {
        println!("Error: Grid has an empty row.");
        return false;
    }

    // Ensure the grid is rectangular (all rows have the same length)
    for (i, row) in grid.iter().enumerate() {
        if row.len() != row_length {
            println!(
                "Error: Row {} has inconsistent length (expected {}, found {}).",
                i,
                row_length,
                row.len()
            );
            return false;
        }
    }

    // Ensure all characters are alphabetic
    for row in grid {
        for &ch in row {
            if !ch.is_alphabetic() {
                println!("Error: Grid contains a non-alphabetic character '{}'.", ch);
                return false;
            }
        }
    }

    true
}

// Function to validate the word list
fn validate_word_list(words: &Vec<String>, grid: &Vec<Vec<char>>) -> bool {
    let max_grid_len = std::cmp::max(grid.len(), grid[0].len()); // Max row or column size

    for word in words {
        if !word.chars().all(char::is_alphabetic) {
            println!("Error: Word '{}' contains invalid characters.", word);
            return false;
        }

        if word.len() > max_grid_len {
            println!(
                "Warning: Word '{}' is longer than the grid's max dimension ({}).",
                word, max_grid_len
            );
        }
    }

    true
}

// Implement the word search function
fn search_word(grid: &Vec<Vec<char>>, word: &str) -> Option<((usize, usize), (usize, usize))> {
    let directions = [
        (0, 1),   // Right
        (1, 0),   // Down
        (0, -1),  // Left
        (-1, 0),  // Up
        (1, 1),   // Down-right
        (1, -1),  // Down-left
        (-1, 1),  // Up-right
        (-1, -1), // Up-left
    ];

    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        for j in 0..cols {
            for dir in &directions {
                let mut found = true;
                let mut end_pos = (i, j);

                for (k, c) in word.chars().enumerate() {
                    let new_row = i as isize + k as isize * dir.0;
                    let new_col = j as isize + k as isize * dir.1;

                    if new_row < 0
                        || new_col < 0
                        || new_row >= rows as isize
                        || new_col >= cols as isize
                    {
                        found = false;
                        break;
                    }

                    if grid[new_row as usize][new_col as usize] != c {
                        found = false;
                        break;
                    }

                    end_pos = (new_row as usize, new_col as usize);
                }

                if found {
                    return Some(((i, j), end_pos));
                }
            }
        }
    }

    None
}

// Search for all words in the grid and print results
fn search_all_words(grid: &Vec<Vec<char>>, words: &Vec<String>) {
    for word in words {
        if let Some((start, end)) = search_word(&grid, word) {
            println!("Word '{}' found from {:?} to {:?}", word, start, end);
        } else {
            println!("Word '{}' not found", word);
        }
    }
}

fn main() {
    // Step 1: Manually provide the text, grid and word list (or load from file)
    let grid_data = read_grid_from_file("data/grid.txt").expect("Failed to read grid");
    let words: Vec<String> = vec![
        "HELLO".to_string(),
        "WORLD".to_string(),
        "FIND".to_string(),
        "HORIZONTAL".to_string(),
        "VERTICAL".to_string(),
        "DIAGONAL".to_string(),
        "GOLDORAK".to_string(),
    ];

    // Step 2: Parse the grid and word list
    // let (grid_data, words) = parse_grid_and_words(&text);

    // Step 3: Validate grid and word list
    if !validate_grid(&grid_data) {
        println!("Grid validation failed. Exiting...");
        return;
    }

    if !validate_word_list(&words, &grid_data) {
        println!("Word list validation failed. Exiting...");
        return;
    }

    // Create a Grid struct for better handling
    let grid = Grid { data: grid_data };
    print_grid(&grid);

    // Step 4: Search for words in the grid
    search_all_words(&grid.data, &words);
}
