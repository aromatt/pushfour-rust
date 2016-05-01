#!/usr/bin/env python

# Output HTTP query param string for a Move, including the game_id and api_key.

# The first argument must be the game_id.
# The second argument must be a string containing a line as output by Move's
# debug format; e.g. "Move { row: Y, col: X, player: PLAYER }"

# Example usage:
#
#     ./web/encode_move.py 42 'Move { row: 4, col: 4 }'

import fileinput
import re
import sys
import os
from os.path import expanduser, join
import json
import urllib

URL = "http://play.pushfour.net"

conf_path = os.environ.get('PUSHFOUR_CONF', join(expanduser('~'), '.pushfour.json'))
conf = json.load(open(conf_path))

game_id = sys.argv[1]
with open(sys.argv[2]) as f:
    move_str = f.read()

row, col = re.findall(r"Move { row: (\d+), col: (\d+)", move_str)[0]
post_params = { 'x': col, 'y': row, 'game_id': game_id, 'api_key': conf['api_key'] }

print urllib.urlencode(post_params)
