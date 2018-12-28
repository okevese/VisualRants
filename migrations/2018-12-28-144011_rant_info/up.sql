-- Your SQL goes here
create table `rant_info` (
    `id` int primary key,
    `text` text not null,
    `score` int not null,
    `created_time` int not null,
    `num_comments` int not null,
    `tags` text not null,
    `vote_state` int not null,
    `edited` boolean not null,
    `user_username` text not null,
    `user_score` int not null
);