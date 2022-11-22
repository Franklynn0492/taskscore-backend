#!/bin/bash
bash ./stop_containers.sh

echo "Removing containers..."
docker rm taskscore-neo4j
echo "Done removing containers."
