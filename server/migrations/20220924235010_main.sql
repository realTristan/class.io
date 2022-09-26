/*

    Users can visit the home endpoint to create their own website
    or
    They can visit the website their teacher provided them
    ex: site.com/user_hash/class_hash/<optional: unit_hash>

*/
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    user_hash TEXT NOT NULL,            -- The users email sha256 encrypted

    user_name TEXT NOT NULL,            -- A name the user can change themselves
    email TEXT NOT NULL,                -- Used for emailing students about their homework
    registration_date INTEGER NOT NULL  -- The time since epoch format of when the user registered
);
CREATE TABLE announcements (
    id INTEGER PRIMARY KEY,
    user_hash TEXT NOT NULL,        -- The users email sha256 encrypted
    class_hash TEXT NOT NULL,       -- user_hash:time.time()

    content TEXT NOT NULL,          -- The announcement content as -> split("user_name:title:description:attached_image", ":")
    date INTEGER NOT NULL           -- The time since epoch format of when the post was made
);

-- MAX 100 USERS
-- endpoint: get_class_data
CREATE TABLE whitelists (
    id INTEGER PRIMARY KEY,
    class_hash TEXT NOT NULL,       -- user_hash:time.time()

    whitelisted_user TEXT NOT NULL  -- The user to be whitelisted's email sha256 encrypted
);

-- MAX 10 CLASSES
-- endpoint: get_class_data
CREATE TABLE classes (
    id INTEGER PRIMARY KEY,
    owner_hash TEXT NOT NULL,           -- The users email sha256 encrypted
    class_hash TEXT NOT NULL,           -- user_hash:time.time()

    class_name TEXT NOT NULL,           -- Ex: MHF4UI
    rsl INTEGER NOT NULL,               -- Require Student Login BOOL
    analytics INTEGER NOT NULL,         -- Enable Analytics (Graphs of activity, submissions, etc.) BOOL
    enable_whitelist INTEGER NOT NULL   -- Whether to use the whitelist for this class
);

-- MAX 12 UNITS                                     
-- endpoint: get_class_data and get_unit_data
CREATE TABLE units (
    id INTEGER PRIMARY KEY,
    class_hash TEXT NOT NULL,       -- user_hash:time.time()
    unit_hash TEXT NOT NULL,        -- class_hash:user_hash:time.time()

    unit_name TEXT NOT NULL,        -- Ex: Unit #8 Polynomials
    locked INTEGER NOT NULL         -- whether the students can access this unit
);

-- MAX (20 * unit_count) LESSONS
-- endpoint: get_unit_lessons
CREATE TABLE lessons (
    id INTEGER PRIMARY KEY,
    unit_hash TEXT NOT NULL,        -- class_hash:user_hash:time.time()
    lesson_data TEXT NOT NULL       -- homework json map that contains: title, description, video, homework_image, homework_questions, homework_solutions
);
-- Homework Submissions for a specific Unit
-- endpoint: get_unit_submissions
CREATE TABLE submissions (
    id INTEGER PRIMARY KEY,
    user_hash TEXT NOT NULL,                -- The users email sha256 encrypted
    class_hash TEXT NOT NULL,               -- class_hash:user_hash:time.time()

    submitter_hash TEXT NOT NULL,           -- The student's user hash (use this to get the user's name, email, etc.)
    submission_date INTEGER NOT NULL,       -- The time since epoch when the user submitted the work

    data TEXT NOT NULL                      -- Homework file (AUTO CONVERT TO PDF)
);
