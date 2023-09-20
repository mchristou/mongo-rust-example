#!/bin/bash

if [ $# -ne 1 ]; then
    echo "Usage: $0 <start|stop>"
    exit 1
fi

action="$1"
db_name="tododb"

if [ "$action" = "start" ]; then
    echo "Starting..."
      docker run -d -p 27017:27017 -v `pwd`/data/db:/data/db --name "${db_name}" mongo
    echo
      cargo run
elif [ "$action" = "stop" ]; then
    echo "Stopping..."
    docker stop "${db_name}" && docker rm "${db_name}"
else
    echo "Usage: $0 <start|stop>"
    exit 1
fi
