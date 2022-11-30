#!/bin/bash

skipTests=false

while getopts s flag
do
    case "${flag}" in
        s) skipTests=true;
    esac
done

git pull

if [ $skipTests = false ] ; then
    echo "Launching tests..."
    (cd .. && exec cargo test --release)

    if [ $? = 0 ] ; then
        echo "Tests succeeded, moving on..."
    else
        echo "Tests failed. Exiting."
        exit -1
    fi
else
    echo "Skipping tests"
fi

(cd .. && exec cargo build --release)
cp -f ../target/release/task_score app/
cp -f .env.remote .env
cp -f nginx/nginx.conf.remote nginx/nginx.conf
docker compose --project-name="taskscore" down --rmi all
docker compose --project-name="taskscore" build --no-cache
docker compose --project-name="taskscore" up -d
