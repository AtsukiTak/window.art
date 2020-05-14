use super::{schema::schedules, Connection, Postgres};
use artell_domain::{
    art::ArtId,
    scheduler::{Schedule, Scheduler, SchedulerRepository},
};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

pub struct PgSchedulerRepository {
    pg: Postgres,
}

impl PgSchedulerRepository {
    pub fn new(pg: Postgres) -> Self {
        PgSchedulerRepository { pg }
    }
}

#[async_trait]
impl SchedulerRepository for PgSchedulerRepository {
    async fn find(&self) -> anyhow::Result<Option<Scheduler>> {
        self.pg.try_with_conn(find).await
    }

    async fn save(&self, scheduler: Scheduler) -> anyhow::Result<()> {
        self.pg
            .try_with_conn(move |conn| save(conn, scheduler))
            .await
    }
}

/*
 * ==========
 * Query
 * ==========
 */
#[derive(Queryable)]
struct QueriedSchedule {
    art_id: Uuid,
    activate_at: DateTime<Utc>,
}

impl Into<Schedule> for QueriedSchedule {
    fn into(self) -> Schedule {
        Schedule {
            art_id: ArtId(self.art_id),
            activate_at: self.activate_at,
        }
    }
}

fn find(conn: Connection) -> anyhow::Result<Option<Scheduler>> {
    let mut schedules = schedules::table
        .filter(schedules::is_scheduled.eq(true))
        .select((schedules::art_id, schedules::activate_at))
        .load::<QueriedSchedule>(&conn)?
        .into_iter()
        .map(QueriedSchedule::into)
        .collect();

    schedules.sort_unstable_by_key(|s| s.activate_at);

    Ok(Some(Scheduler { schedules }))
}

/*
 * ========
 * Update
 * ========
 */
#[derive(Clone, Copy, Insertable)]
#[table_name = "schedules"]
struct NewSchedule<'a> {
    art_id: &'a Uuid,
    activate_at: &'a DateTime<Utc>,
    is_scheduled: bool,
}

impl<'a> From<&'a Schedule> for NewSchedule<'a> {
    fn from(schedule: &'a Schedule) -> Self {
        NewSchedule {
            art_id: &schedule.art_id.0,
            activate_at: &schedule.activate_at,
            is_scheduled: true,
        }
    }
}

fn save(conn: Connection, scheduler: Scheduler) -> anyhow::Result<()> {
    // 一旦、全てのscheduleを無効にする
    diesel::update(schedules::table)
        .set(schedules::is_scheduled.eq(false))
        .execute(&conn)?;

    // 現在有効なscheduleを入力する
    let new_schedules = scheduler
        .schedules
        .iter()
        .map(NewSchedule::from)
        .collect::<Vec<_>>();

    diesel::insert_into(schedules::table)
        .values(new_schedules)
        .on_conflict((schedules::art_id, schedules::activate_at))
        .do_update()
        .set(schedules::is_scheduled.eq(true))
        .execute(&conn)?;

    Ok(())
}
