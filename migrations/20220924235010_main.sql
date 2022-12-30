CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    user_id TEXT NOT NULL,
    bearer TEXT NOT NULL,
    user_name TEXT NOT NULL,
    email TEXT NOT NULL,
    registration_date INTEGER NOT NULL
);

CREATE TABLE announcements (
    id INTEGER PRIMARY KEY,
    owner_bearer TEXT NOT NULL,
    class_id TEXT NOT NULL,
    announcement_id TEXT NOT NULL,
    author_name TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    attachment TEXT NOT NULL,
    date INTEGER NOT NULL
);

CREATE TABLE whitelists (
    id INTEGER PRIMARY KEY,
    owner_bearer TEXT NOT NULL,
    class_id TEXT NOT NULL,
    whitelisted_user_name TEXT NOT NULL,
    whitelisted_user_id TEXT NOT NULL
);

CREATE TABLE classes (
    id INTEGER PRIMARY KEY,
    owner_id TEXT NOT NULL,
    owner_bearer TEXT NOT NULL,
    class_id TEXT NOT NULL,
    class_name TEXT NOT NULL,
    enable_whitelist INTEGER NOT NULL
);

CREATE TABLE units (
    id INTEGER PRIMARY KEY,
    owner_bearer TEXT NOT NULL,
    class_id TEXT NOT NULL,
    unit_id TEXT NOT NULL,
    unit_name TEXT NOT NULL,
    locked INTEGER NOT NULL
);

CREATE TABLE lessons (
    id INTEGER PRIMARY KEY,
    owner_bearer TEXT NOT NULL,
    unit_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    video TEXT NOT NULL,
    work TEXT NOT NULL,
    work_solutions TEXT NOT NULL
);

CREATE TABLE submissions (
    id INTEGER PRIMARY KEY,
    class_id TEXT NOT NULL,
    submission_id TEXT NOT NULL,
    submitter_bearer TEXT NOT NULL,
    submission_date INTEGER NOT NULL,
    data TEXT NOT NULL
);
