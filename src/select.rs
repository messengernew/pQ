use std::collections::HashSet;
use std::io::{self, Write};

pub async fn get_selected_indices(len: usize) -> HashSet<usize> {
    let sel = "Packages to install";
    let err_buf = "Failed to clear buffer";
    let err_line = "Failed to read the line";

    match len {
        1 => {
            let mut selected_indices = HashSet::new();
            parse_part("1", &mut selected_indices);
            selected_indices
        }
        2 => {
            print!("{sel} (e.g., 1 2/1,2): ");
            io::stdout().flush().expect(err_buf);

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect(err_line);

            let input = input.trim();
            let mut selected_indices = HashSet::new();

            for part in input.split_whitespace() {
                for subpart in part.split(',') {
                    parse_part(subpart.trim(), &mut selected_indices);
                }
            }

            selected_indices
        }
        _ => {
            print!("{sel} (e.g., 1 2 3, 1-3): ");
            io::stdout().flush().expect(err_buf);

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect(err_line);

            let input = input.trim();
            let mut selected_indices = HashSet::new();

            for part in input.split_whitespace() {
                for subpart in part.split(',') {
                    parse_part(subpart.trim(), &mut selected_indices);
                }
            }

            selected_indices
        }
    }
}

fn parse_part(part: &str, selected_indices: &mut HashSet<usize>) {
    if part.contains('-') {
        parse_range(part, selected_indices);
    } else {
        parse_single(part, selected_indices);
    }
}

fn parse_single(part: &str, selected_indices: &mut HashSet<usize>) {
    if let Ok(index) = part.parse::<usize>() {
        selected_indices.insert(index);
    }
}

fn parse_range(part: &str, selected_indices: &mut HashSet<usize>) {
    let range: Vec<&str> = part.split('-').collect();
    match range.as_slice() {
        [start, end] => {
            if let (Ok(start), Ok(end)) = (start.trim().parse::<usize>(), end.trim().parse::<usize>()) {
                for i in start..=end {
                    selected_indices.insert(i);
                }
            }
        }
        _ => {}
    }
}
