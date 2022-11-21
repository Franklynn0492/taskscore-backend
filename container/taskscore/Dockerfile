FROM alpine

ARG TARGET=release

RUN echo "Building image with target ${TARGET}"
COPY ../../target/${TARGET}/TaskScore /usr/bin
EXPOSE

CMD ["/usr/bin/TaskScore"]