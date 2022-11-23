#!/bin/bash
target="${1:-debug}"
echo "Copying taskscore binary from target ${target} to app directory"
cp -f ../target/${target}/TaskScore app/

docker build --tag taskscore-neo4j ./neo4j
docker build --tag taskscore-app ./app
docker build --tag taskscore-nginx ./nginx
