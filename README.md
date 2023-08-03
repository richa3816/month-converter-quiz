# Month converter quiz

## Description

**What does it do?**

I made this to practice reading months as numbers and knowing which month it correlates to, it is a quiz that will give you a number (0..=12) and you will have to answer which month it is. I also made this to learn Rust and learn to write TUI programs, and this is my first project for both Rust and TUIs!

## Installation

### Dependencies

#### Git

Git is required to clone the repository if you do not wish to download it as a `.zip`.

**Archlinux**

Install git `doas pacman -S git`.

#### Cargo

Cargo is required to build the project and it's dependencies, producing an executable that you can run.

**Archlinux**

Install rustup, the recommended Rust toolchain manager `doas pacman -S rustup`.

Install the default, stable toolchain (includes Cargo) `rustup default stable`.

**WSL**

Install rustup, the recommended Rust toolchain manager `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.

**Windows**

Download `rustup-init.exe` from https://rustup.rs and install rustup. Once rustup is installed open PowerShell as an administrator and type `rustup default stable` to install the latest, stable Rust toolchain, then close PowerShell.

### Build instructions

Get the files locally through https `git clone https://github.com/richa3816/month-converter-quiz.git`.

Go into this directory `cd month-converter-quiz`.

Build the executable from the source code `cargo build`.

Run the executable automatically `cargo run -q` (the `-q` part is optional, but just makes it look a bit nicer).

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
| `<C-BS>` | Deletes last word |
| `<C-w>` | Deletes last word |

# Known issues (I'm using this as a TODO list basically, don't mind this)

[ ] The randomisation seems skewed, repeated numbers are surprisingly common and June is a rarely chosen month

[ ] The UI file is potentially too bulky so may need to be organised into more files

[ ] Add more modes beyond pride mode

[ ] Adding `<C-BS>` and `<C-w>` for word-deletion Functionality

[ ] Testing with better colors and layouts

[ ] Adding a red background to "WRONG!" text

[ ] Adding unit tests and instructions for testing

[ ] Adding git as a listed dependency

[ ] Add a TOC to the README.md

[ ] Add image tutorials after changing the statusbar
