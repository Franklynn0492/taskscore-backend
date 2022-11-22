#!/bin/bash

echo "Running images..."
docker run -d --name taskscore-neo4j --publish=7687:7687 --volume=$HOME/neo4j/data:/data --env NEO4JLABS_PLUGINS='["apoc"]' --env dbms.directories.import="/" --network="taskscore-network" --network-alias=taskscore-neo4j taskscore-neo4j:latest
sleep 30
docker run -d --name taskscore-app --publish=8000:8000 --network="taskscore-network" --network-alias=taskscore-app taskscore-app:latest
echo "Done running images."
