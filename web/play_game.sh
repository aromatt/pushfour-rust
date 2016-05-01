#!/bin/bash

url=http://play.pushfour.net

depth=$1
game_id=$2

topdir=$(cd $(dirname $0)/.. && pwd)
game_dir=$topdir/.games/$game_id

convert_board=$topdir/web/convert_board.py
run_scenario=$topdir/target/release/run-scenario
encode_move=$topdir/web/encode_move.py

cd $topdir
cargo build --release

mkdir -p $game_dir

# Fetch and convert current board state
curl -sL $url/game_details/$game_id > $game_dir/tmp.state
$convert_board $game_dir/tmp.state > $game_dir/tmp.board

# Exit unless the state has changed
if [ -f $game_dir/last.board ]; then
  diff $game_dir/last.board $game_dir/tmp.board >/dev/null
  # TODO verify this actually works
  if [[ $? -eq 0 ]]; then
    echo "No change"
    exit 0
  fi
fi

# Compute the next move
$run_scenario -d $depth $game_dir/tmp.board > $game_dir/tmp.move

# Encode and post the move
params=$($encode_move $game_id $game_dir/tmp.move)
post_url=$url/bot_move
curl -XPOST -sL $post_url -d "$params"

# Log the move
stamp=$(date -u +"%Y%m%d.%H%M%S")
if [[ $? -eq 0 ]] ; then
  cd $game_dir
  for file in state board move; do
    mv tmp.$file $stamp.$file
    ln -sf $stamp.$file last.$file
  done
fi
