use crate::art::ArtId;
use chrono::{DateTime, Utc};

pub struct Scheduler {
    pub schedules: Vec<Schedule>,
}

pub struct Schedule {
    pub art_id: ArtId,
    pub activate_at: DateTime<Utc>,
}

/*
 * ==========
 * Query
 * ==========
 */
impl Scheduler {
    pub fn current_art_id(&self) -> Option<&ArtId> {
        self.schedules.first().map(|sch| &sch.art_id)
    }
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            schedules: Vec::new(),
        }
    }

    /// `activate_at` が過去の時間でも追加できる。
    /// その場合、次の `check_update` 時にそのArtが適用される
    pub fn add_schedule(&mut self, art_id: ArtId, activate_at: DateTime<Utc>) {
        self.schedules.push(Schedule {
            art_id,
            activate_at,
        });
        self.schedules.sort_unstable_by_key(|s| s.activate_at);
    }

    /// 2番目のScheduleの開始時間がすぎていれば、
    /// それを1番目にする。
    /// 更新したかどうかをboolで返す
    pub fn check_update(&mut self) -> bool {
        let need_update = self
            .schedules
            .get(1)
            .map(|next| next.activate_at <= Utc::now())
            .unwrap_or(false);

        if need_update {
            self.schedules.remove(0);
        }

        need_update
    }
}
