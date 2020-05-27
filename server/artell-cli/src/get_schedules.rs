use crate::API_SERVER_BASE;
use chrono::{DateTime, FixedOffset, Utc};
use structopt::StructOpt;
use uuid::Uuid;

#[derive(StructOpt, Debug)]
pub struct Command {}

#[derive(Deserialize, Debug)]
struct ResBody {
    schedules: Vec<ResSchedule>,
}

#[derive(Deserialize, Debug)]
struct ResSchedule {
    art_id: Uuid,
    activate_at: DateTime<Utc>,
}

pub async fn execute(_: Command) {
    let res = reqwest::Client::new()
        .get(&format!("{}/api/v1/admin/get_schedules", API_SERVER_BASE))
        .send()
        .await
        .unwrap()
        .json::<ResBody>()
        .await
        .unwrap();

    let jst_offset = FixedOffset::east(9 * 3600);

    for schedule in res.schedules {
        let activate_jst = schedule.activate_at + jst_offset;
        println!("{}   {}", activate_jst, schedule.art_id);
    }
}
