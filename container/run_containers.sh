#!/bin/bash

echo "Running images..."
docker run -d --name taskscore-neo4j --publish=7687:7687 --volume=$HOME/neo4j/data:/data --env NEO4JLABS_PLUGINS='["apoc"]' --env dbms.directories.import="/" taskscore-neo4j:latest
echo "Done running images."
