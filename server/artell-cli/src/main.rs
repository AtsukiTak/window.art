#[macro_use]
extern crate serde;

mod add_art;
mod add_artist;
mod add_schedule;
mod get_schedules;

const API_SERVER_BASE: &'static str = env!("API_SERVER_BASE");

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "admin-cli")]
enum Command {
    AddArtist(add_artist::Command),
    AddArt(add_art::Command),
    AddSchedule(add_schedule::Command),
    GetSchedules(get_schedules::Command),
}

#[tokio::main]
async fn main() {
    let command = Command::from_args();
    match command {
        Command::AddArtist(cmd) => add_artist::execute(cmd).await,
        Command::AddArt(cmd) => add_art::execute(cmd).await,
        Command::AddSchedule(cmd) => add_schedule::execute(cmd).await,
        Command::GetSchedules(cmd) => get_schedules::execute(cmd).await,
    };
}
