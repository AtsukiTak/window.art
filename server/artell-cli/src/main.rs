#[macro_use]
extern crate serde;

mod get_schedules;

use chrono::{DateTime, Utc};
use structopt::StructOpt;
use uuid::Uuid;

#[derive(Debug, StructOpt)]
#[structopt(name = "admin-cli")]
enum Command {
    AddArtist(AddArtistCommand),
    AddArt(AddArtCommand),
    AddSchedule(AddScheduleCommand),
    GetSchedules(get_schedules::Command),
}

#[tokio::main]
async fn main() {
    let command = Command::from_args();
    match command {
        Command::AddArtist(cmd) => add_artist(cmd).await,
        Command::AddArt(cmd) => add_art(cmd).await,
        Command::AddSchedule(cmd) => add_schedule(cmd).await,
        Command::GetSchedules(cmd) => get_schedules::execute(cmd).await,
    };
}

/*
 * ===========
 * AddArtist
 * ===========
 */
#[derive(Debug, StructOpt)]
struct AddArtistCommand {
    server_url: String,
    name: String,
    email: String,
    status_msg: String,
    description: String,
    instagram: String,
    twitter: String,
}

async fn add_artist(cmd: AddArtistCommand) {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct ReqBody {
        name: String,
        email: String,
        status_msg: String,
        description: String,
        instagram: String,
        twitter: String,
    }

    let body = ReqBody {
        name: cmd.name,
        email: cmd.email,
        status_msg: cmd.status_msg,
        description: cmd.description,
        instagram: cmd.instagram,
        twitter: cmd.twitter,
    };

    let res = reqwest::Client::new()
        .post(&format!("{}/api/v1/admin/add_artist", cmd.server_url))
        .json(&body)
        .send()
        .await
        .unwrap();
    println!("{:?}", res);
    let txt = res.text().await.unwrap();
    println!("{:?}", txt);
}

/*
 * ===========
 * AddArt
 * ===========
 */
#[derive(Debug, StructOpt)]
struct AddArtCommand {
    server_url: String,
    artist_id: Uuid,
    title: String,
    materials: String,
    width: Option<usize>,
    height: Option<usize>,
    image_path: String,
    portfolio_id: String,
}

async fn add_art(cmd: AddArtCommand) {
    let img_data = std::fs::read(cmd.image_path.as_str()).unwrap();
    let encoded_img_data = base64::encode(img_data);

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct ReqBody {
        artist_id: Uuid,
        title: String,
        materials: String,
        size: Option<(usize, usize)>,
        image_data: String,
        portfolio_id: String,
    }

    let size = cmd
        .width
        .as_ref()
        .and_then(|w| cmd.height.as_ref().map(|h| (*w, *h)));

    let body = ReqBody {
        artist_id: cmd.artist_id,
        title: cmd.title,
        materials: cmd.materials,
        size,
        image_data: encoded_img_data,
        portfolio_id: cmd.portfolio_id,
    };

    let res = reqwest::Client::new()
        .post(&format!("{}/api/v1/admin/add_art", cmd.server_url))
        .json(&body)
        .send()
        .await
        .unwrap();
    println!("{:?}", res);
    let txt = res.text().await.unwrap();
    println!("{:?}", txt);
}

/*
 * ===========
 * AddSchedule
 * ===========
 */
#[derive(Debug, StructOpt)]
struct AddScheduleCommand {
    server_url: String,
    art_id: Uuid,
    activate_at: DateTime<Utc>,
}

async fn add_schedule(cmd: AddScheduleCommand) {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct ReqBody {
        art_id: Uuid,
        activate_at: DateTime<Utc>,
    }

    let body = ReqBody {
        art_id: cmd.art_id,
        activate_at: cmd.activate_at,
    };

    let res = reqwest::Client::new()
        .post(&format!("{}/api/v1/admin/add_schedule", cmd.server_url))
        .json(&body)
        .send()
        .await
        .unwrap();
    println!("{:?}", res);
    let txt = res.text().await.unwrap();
    println!("{:?}", txt);
}
