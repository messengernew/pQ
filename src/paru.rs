use std::ffi::OsStr;
use std::io;
use std::io::Write;
use std::process::{Command, Stdio};

pub async fn install <A: AsRef< OsStr > + std::fmt::Display>
(
    arg: A
) -> i32
{
    match paru_installed().await {
        true => {
            match Command::new("paru").arg("-Sy").arg(&arg).status()
            {
                Ok(status) => status.code().unwrap_or(-1),
                Err(e) => {
                    eprintln!("Installation finished with an error: {}", e);
                    e.raw_os_error().unwrap_or(-1)
                }
            }
        },
        false => {
            if answer_yes_no("Do you want to install paru?") {
                let args = vec!["install", "paru"];
                match run_command("cargo", args).await {
                    true => {
                        match Command::new("paru").arg("-Sy").arg(arg).status()
                        {
                            Ok(status) => status.code().unwrap_or(-1),
                            Err(e) => {
                                eprintln!("Installation finished with an error: {}", e);
                                e.raw_os_error().unwrap_or(-1)
                            }
                        }
                    },
                    false => { eprintln!("Failed to install paru"); -1 }
                }
            } else { 0 }
        }
    }
}

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

pub(crate) async fn paru_installed() -> bool {
    Command::new("which")
        .arg("paru")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}
