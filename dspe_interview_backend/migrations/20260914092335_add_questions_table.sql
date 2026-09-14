-- Add migration script here
CREATE TABLE questions
(
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    seniority_level TEXT NOT NULL,

    topic TEXT NOT NULL,

    question_text TEXT NOT NULL,

    suggested_answer TEXT NOT NULL,

    created_at TEXT NOT NULL
);