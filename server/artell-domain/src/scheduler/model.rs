use crate::art::ArtId;
use chrono::{DateTime, Utc};

pub struct Scheduler {
    pub current_art_id: ArtId,
    pub waiting_list: Vec<Schedule>,
}

pub struct Schedule {
    pub art_id: ArtId,
    pub activate_at: DateTime<Utc>,
}

impl Scheduler {
    pub fn new(current_art_id: ArtId) -> Scheduler {
        Scheduler {
            current_art_id,
            waiting_list: Vec::new(),
        }
    }

    /// `activate_at` が過去の時間でも追加できる。
    /// その場合、次の `check_update` 時にそのArtが適用される
    pub fn add_schedule(&mut self, art_id: ArtId, activate_at: DateTime<Utc>) {
        self.waiting_list.push(Schedule {
            art_id,
            activate_at,
        });
        self.waiting_list.sort_unstable_by_key(|s| s.activate_at);
    }

    /// `waiting_list` の中で最もactivateが早いもの
    /// かつすでにactivate時間が過ぎているものを
    /// `current_art_id` に更新する
    pub fn check_update(&mut self) {
        let need_update = self
            .waiting_list
            .first()
            .map(|next| next.activate_at <= Utc::now())
            .unwrap_or(false);

        if need_update {
            let next = self.waiting_list.remove(0);
            self.current_art_id = next.art_id;
        }
    }
}
