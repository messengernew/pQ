use std::collections::HashMap;

pub async fn parse_line(line: &str) -> Option<(String, String, String)> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() == 4 && parts[2] == "->" {
        let name = parts[0].to_string();
        let old_version = parts[1].to_string();
        let new_version = parts[3].to_string();
        Some((name, old_version, new_version))
    } else {
        None
    }
}

#[allow(dead_code)]
pub async fn parse_range(part: &str, selected_indices: &mut HashMap<usize, ()>) {
    let range: Vec<&str> = part.split('-').collect();
    match range.as_slice() {
        [start] => {
            if let Ok(start) = start.parse::<usize>() {
                selected_indices.insert(start, ());
            }
        }
        [start, end] => {
            if let (Ok(start), Ok(end)) = (start.parse::<usize>(), end.parse::<usize>()) {
                for i in start..=end {
                    selected_indices.insert(i, ());
                }
            }
        }
        _ => {}
    }
}