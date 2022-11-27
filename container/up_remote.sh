#!/bin/bash

git pull
(cd .. && exec cargo build --release)
cp -f ../target/release/TaskScore app/
cp -f .env.remote .env
cp -f nginx/nginx.conf.remote nginx/nginx.conf
docker compose --project-name="taskscore" down --rmi all
docker compose --project-name="taskscore" build --no-cache
docker compose --project-name="taskscore" up -d