import urllib2
import json
import sys

URL = "http://play.pushfour.net"

def get_board(game_id):
    f = urllib2.urlopen("%s/%s/%s" % (URL, 'game_details', game_id))
    return json.loads(f.read())['game']['game_detail']['xy']

def warmer_to_aromatt(board):
    chars = { 0: '-', 1: 'r', 2: 'b', 4: '#' }
    size = len(board)
    strrows = ['+ ' + ' '.join([str(i) for i in range(size)])]
    for i, row in enumerate(board):
        strrows.append(str(i) + ' ' + ' '.join([chars[c] for c in row]))
    return '\n'.join(strrows)

try:
    print warmer_to_aromatt(get_board(sys.argv[1]))
except Exception as e:
    print "Provide game ID as first argument"
    sys.exit(1)
