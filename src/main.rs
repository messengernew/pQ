// ---------------- {
mod utils;
// ----------------
mod select;
// ----------------
mod run;
// ----------------
mod package;
// ----------------
mod process;
// ----------------
mod timezone;
// ---------------- }

#[tokio::main]
async fn main() {
    run::app().await;
}
