# Month converter quiz

## Description

**What does it do?**

I made this to practice reading months as numbers and knowing which month it correlates to, it is a quiz that will give you a number (0..=12) and you will have to answer which month it is. I also made this to learn Rust and learn to write TUI programs, and this is my first project for both Rust and TUIs!

## Installation

It is a requirement to have Cargo to build and run this project.

**Archlinux**
```
$ doas pacman -S rustup
$ rustup default stable
$ git clone git@github.com:richa3816/month-converter-quiz
$ cd month-converter-quiz
$ cargo run
```

**WSL**
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ rustup default stable
$ git clone git@github.com:richa3816/month-converter-quiz
$ cd month-converter-quiz
$ cargo run
```

**Windows**

Download `rustup-init.exe` from https://rustup.rs and install rustup.
Open PowerShell as administrator and type `rustup default stable`.
Open PowerShell as a regular user:
```
> git clone git@github.com:richa3816/month-converter-quiz
> cd month-converter-quiz
> cargo run
```

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

# Known issues (I'm using this as a TODO list basically, don't mind this)

- The UI file is potentially too bulky so may need to be organised into more files
- Add more modes beyond pride mode
- Adding `<C-BS>` and `<C-w>` for word-deletion Functionality
- Testing with better colors and layouts
- Adding a red background to "WRONG!" text
- Adding unit tests and instructions for testing
- Adding git as a listed dependency
- Compartmentalise the installation instructions per step rather than per OS
- Add a TOC to the README.md
- Add image tutorials after changing the statusbar
