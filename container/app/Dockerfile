FROM ubuntu

RUN echo "Building taskscore-app image"
ENV TS_DATABASE_ADDRESS=taskscore-neo4j:7687
ENV TS_DATABASE_PRINCIPAL=tsadmin
ENV TS_DATABASE_PASSWORD=G3n3ricPwd
ENV RUST_BACKTRACE=1
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

COPY TaskScore /usr/bin

#CMD ["tail", "-f", "/dev/null"]
CMD ["/usr/bin/TaskScore"]