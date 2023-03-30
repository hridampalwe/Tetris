use std::fs::File;
use std::io::{self, Read, Write};

pub fn write_into_file(filename: &str, content: &str) -> io::Result<()> {
    let mut file_handler = File::create(filename)?;
    file_handler.write_all(content.as_bytes())
}

pub fn read_from_file(filename: &str) -> io::Result<String> {
    let mut file_handler = File::open(filename)?;
    let mut file_content = String::new();
    file_handler.read_to_string(&mut file_content)?;
    Ok(file_content)
}

pub fn slice_to_string(slice: &[u32]) -> String {
    slice
        .iter()
        .map(|highscore| highscore.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn save_highscore_and_lines(highscores: &[u32], number_of_lines: &[u32]) -> bool {
    let s_highscore = slice_to_string(highscores);
    let s_number_of_lines = slice_to_string(number_of_lines);
    write_into_file(
        "assets/scores.txt",
        &format!("{} \n {} \n ", s_highscore, s_number_of_lines)[..],
    )
    .is_ok()
}

fn string_to_slice(string_line: &str) -> Vec<u32> {
    string_line
        .split(" ")
        .filter_map(|nb| nb.parse::<u32>().ok())
        .collect::<Vec<u32>>()
}

pub fn read_high_scores() -> Option<(Vec<u32>, Vec<u32>)> {
    if let Ok(content) = read_from_file("assets/scores.txt") {
        let mut lines = content
            .splitn(2, "\n")
            .map(|line| string_to_slice(line))
            .collect::<Vec<_>>();
        if lines.len() == 2 {
            let (number_of_lines, highscore) = (lines.pop().unwrap(), lines.pop().unwrap());
            Some((number_of_lines, highscore))
        } else {
            None
        }
    } else {
        None
    }
}

pub fn update_vec(vec: &mut Vec<u32>, value: u32) -> bool {
    if vec.len() < 5 {
        vec.push(value);
        true
    } else {
        for item in vec.iter_mut() {
            if *item < value {
                *item = value;
                return true;
            }
        }
        false
    }
}
