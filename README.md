# class.io ![Stars](https://img.shields.io/github/stars/realTristan/class.io?color=brightgreen) ![Watchers](https://img.shields.io/github/watchers/realTristan/class.io?label=Watchers)

![Capture](https://user-images.githubusercontent.com/75189508/193428491-28348d48-5a33-4975-8dfa-9c339f8f9db4.PNG)

# About
<h3>Why Rust?</h3>

- I wanted to learn Rust
- Rust is fast, lightweight and great for low-memory usage.

<h3>Why Svelte and Tailwind CSS?</h3>

- I wanted to learn Tailwind and get better at Svelte
- Svelte is 30% faster than other frameworks.

# API Showcase
 
<h3>Current State</h3>

```json
{
    "class_hash": "e8bc5598c2f61d2c5e7f8ad1d447fd1ea6ad5020", 
    "class_name": "test_class_name", 
    "enable_whitelist": false,
    "rsl":false, 
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
<h3>Major</h3>

- Add a section to select the unit
- Add a section to submit homework answers
- Add a section for the teacher to mark homework answers

<h3> Implementations </h3>

- Implement once homework has been marked, it will email the student
- Implement auto convert png to pdf for homework submissions
- Implement google oauth login using firebase

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
