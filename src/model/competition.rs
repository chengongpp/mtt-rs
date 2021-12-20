use chrono::{DateTime, Utc};
use crate::model::user::User;

struct Team {
    id: i32,
    competition_id: i32,
    name: String,
}

struct Competition {
    id: i32,
    name: String,
    desc: String,
    is_public: bool,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    score_rule: i32,
}

struct Quiz {
    id: i32,
    competition_id: i32,
    name: String,
    desc: String,
}

struct Tag {
    id : i32,
    tag: String,
}

struct Flag {
    id: i32,
    quiz_id: i32,
    flag: String,
    ctime: DateTime<Utc>,
}