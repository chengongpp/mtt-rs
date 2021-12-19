use crate::model::user::User;

struct Team {
    id: i32,
    name: String,
    member: Vec<User>
}

struct Competition {
    id: i32,
    name: String,
    desc: String,
    is_public: bool,

}