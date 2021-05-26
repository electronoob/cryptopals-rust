#!/bin/bash 
COMMAND="hex2str"
while true
do
    watch -n1 -d -t -g "sha1sum ${COMMAND}.rs"
    rustc ./${COMMAND}.rs && ./${COMMAND}
    sleep 5
done