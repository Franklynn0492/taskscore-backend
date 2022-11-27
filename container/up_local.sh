#!/bin/bash
target="${1:-debug}"

cp -f ../target/release/TaskScore app/
cp -f .env.local .env
docker compose rm
docker compose --project-name="taskscore" up -d
