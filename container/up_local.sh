#!/bin/bash
target="${1:-debug}"

cp -f ../target/${target}/TaskScore app/
cp -f .env.local .env
cp -f nginx/nginx.conf.local nginx/nginx.conf
docker compose --project-name="taskscore" down --rmi all
docker compose --project-name="taskscore" build --no-cache
docker compose --project-name="taskscore" up -d
