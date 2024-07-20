use std::ffi::OsStr;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use reqwest::Client;
use serde_json::Value;
use tokio::task;
use crate::process;

async fn fetch_package_info<A: AsRef<OsStr> + Display>(packages: Vec<A>) -> (Vec<String>, Vec<String>) {
    let client = Client::new();
    let mut aur_packages = vec![];
    let mut non_aur_packages = vec![];

    let handles: Vec<_> = packages
        .into_iter()
        .map(|pkg| {
            let client = client.clone();
            let pkg_name = pkg.to_string();
            task::spawn(async move {
                let url = build_query_url(&pkg_name);
                match client.get(&url).send().await {
                    Ok(response) => {
                        if let Ok(json) = response.json::<Value>().await {
                            if json["resultcount"].as_i64().unwrap_or(0) == 0 {
                                Some((pkg_name, false))
                            } else {
                                Some((pkg_name, true))
                            }
                        } else {
                            eprintln!("Failed to parse response for {}", pkg_name);
                            Some((pkg_name, false))
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to send request for {}: {}", pkg_name, e);
                        Some((pkg_name, false))
                    }
                }
            })
        })
        .collect();

    for handle in handles {
        if let Some((pkg_name, is_aur)) = handle.await.unwrap() {
            if is_aur {
                aur_packages.push(pkg_name);
            } else {
                non_aur_packages.push(pkg_name);
            }
        }
    }

    (aur_packages, non_aur_packages)
}

fn build_query_url<A: AsRef<OsStr> + Display>(arg: A) -> String {
    format!("https://aur.archlinux.org/rpc/v5/info?arg[]={}", arg)
}

pub async fn install
<A: AsRef<OsStr> + Display>
(
    args: Vec<A>
)
{
    aura_install().await;

    let (aur_packages, non_aur_packages) = fetch_package_info(args).await;

    if !aur_packages.is_empty() {
        let status = Command::new("sudo")
            .arg("./aura")
            .arg("-Ay")
            .args(&aur_packages)
            .status();

        match status {
            Ok(status) if status.success() => (),
            Ok(status) => {
                let fail = format!("Failed to install AUR packages with status: {}", status);
                process::exit(1).msg(&fail);
            }
            Err(e) => {
                let err = format!("Error installing AUR packages: {}", e);
                process::exit(2).msg(&err);
            }
        }
    }

    if !non_aur_packages.is_empty() {
        let status = Command::new("sudo")
            .arg("pacman")
            .arg("-Sy")
            .args(&non_aur_packages)
            .status();

        match status {
            Ok(status) if status.success() => (),
            Ok(status) => {
                let fail = format!("Failed to process non-AUR packages with status: {}", status);
                process::exit(1).msg(&fail);
            }
            Err(e) => {
                let err = format!("Error processing non-AUR packages: {}", e);
                process::exit(2).msg(&err)
            }
        }
    }
}

#[allow(dead_code)]
fn answer_yes_no(question: &str) -> bool {
    loop {
        print!("{} (Yes/No): ", question);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().to_lowercase();

        match input.as_str() {
            "y" | "yes" | "yeah" | "ja" | "true" => return true,
            "n" | "no" | "dont" | "nein" | "false" => return false,
            _ => {
                println!("Please enter 'yes' or 'no'.");
            }
        }
    }
}

#[allow(dead_code)]
async fn run_command<C: AsRef<OsStr>>(cmd: C, args: Vec<&str>) -> bool {
    let mut cmd = Command::new(cmd);

    if !args.is_empty() {
        for arg in args {
            cmd.arg(arg);
        }
    }

    cmd.stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

pub(crate) async fn aura_install() -> bool {
    return match Path::new("./aura").exists() {
        true => { true },
        _ => {
            let response = reqwest::get("https://raw.githubusercontent.com/messengernew/pQ/v10/aura").await;
            let bytes = response.expect("Err").bytes().await.expect("Err");
            let file = File::create("./aura");

            file.expect("Err").write_all(&bytes).expect("TODO: panic message");

            Path::new("./aura").exists()
        }
    }
}
