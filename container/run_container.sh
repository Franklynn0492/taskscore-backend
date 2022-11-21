#!/bin/bash

docker run -d --name taskscore-neo4j --publish=7687:7687 --volume=$HOME/neo4j/data:/data taskscore-neo4j:latest