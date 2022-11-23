#!/bin/bash

host="${1:-localhost}"
port="${2:-8080}"

echo "Running containers..."
docker run -d --name taskscore-neo4j -p 7687:7687 --volume=$HOME/neo4j/data:/data --env NEO4JLABS_PLUGINS='["apoc"]' --env dbms.directories.import="/" --network="taskscore-network" --network-alias="taskscore-neo4j" taskscore-neo4j:latest
echo "Waiting for database to start..."
sleep 30
docker run -d --name taskscore-app --network="taskscore-network" --network-alias="taskscore-app" taskscore-app:latest
echo "Waiting for application to start..."
sleep 15
docker run -d --name taskscore-swagger -e BASE_URL=/TaskScore/swagger -e SWAGGER_JSON_URL=http://${host}:${port}/TaskScore/rest/openapi.json  --network="taskscore-network" --network-alias="taskscore-swagger" swaggerapi/swagger-ui
docker run -d --name taskscore-nginx -p 8080:8080 --network="taskscore-network" --network-alias="taskscore-nginx" taskscore-nginx:latest
sleep 5
echo "Done running containers."
docker ps
