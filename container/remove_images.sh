#!/bin/bash
bash ./remove_containers.sh

echo "Removing images..."
docker image rm taskscore_app
docker image rm taskscore_neo4j
docker image rm taskscore_nginx
echo "Done removing images."