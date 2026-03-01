#!/usr/bin/env nu

def main [file: string, --no-record(-n)] {
    let timestamp = (date now | format date "%Y%m%d_%H%M%S")
    let stem = ($file | path basename | path parse | get stem)
    let cast_file = $"($stem)_($timestamp).cast"
    let session = $"sheets_($stem)"

    # snapshot on open - "here is what I received"
    git-sheets snapshot $file -m $"session open: ($timestamp)"

    # start tmux session
    tmux new-session -d -s $session -x 220 -y 50

    # left pane - grid view
    tmux send-keys -t $session $"tabiew ($file)" Enter

    # right pane - nu repl
    tmux split-window -t $session -h
    tmux send-keys -t $session $"open ($file)" Enter

    # status bar showing file and session
    tmux set-option -t $session status-left $" ($file) | ($timestamp) "

    # start recording unless --no-record
    if not $no_record {
        tmux new-window -t $session -n "record"
        tmux send-keys -t $session $"asciinema rec ($cast_file)" Enter
        tmux select-window -t $"session:0"
    }

    # attach
    tmux attach-session -t $session
}
