use chrono::{DateTime, Utc};
use structopt::StructOpt;
use uuid::Uuid;

#[derive(Debug, StructOpt)]
pub struct Command {
    art_id: Uuid,
    activate_at: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ReqBody {
    art_id: Uuid,
    activate_at: DateTime<Utc>,
}

pub async fn execute(cmd: Command) {
    let body = ReqBody {
        art_id: cmd.art_id,
        activate_at: cmd.activate_at,
    };

    let res = reqwest::Client::new()
        .post(&format!(
            "{}/api/v1/admin/add_schedule",
            crate::API_SERVER_BASE
        ))
        .json(&body)
        .send()
        .await
        .unwrap();
    println!("{:?}", res);
    let txt = res.text().await.unwrap();
    println!("{:?}", txt);
}
