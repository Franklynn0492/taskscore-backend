#!/bin/bash
bash ./remove_containers.sh

echo "Removing images..."
docker image rm taskscore-app
docker image rm taskscore-neo4j
echo "Done removing images."