CREATE TABLE tasks (
       id    INTEGER PRIMARY KEY,
       description VACHAR NOT NULL,
       completed BOOLEAN NOT NULL DEFAULT 0
);

INSERT INTO tasks (description) VALUES ("my first task");
