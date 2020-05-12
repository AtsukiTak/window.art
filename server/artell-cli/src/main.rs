use serde::Serialize;
use structopt::StructOpt;
use uuid::Uuid;

#[derive(Debug, StructOpt)]
#[structopt(name = "admin-cli")]
enum Command {
    AddArt(AddArtCommand),
}

#[tokio::main]
async fn main() {
    let command = Command::from_args();
    match command {
        Command::AddArt(cmd) => add_art(cmd).await,
    };
}

#[derive(Debug, StructOpt)]
struct AddArtCommand {
    server_url: String,
    #[structopt(short)]
    artist_id: Uuid,
    #[structopt(short)]
    title: String,
    #[structopt(short)]
    image_path: String,
}

async fn add_art(cmd: AddArtCommand) {
    let img_data = std::fs::read(cmd.image_path).unwrap();
    let encoded_img_data = base64::encode(img_data);

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct ReqBody {
        artist_id: Uuid,
        title: String,
        image_data: String,
    }

    let body = ReqBody {
        artist_id: cmd.artist_id,
        title: cmd.title,
        image_data: encoded_img_data,
    };

    let res = reqwest::Client::new()
        .post(&format!("{}/api/v1/admin/add_art", cmd.server_url))
        .json(&body)
        .send()
        .await
        .unwrap();
    println!("{:?}", res);
    let txt = res.text().await;
    println!("{:?}", txt);
}
