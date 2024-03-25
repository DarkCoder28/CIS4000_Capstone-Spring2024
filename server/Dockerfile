FROM alpine:3
LABEL edu.gmercyu.gwynedd-valley.authors="agsheeran@duck.com"
RUN apk add --no-cache --upgrade bash
EXPOSE 3000/tcp
ENV MONGODB_URI="mongodb://mongodbserver:27017"
WORKDIR /srv/certs/
COPY ./build_cache/cert.pem /srv/certs/
COPY ./build_cache/server.key.pem /srv/certs/
WORKDIR /srv/gwynedd-valley/
COPY ./gwynedd-valley /srv/gwynedd-valley/
# CMD ["/bin/sh"]
ENV RUST_BACKTRACE=1
ENTRYPOINT ["/srv/gwynedd-valley/gwynedd-valley"]