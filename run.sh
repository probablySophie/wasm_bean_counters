tmux clear-history; 
clear; 
. build.sh; 
# TODO: Don't run if build.sh went wrong somewhere
#python3 run_server.py;
http www -q
