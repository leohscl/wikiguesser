#!/bin/bash
cd ~/project/wikiguesser/
tmux new-session -s "wg" -d
cd ~/project/wikiguesser/backend
tmux new-session -s "ba" -d
tmux send -t ba:1 "nvim src/main.rs" C-m
tmux send-keys -t ba:1 Space e C-m
cd ~/project/wikiguesser/frontend
tmux new-session -s "fr" -d
tmux send -t fr:1 "nvim src/main.rs" C-m
tmux send-keys -t fr:1 Space e C-m

# Select window #1 and attach to the session
tmux select-window -t "wg:1"
tmux -2 attach-session -t "wg"
