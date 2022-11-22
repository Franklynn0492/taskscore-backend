#!/bin/bash

echo "Stopping containers..."
docker stop taskscore-app
docker stop taskscore-neo4j
echo "Done stopping containers."
