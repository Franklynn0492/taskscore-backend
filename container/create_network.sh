#!/bin/bash

echo "Creating network..."
docker network create -d bridge taskscore-network
echo "Done creating network."
