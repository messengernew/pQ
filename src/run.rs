use crate::select::get_selected_indices;
use crate::utils::{parse_line};
use serde_json::json;
use std::collections::HashMap;
use std::process::{
    Command,
    exit as proc_exit,
    Output
};
use crate::paru::install;
use crate::process;

#[derive(Debug)]
struct Item {
    name: String,
    old_version: String,
    new_version: String,
}

async fn run_command() -> Output {
    Command::new("sh")
        .arg("checkupdates")
        .output()
        .expect("Error executing command")
}

pub async fn app() {
    let output = run_command().await;

    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut items = Vec::new();
    let mut result = HashMap::new();

    for (index, line) in stdout.lines().enumerate() {
        if let Some((name, old_version, new_version)) = parse_line(line).await {
            let item = Item {
                name: name.to_string(),
                old_version: old_version.to_string(),
                new_version: new_version.to_string(),
            };
            items.push(item);
            result.insert(index + 1, json!({
                "name": name,
                "old": old_version,
                "new": new_version
            }));
        }
    }

    if items.is_empty() { process::exit(2).msg("Updates not found") }
    println!("Update List:");
    for (i, item) in items.iter().enumerate() {
        println!("{}. {} | {} -> {}", i + 1, item.name, item.old_version, item.new_version);
    }

    let selected_indices = get_selected_indices(items.len()).await;

    let filtered_result: HashMap<_, _> = result.into_iter()
        .filter(|(key, _)| selected_indices.contains(key))
        .collect();

    let _json_result = json!(filtered_result);

    let names_string = filtered_result.values()
        .map(|item| item["name"].as_str().unwrap())
        .collect::<Vec<_>>()
        .join(" ");

    proc_exit(install(&names_string).await);
}
