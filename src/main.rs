mod utils;
mod select;
mod run;
mod paru;
mod process;
mod timezone;

#[tokio::main]
async fn main() {
    run::app().await;
}
