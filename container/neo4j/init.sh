#!/bin/bash

neo4j-admin dbms set-initial-password "${DB_PWD}"
neo4j --verbose start
sleep 15
cypher-shell -u neo4j -p ${DB_PWD} < import/01_create.cypher
cypher-shell -u neo4j -p ${DB_PWD} < import/02_data.cypher

tail -f /dev/null