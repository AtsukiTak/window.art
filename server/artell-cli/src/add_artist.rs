use structopt::StructOpt;

/*
 * ===========
 * AddArtist
 * ===========
 */
#[derive(Debug, StructOpt)]
pub struct Command {
    name: String,
    email: String,
    status_msg: String,
    description: String,
    instagram: String,
    twitter: String,
}

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

pub async fn execute(cmd: Command) {
    let body = ReqBody {
        name: cmd.name,
        email: cmd.email,
        status_msg: cmd.status_msg,
        description: cmd.description,
        instagram: cmd.instagram,
        twitter: cmd.twitter,
    };

    let res = reqwest::Client::new()
        .post(&format!(
            "{}/api/v1/admin/add_artist",
            crate::API_SERVER_BASE
        ))
        .json(&body)
        .send()
        .await
        .unwrap()
        .json::<String>()
        .await
        .unwrap();
    println!("{:?}", res);
}
