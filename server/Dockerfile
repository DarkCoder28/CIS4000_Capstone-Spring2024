FROM alpine:3
LABEL com.aidensheeran.cis4000.authors="agsheeran@duck.com"
EXPOSE 3000/tcp
ENV MONGODB_URI="mongodb://mongodbserver:27017"
ENV PK_ID="your.domain.xyz"
ENV PK_ORIGIN="https://your.domain.xyz:443"
ENV INSECURE=
WORKDIR /srv/cis4000/
COPY ./cis4000-project /srv/cis4000/
ENTRYPOINT ["/srv/cis4000/cis4000-project"]