use chrono::{DateTime, Utc};
use structopt::StructOpt;
use uuid::Uuid;

#[derive(StructOpt, Debug)]
pub struct Command {
    server_url: String,
}

#[derive(Deserialize, Debug)]
struct ResBody {
    schedules: Vec<ResSchedule>,
}

#[derive(Deserialize, Debug)]
struct ResSchedule {
    art_id: Uuid,
    activate_at: DateTime<Utc>,
}

pub async fn execute(cmd: Command) {
    let res = reqwest::Client::new()
        .get(&format!("{}/api/v1/admin/get_schedules", cmd.server_url))
        .send()
        .await
        .unwrap()
        .json::<ResBody>()
        .await
        .unwrap();

    println!("{:?}", res);
}
