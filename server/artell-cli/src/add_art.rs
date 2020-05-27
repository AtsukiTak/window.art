use structopt::StructOpt;
use uuid::Uuid;

/*
 * ===========
 * AddArt
 * ===========
 */
#[derive(Debug, StructOpt)]
pub struct Command {
    artist_id: Uuid,
    title: String,
    materials: String,
    width: Option<usize>,
    height: Option<usize>,
    image_path: String,
    portfolio_link: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ReqBody {
    artist_id: Uuid,
    title: String,
    materials: String,
    size: Option<(usize, usize)>,
    image_data: String,
    portfolio_link: String,
}

pub async fn execute(cmd: Command) {
    let img_data = std::fs::read(cmd.image_path.as_str()).unwrap();
    let encoded_img_data = base64::encode(img_data);

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
        portfolio_link: cmd.portfolio_link,
    };

    let res = reqwest::Client::new()
        .post(&format!("{}/api/v1/admin/add_art", crate::API_SERVER_BASE))
        .json(&body)
        .send()
        .await
        .unwrap()
        .json::<String>()
        .await
        .unwrap();

    println!("{}", res);
}
