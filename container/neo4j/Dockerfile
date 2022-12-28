FROM neo4j

ENV PWD=D3f4ultPwd

COPY 01_create.cypher /var/lib/neo4j/import/
COPY 02_data.cypher /var/lib/neo4j/import/
COPY init.sh ./

VOLUME $HOME/neo4j/data:/data

# Set the locale
RUN apt-get update
RUN apt-get -y install locales
RUN sed -i '/de_DE.UTF-8/s/^# //g' /etc/locale.gen && locale-gen
ENV LANG de_DE.UTF-8  
ENV LANGUAGE de_DE:de  
ENV LC_ALL de_DE.UTF-8   

ENV DB_PWD $PWD
ENV NEO4J_AUTH neo4j/$PWD

RUN sed -i 's/#initial.dbms.default_database=neo4j/initial.dbms.default_database=taskscore/g' $NEO4J_HOME/conf/neo4j.conf

RUN apt-get update && apt-get install -y net-tools

CMD ["/bin/bash", "./init.sh"]
#CMD ["tail", "-f", "/dev/null"]
