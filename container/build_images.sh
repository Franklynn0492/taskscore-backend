#!/bin/bash
target="${1:-debug}"
echo "Copying taskscore binary from target ${target} to app directory"
cp -f ../target/${target}/TaskScore app/

docker build --tag taskscore_neo4j ./neo4j
docker build --tag taskscore_app ./app
docker build --tag taskscore_nginx ./nginx
