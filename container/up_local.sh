#!/bin/bash

target="debug"
skipTests=false

while getopts t:s flag
do
    case "${flag}" in
        t) target=${OPTARG};;
        s) skipTests=true;
    esac
done

if [ $skipTests = false ] ; then
    echo "Launching tests..."
    (cd .. && exec cargo test)

    if [ $? = 0 ] ; then
        echo "Tests succeeded, moving on..."
    else
        echo "Tests failed. Exiting."
        exit -1
    fi
else
    echo "Skipping tests"
fi


cp -f ../target/${target}/task_score app/
cp -f .env.local .env
cp -f nginx/nginx.conf.local nginx/nginx.conf
docker compose --project-name="taskscore" down --rmi all
docker compose --project-name="taskscore" build --no-cache
docker compose --project-name="taskscore" up -d
