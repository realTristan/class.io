/*

    Users can visit the home endpoint to create their own website
    or
    They can visit the website their teacher provided them
    ex: site.com/bearer/class_id/<optional: unit_id>

*/
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    user_id TEXT NOT NULL,              -- Random id given to the user
    bearer TEXT NOT NULL,               -- The users firebase token sha256 encrypted

    user_name TEXT NOT NULL,            -- A name the user can change themselves
    email TEXT NOT NULL,                -- Used for emailing students about their homework
    registration_date INTEGER NOT NULL  -- The time since epoch format of when the user registered
);
CREATE TABLE announcements (
    id INTEGER PRIMARY KEY,
    owner_bearer TEXT NOT NULL,           -- used to verify bearer
    class_id TEXT NOT NULL,           -- bearer:time.time()
    announcement_id TEXT NOT NULL,    -- the announcements unique identifier

    author_name TEXT NOT NULL,          -- The Teacher's name
    title TEXT NOT NULL,                -- Announcement title
    description TEXT NOT NULL,          -- Announcement content
    attachment TEXT NOT NULL,           -- Base64 encoded images/video
    date INTEGER NOT NULL               -- The time since epoch format of when the post was made
);

-- MAX 100 USERS
-- endpoint: get_class_data
CREATE TABLE whitelists (
    id INTEGER PRIMARY KEY,
    owner_bearer TEXT NOT NULL,       -- used to verify bearer
    class_id TEXT NOT NULL,       -- bearer:time.time()

    whitelisted_user TEXT NOT NULL  -- The user to be whitelisted's email sha256 encrypted
);

-- MAX 10 CLASSES
-- endpoint: get_class_data
CREATE TABLE classes (
    id INTEGER PRIMARY KEY,
    owner_id TEXT NOT NULL,
    owner_bearer TEXT NOT NULL,           -- The users email sha256 encrypted
    class_id TEXT NOT NULL,           -- bearer:time.time()

    class_name TEXT NOT NULL,           -- Ex: MHF4UI
    enable_whitelist INTEGER NOT NULL   -- Whether to use the whitelist for this class
);

-- MAX 12 UNITS                                     
-- endpoint: get_class_data and get_unit_data
CREATE TABLE units (
    id INTEGER PRIMARY KEY,
    owner_bearer TEXT NOT NULL,       -- used to verify bearer
    class_id TEXT NOT NULL,       -- bearer:time.time()
    unit_id TEXT NOT NULL,        -- class_id:bearer:time.time()

    unit_name TEXT NOT NULL,        -- Ex: Unit #8 Polynomials
    locked INTEGER NOT NULL         -- whether the students can access this unit
);

-- MAX (20 * unit_count) LESSONS
-- endpoint: get_unit_lessons
CREATE TABLE lessons (
    id INTEGER PRIMARY KEY,
    owner_bearer TEXT NOT NULL,        -- used to verify bearer
    unit_id TEXT NOT NULL,         -- class_id:bearer:time.time()

    title TEXT NOT NULL,             -- Lesson Title
    description TEXT NOT NULL,       -- Lesson Description
    video TEXT NOT NULL,             -- Lesson Youtube Video
    work TEXT NOT NULL,              -- Lesson Work
    work_solutions TEXT NOT NULL     -- Lesson Work Solutions
);
-- Homework Submissions for a specific Unit
-- endpoint: get_unit_submissions
CREATE TABLE submissions (
    id INTEGER PRIMARY KEY,
    class_id TEXT NOT NULL,               -- class_id:bearer:time.time()
    submission_id TEXT NOT NULL,

    submitter_bearer TEXT NOT NULL,           -- The student's user hash (use this to get the user's name, email, etc.)
    submission_date INTEGER NOT NULL,       -- The time since epoch when the user submitted the work
    data TEXT NOT NULL                      -- Homework file (AUTO CONVERT TO PDF)
);