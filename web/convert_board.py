#!/usr/bin/env python

import json
import sys
import os

# use '-v' flag if the bot's color is blue

def get_board(fname):
    with open(fname) as f:
        return json.load(f)['game']['game_detail']['xy']

def warmer_to_aromatt(board):
    # TODO this is hack because my bot is hardcoded to play as red
    color = os.environ.get('PUSHFOUR_COLOR', 'red')
    if color == 'blue':
        chars = { 0: '-', 2: 'r', 1: 'b', 4: '#' }
    else:
        chars = { 0: '-', 1: 'r', 2: 'b', 4: '#' }
    size = len(board)
    strrows = ['+ ' + ' '.join([str(i) for i in range(size)])]
    for i, row in enumerate(board):
        strrows.append(str(i) + ' ' + ' '.join([chars[c] for c in row]))
    return '\n'.join(strrows)

try:
    print warmer_to_aromatt(get_board(sys.argv[1]))
except Exception as e:
    print e
    print "Provide file name as first argument"
    sys.exit(1)
