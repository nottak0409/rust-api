-- This file should undo anything in `up.sql`
migrations/2022-04-12-132620_create_users/up.sql
CREATE TABLE users
(
    id INT PRIMARY KEY AUTO_INCREMENT,
    email VARCHAR(64) NOT NULL
);