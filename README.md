# git-report
A tool to give an overview of git repositories in Rust

## How to use

`git-report summarize`: Prints the current number of lines written by each user.

## Current development

Features:
- [ ] Summarize number of lines written by each user.
    - [x] Get all files.
    - [x] Run git blame for each file.
    - [x] Summarize information.
    - [ ] Disply in a nice way.
- [ ] Weighting by the importance of the file (by imports, methods being called).
    - [ ] Write a regex to get what files each file is importing.
        - [ ] Support to Rust.
        - [ ] Support to Python.
    - [ ] Warn if there are star imports, but count then as being used for now.
    - [ ] Summarize how much each file was imported.
    - [ ] Calculate the importance factor.
- [ ] Measure mutation of each file (commit counts).
    - [ ] Get commits by file with timestamp.
    - [ ] Calculate the mutation factor.
