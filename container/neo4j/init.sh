#!/bin/bash

neo4j-admin dbms set-initial-password "${DB_PWD}"
neo4j --verbose start
SEMAPHORE_FILE="/var/semaphore"

if [ ! -f "$SEMAPHORE_FILE" ]; then
    sleep 15
    cypher-shell -u neo4j -p ${DB_PWD} < import/01_create.cypher
    cypher-shell -u neo4j -p ${DB_PWD} < import/02_data.cypher
    touch "$SEMAPHORE_FILE"
fi

tail -f /dev/null