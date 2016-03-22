# pushfour-rust [![Build Status](https://travis-ci.org/aromatt/pushfour-rust.svg?branch=master)](https://travis-ci.org/aromatt/pushfour-rust)
A pushfour bot in Rust. Uses minimax with alpha-beta pruning.

## Dependencies
This project currently depends on [Nightly Rust](https://doc.rust-lang.org/book/nightly-rust.html). If you're ok with executing arbitrary code from the internet, just run:

    $ curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly

## Run tests

    $ cargo test && ./baseline

## Play against the bot

    $ cargo run

Or:

    $ cargo build --release
    $ ./target/release/play-pushfour

## Run scenarios
There are game scenarios for debugging the bot's logic in `tests/scenarios/`. To run them:

    # Build the tool
    $ cargo build --release

    # Run scenarios
    $ ./target/release/run-scenario tests/scenarios/*
