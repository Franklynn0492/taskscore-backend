#!/bin/bash
bash ./stop_containers.sh

echo "Removing containers..."
docker rm taskscore-nginx
docker rm taskscore-swagger
docker rm taskscore-app
docker rm taskscore-neo4j
echo "Done removing containers."
