#!/bin/bash

echo "Running containers..."
docker run -d --name taskscore-neo4j --publish=7687:7687 --volume=$HOME/neo4j/data:/data --env NEO4JLABS_PLUGINS='["apoc"]' --env dbms.directories.import="/" --network="taskscore-network" --network-alias="taskscore-neo4j" taskscore-neo4j:latest
echo "Waiting for database to start..."
sleep 20
docker run -d --name taskscore-app --publish=8000:8000 --network="taskscore-network" --network-alias="taskscore-app" taskscore-app:latest
echo "Waiting for application to start..."
sleep 20
docker run -d --name taskscore-swagger -p 8080:8080 -e BASE_URL=/TaskScore/swagger -e SWAGGER_JSON_URL=http://taskscore-app:8000/TaskScore/rest/openapi.json  --network="taskscore-network" --network-alias="taskscore-swagger" swaggerapi/swagger-ui
sleep 5
echo "Done running containers."
docker ps
