tmux clear-history; 
clear; 
if . build.sh; then
	printf "${green}Successfully built.  Hosting on: http://127.0.0.1:8001${normal}\n"
	http www -q
else
	printf "${red}Failed to build.  Not running web server${normal}\n"
fi
