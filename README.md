# pushfour-rust [![Build Status](https://travis-ci.org/aromatt/pushfour-rust.svg?branch=master)](https://travis-ci.org/aromatt/pushfour-rust)
A pushfour bot in Rust. Uses minimax with alpha-beta pruning.

## What's Pushfour?
Connect Four, except pieces can be inserted from any edge. Boards are initialized
with random obstructions to make things interesting.

## Dependencies
This project currently depends on [Nightly Rust](https://doc.rust-lang.org/book/nightly-rust.html).
If you're ok with executing arbitrary code from the internet, just run:

    $ curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly

## Run tests

    $ cargo test && ./run-scenarios

## Play against the bot
Do either of the following (you'll be player 'b'; enter colon-separated `row:col` coordinates on
your turn):

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

Each scenario has its test-case-specific minimax depth embedded in its filename as `depth_N`.

You can specify a scenario file *without* this naming convention by adding `-d DEPTH` to the
invocation:

    $ ./target/release/run-scenario -d 5 some_scenario.txt

This can also be used to run one of the scenarios in `tests/scenarios/` with a different depth
than its filename dictates:

    # Run scenario foo_depth_8.txt at depth 5 instead of depth 8
    $ ./target/release/run-scenario -d 5 <(cat tests/scenarios/foo_depth_8.txt)
