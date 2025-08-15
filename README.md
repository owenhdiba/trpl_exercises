This repository contains solutions to some of the coding exercisees suggested in
*The Rust Programming Language*. 

## Included Crates

1. `vec_med_mode`

    Solution to an exercise at end of chapter 8:

    *'Given a list of integers, use a vector and return the median (when sorted, the
    value in the middle position) and mode (the value that occurs most often; a hash
    map will be helpful here) of the list.'*
    
2. `pig_latin`
    
    Solution to an exercise at end of chapter 8:

    *'Convert strings to pig latin. The first consonant of each word is moved to the
    end of the word and ay is added, so first becomes irst-fay. Words that start
    with a vowel have hay added to the end instead (apple becomes apple-hay). Keep
    in mind the details about UTF-8 encoding!'*

3. `add_employee`
    
    Solution to an exercise at end of chapter 8:

    *'Using a hash map and vectors, create a text interface to allow a user to add
    employee names to a department in a company; for example, “Add Sally to
    Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of
    all people in a department or all people in the company by department,
    sorted alphabetically.'*

4. `minigrep`
    
    This is the crate that Chapter 12 of TRPL develops - *'a version of the
    classic command line search tool grep.'*

    It is mostly the same as the code given in the book other than how case
    insensitive search is selected - I use a CLI argument rather than an
    environment variable.

