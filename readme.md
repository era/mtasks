# MTask

A simple CLI to track the tasks as you do during your day, 
in order to remember them when you have to tell your co-workers what you did last day during standup.

The CLI was done while streaming at [twitch](https://www.twitch.tv/code_elias_code),
with @[karreiro](https://codeberg.org/karreiro).

## Decisions made while Streaming

- All tasks are saved in a daily file (e.g. tasks done at 2022-12-06 will be at the file 20221206).
- The files will be saved in the path: `$HOME/.mtasks/`
- In order to simplify error handling, we panic with an user-friendly error message
everytime something is not in the way we expect (e.g. user tries to list the tasks of a day which does not exist).
instead of using Result enum.

## Installing
- Just run the following:
```shell
cargo install --git https://codeberg.org/era/mtasks/
```