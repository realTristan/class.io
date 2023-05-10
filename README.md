# class.io ![Stars](https://img.shields.io/github/stars/realTristan/class.io?color=brightgreen) ![Watchers](https://img.shields.io/github/watchers/realTristan/class.io?label=Watchers)
![Capture](https://user-images.githubusercontent.com/75189508/193428491-28348d48-5a33-4975-8dfa-9c339f8f9db4.PNG)

# About
* This project was built primarily for the backend portion of this application. The frontend is just extra.
* I love writing backend applications and wanted to try doing it with Rust. I instantly fell inlove with not only the speed of Rust, but it's simplicity and security when writing code.
* The database uses sqlite by sqlx (https://github.com/launchbadge/sqlx/tree/main/examples/sqlite/todos)

# API Documentation
## Users
### Insert new user
```cpp
HTTP PUT /users/
HEADERS: {
    authorization: sha256(firebase_token)
    access_token: sha256("{bearer}:{time_in_seconds}:{secret_code}")
}
BODY: {
    user_name: String,
    email: String
}
```

### Update user data
```cpp
HTTP POST /users/{user_id}
HEADERS: {
    authorization: sha256(firebase_token)
    access_token: sha256("{bearer}:{time_in_seconds}:{secret_code}")
}
BODY: {
    user_name: String
}
```

### Get user data
```cpp
HTTP GET /users/{user_id}
HEADERS: {
    authorization: sha256(firebase_token)
    access_token: sha256("{bearer}:{time_in_seconds}:{secret_code}")
}
RESPONSE: {
    user_name: String,
    user_id: String
}
```

## Classes
### Get class data
```cpp
HTTP GET /class/{class_id}
HEADERS: {
    authorization: sha256(firebase_token)
    access_token: sha256("{bearer}:{time_in_seconds}:{secret_code}")
}
RESPONSE: {
    class_id: int,
    owner_id: int,
    class_name: String,
    enable_whitelist: bool,
    units: [
        unit_name: String,
        locked: bool,
        lessons: [
            title: String,
            description: String,
            video: String,
            work: String,
            work_solutions: String
        ]
    ],
    whitelist: [
        whitelisted_user_name: String,
        whitelisted_user_id: String
    ],
    announcements: [
        author_name: String,
        title: String,
        description: String,
        attachment: String
    ]
}
```

### Update class data
```cpp
HTTP POST /class/{class_id}
HEADERS: {
    authorization: sha256(firebase_token)
    access_token: sha256("{bearer}:{time_in_seconds}:{secret_code}")
}
BODY: {
    enable_whitelist: bool,
    class_name: String
}
```

### Create new class
```cpp
HTTP PUT /class/
HEADERS: {
    authorization: sha256(firebase_token)
    access_token: sha256("{bearer}:{time_in_seconds}:{secret_code}")
}
BODY: {
    class_name: String
}
```

## Units
### Create new unit
```cpp
HTTP PUT /class/{class_id}/units/
HEADERS: {
    authorization: sha256(firebase_token)
    access_token: sha256("{bearer}:{time_in_seconds}:{secret_code}")
}
BODY: {
    unit_name: String
}
```

### Delete a unit
```cpp
HTTP DELETE /class/{class_id}/units/{unit_id}/
HEADERS: {
    authorization: sha256(firebase_token)
    access_token: sha256("{bearer}:{time_in_seconds}:{secret_code}")
}
```

### Update a unit
```cpp
HTTP POST /class/{class_id}/units/"
HEADERS: {
    authorization: sha256(firebase_token)
    access_token: sha256("{bearer}:{time_in_seconds}:{secret_code}")
}
BODY: {
    unit_name: String,
    locked: bool
}
```

## Announcements
### Create new announcement
```cpp
HTTP PUT /class/{class_id}/announcements/
HEADERS: {
    authorization: sha256(firebase_token)
    access_token: sha256("{bearer}:{time_in_seconds}:{secret_code}")
}
BODY: {
    author_name: String,
    title: String,
    description: String,
    attachment: String
}
```

### Delete an announcement
```cpp
HTTP DELETE /class/{class_id}/announcements/{announcement_id}
HEADERS: {
    authorization: sha256(firebase_token)
    access_token: sha256("{bearer}:{time_in_seconds}:{secret_code}")
}
```

## Submissions
### Get class submissions
```cpp
HTTP GET /class/{class_id}/submissions/
HEADERS: {
    authorization: sha256(firebase_token)
    access_token: sha256("{bearer}:{time_in_seconds}:{secret_code}")
}
RESPONSE: {
    submissions: [
        submitter_bearer: String,
        submission_id: String,
        submission_date: int,
        data: String
    ]
}
```

### Get user submissions
```cpp
HTTP GET /class/{class_id}/student/submissions/
HEADERS: {
    authorization: sha256(firebase_token)
    access_token: sha256("{bearer}:{time_in_seconds}:{secret_code}")
}
RESPONSE: {
    submissions: [
        submitter_bearer: String,
        submission_id: String,
        submission_date: int,
        data: String
    ]
}
```

### Create new submission
```cpp
HTTP PUT /class/{class_id}/submissions/
HEADERS: {
    authorization: sha256(firebase_token)
    access_token: sha256("{bearer}:{time_in_seconds}:{secret_code}")
}
BODY: {
    data: String
}
```

### Delete a submission
```cpp
HTTP DELETE /class/{class_id}/submissions/{submission_id}/
HEADERS: {
    authorization: sha256(firebase_token)
    access_token: sha256("{bearer}:{time_in_seconds}:{secret_code}")
}
```

## Whitelist
### Add a student to class whitelist
```cpp
HTTP PUT /class/{class_id}/whitelist/
HEADERS: {
    authorization: sha256(firebase_token)
    access_token: sha256("{bearer}:{time_in_seconds}:{secret_code}")
}
BODY: {
    user_id: String
}
```

### Remove a student from class whitelist
```cpp
HTTP DELETE /class/{class_id}/whitelist/{user_id}/
HEADERS: {
    authorization: sha256(firebase_token)
    access_token: sha256("{bearer}:{time_in_seconds}:{secret_code}")
}
```

# API Showcase
 
<h3>Current State</h3>

```json
{
    "class_id": "e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020", 
    "class_name": "test_class_name", 
    "enable_whitelist": false,
    "units": [
        {
            "unit_name": "test_unit_name", 
            "locked": false, 
            "lessons": [
                {
                    "title": "test_lesson_title", 
                    "description":"test_lesson_desc", 
                    "video": "test_lesson_video", 
                    "work":"test_lesson_work", 
                    "work_solutions":"test_lesson_work_solutions"
                }
            ]
        }
    ], 
    "whitelist": [
        "test_whitelisted_user1"
    ], 
    "announcements": [
        {
            "author_name": "test_author_name", 
            "title": "test_title", 
            "description": "test_desc", 
            "attachment": "no_attachment"
        }
    ]
}
```

# Todo
### Frontend

- Add a section to select the unit
- Add a section to submit homework answers
- Add a section for the teacher to mark homework answers
- Implement once homework has been marked, it will email the student
- Implement auto convert png to pdf for homework submissions
- Implement google oauth login using firebase
- Encrypted google oauth token is used as the user_id

# License
MIT License

Copyright (c) 2022 Tristan Simpson

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
