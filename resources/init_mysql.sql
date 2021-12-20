create table user
(
    id       int auto_increment primary key not null,
    email    varchar(64),
    phone    varchar(20),
    username varchar(32),
    password varchar(64)
);

create table user_detail
(
    id         int,
    nickname   varchar(32),
    avatar     varchar(32),
    created_at timestamp,
    updated_at timestamp,
    foreign key (id) references user (id)
);

create table admin_user
(
    id       int auto_increment primary key not null,
    email    varchar(64),
    phone    varchar(20),
    username varchar(32),
    password varchar(64)
);

create table competition
(
    id          int auto_increment primary key not null,
    name        varchar(64),
    description text,
    is_public   bool,
    start_time  datetime,
    end_time    datetime,
    score_rule  int,
    index name_idx (name)
);

create table quiz
(
    id          int auto_increment primary key not null,
    competition_id int,
    name        varchar(32),
    description varchar(32),
    attachment_filename varchar(128),
    attachment_url varchar(128),
    foreign key (quiz_id) references competition (id)
);

create table flag
(
    id  int,
    quiz_id int,
    flag varchar(128),
    ctime timestamp,
    foreign key (quiz_id) references quiz (id)
);

create table tag
(
    id  int auto_increment primary key not null,
    tag varchar(16),
    index tag_idx (tag)
);

create table team
(
    id  int primary key auto_increment not null ,
    competition_id  int,
    name    varchar(32),
    foreign key (competition_id) references competition (id),
    index competition_idx(competition_id)
);

create table competition_quiz
(
    competition_id int,
    quiz_id        int,
    tag_id         int,
    score          int,
    foreign key (quiz_id) references quiz (id),
    foreign key (competition_id) references competition (id),
    foreign key (tag_id) references tag (id)
);

create table competition_submit
(
    id             int auto_increment primary key not null,
    competition_id int,
    user_id        int,
    submit_time    timestamp,
    submit_content  varchar(256),
    index competition_idx(competition_id)
);

create table competition_member
(
    competition_id int,
    user_id        int,
    foreign key (competition_id) references competition (id),
    foreign key (user_id) references user (id)
);

create table member_team
(
    user_id int,
    team_id int,
    foreign key (user_id) references  user (id),
    foreign key (team_id) references  team (id)
);

create table competition_tags
(
    competition_id int,
    tag_id        int,
    foreign key (competition_id) references competition (id),
    foreign key (tag_id) references tag (id)
);

create table quiz_tags
(
    quiz_id int,
    tag_id  int,
    foreign key (quiz_id) references quiz (id),
    foreign key (tag_id) references tag (id)
);