#!/bin/bash
bash ./stop_containers.sh

echo "Starting containers..."
docker start taskscore-neo4j
echo "Waiting for database to start..."
sleep 30
docker start taskscore-app
echo "Waiting for application to start..."
sleep 15
docker start taskscore-swagger
docker start taskscore-nginx
sleep 5
docker ps
echo "Done starting containers."
