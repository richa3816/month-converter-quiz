# Month converter quiz

## Description

**What does it do?**

I made this to practice reading months as numbers and knowing which month it correlates to, it is a quiz that will give you a number (0..=12) and you will have to answer which month it is. I also made this to learn Rust and learn to write TUI programs, and this is my first project for both Rust and TUIs!

## How to use

It functions similarly to Vim in that the default mode is the `Normal` mode. From here you can change the settings, or enter `Insert` mode to begin answering the questions.

Reading the status bar is easy too as it's all in plain text displaying the mode as the leftmost item, and the `pride_mode` option as the rightmost

### Keybindings

**Normal mode**

| Key | Action            |
| --- | ----------------- |
| `q` | Quit the program  |
| `i` | Enter insert mode |
| `p` | Toggle pride mode |

**In insert mode**

| Key    | Action          |
| ------ | --------------- |
| `Esc`  | Exit insert mode|
| `<CR>` | Submit answer   |
