-- Your SQL goes here
CREATE TABLE `rant_info` (
    `id` INT PRIMARY KEY,
    `text` TEXT NOT NULL,
    `score` INT NOT NULL,
    `created_time` INT NOT NULL,
    `num_comments` INT NOT NULL,
    `tags` TEXT NOT NULL,
    `vote_state` INT NOT NULL,
    `edited` BOOLEAN NOT NULL,
    `user_username` TEXT NOT NULL,
    `user_score` INT NOT NULL
);