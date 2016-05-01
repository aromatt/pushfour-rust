This directory contains tooling to allow the Rust bot to play online at warmer's [pushfour website](http://play.pushfour.net). Info about the API is [here](http://play.pushfour.net/about).

Your bot must be registered at the site and you must place the account's api_key into a JSON file
located at ~/.pushfour.conf (or alternatively, located at `$PUSHFOUR_CONF`). Example config:

    { "api_key": "deadbeef123456789..." }

## Workflow
To play a game, we'll use the `run-scenarios` program using this hacky, brittle workflow:

    GET board -> convert board -> run-scenario -> parse/convert move -> POST move

## Instructions
First, create a game using the web UI, and note the game id and your bot's color (first player is
currently always red).

Because the Rust bot always thinks it's red, we'll invert the colors of the board if we're
supposed to be blue by setting `PUSHFOUR_COLOR=blue`.

To have the bot play a turn, run the following:

    $ PUSHFOUR_COLOR=YOUR_COLOR ./web/play_game.sh DEPTH GAME_ID

# TODO
- Automate the bot (including polling for active games)
- Allow specification of bot color to `run-scenario` to obviate the color-inversion hack
- Use FFI (or just implement the game-playing client in Rust) instead of gluing things together with shell scripts
