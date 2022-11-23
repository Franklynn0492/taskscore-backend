#!/bin/bash
bash ./remove_containers.sh

echo "Removing images..."
docker image rm taskscore-app
docker image rm taskscore-neo4j
docker image rm taskscore-nginx
echo "Done removing images."