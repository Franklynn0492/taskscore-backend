FROM neo4j

ENV PWD=G3n3ricPwd

COPY 01_create.cypher /startup/
COPY 02_data.cypher /startup/

VOLUME "${HOME}"/neo4j/data:/data

ENV NEO4J_AUTH neo4j/"${PWD}"

RUN sed -i 's/#initial.dbms.default_database=neo4j/initial.dbms.default_database=taskscore/g' $NEO4J_HOME/conf/neo4j.conf

#RUN neo4j start
#RUN cypher-shell -u neo4j -p "${PWD}" -f /startup/01_create.cypher
#RUN cypher-shell -u neo4j -p "${PWD}" -f /startup/02_data.cypher

CMD ["neo4j"]