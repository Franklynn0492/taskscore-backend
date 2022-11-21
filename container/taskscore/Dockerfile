FROM alpine

ARG TARGET=release

RUN echo "Building image with target ${TARGET}"
COPY ../../target/${TARGET}/TaskScore /usr/bin
EXPOSE 8000

CMD ["/usr/bin/TaskScore"]