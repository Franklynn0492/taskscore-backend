FROM neo4j

ENV PWD=D3f4ultPwd

COPY 01_create.cypher /var/lib/neo4j/import/
COPY 02_data.cypher /var/lib/neo4j/import/
COPY apoc.conf /var/lib/neo4j/conf/

VOLUME $HOME/neo4j/data:/data

ENV NEO4J_AUTH neo4j/$PWD

RUN sed -i 's/#initial.dbms.default_database=neo4j/initial.dbms.default_database=taskscore/g' $NEO4J_HOME/conf/neo4j.conf
RUN apt-get update && apt-get install -y net-tools

CMD ["neo4j"]
