FROM ubuntu

RUN echo "Building taskscore-app image"
ENV TS_DATABASE_ADDRESS=taskscore-neo4j:7687
ENV TS_DATABASE_PRINCIPAL=tsadmin
ENV TS_DATABASE_PASSWORD=G3n3ricPwd

RUN echo "export TS_DATABASE_ADDRESS=${TS_DATABASE_ADDRESS}" >> ~/.profile
RUN echo "export TS_DATABASE_PRINCIPAL=${TS_DATABASE_PRINCIPAL}" >> ~/.profile
RUN echo "export TS_DATABASE_PASSWORD=${TS_DATABASE_PASSWORD}" >> ~/.profile
RUN echo "export RUST_BACKTRACE=1" >> ~/.profile

COPY TaskScore /usr/bin
EXPOSE 8000

#CMD ["tail", "-f", "/dev/null"]
CMD ["/usr/bin/TaskScore"]