# tactic

A terminal tic-tac-toe game with AI opponent, written in Rust.

Play tic-tac-toe (a.k.a. "noughts and crosses" or "Xs and Os") in the terminal against an AI opponent or another human player.

# Features

- Play against a friend, play against a computer, or pit two computer players against each other

- Configurable computer player difficulty 

- Configurable game ending settings

    - Best of x number of games

    - Best of x number of won (non-draw) games

    - First player to x score

    - Unlimited (play until deciding to quit)

- Reverse mode

    - Instead of playing to get three of your pieces in a row, try to force the opposing player to place three of their pieces in a row

# Installation instructions

1. Install the Rust programming language with [rustup](https://rustup.rs/)

2. Install tactic with `cargo install tactic`

# Development instructions

1. Install the Rust programming language with [rustup](https://rustup.rs/)

2. Clone the tactic repository locally 

    - e.g. `git clone https://github.com/generic-user1/tactic.git`

3. Enter the local copy of the repository

    - e.g. `cd tactic`

4. Make changes to source code as desired

5. Run project with `cargo run`

    - To improve game performance (especially with computer players), run in release mode
    
        - e.g. `cargo run --release`


## Dependencies

- [crossterm](https://github.com/crossterm-rs/crossterm)
- [rand](https://github.com/rust-random/rand)