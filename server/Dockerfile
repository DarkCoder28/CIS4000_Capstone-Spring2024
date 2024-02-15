FROM alpine:3
LABEL edu.gmercyu.gwynedd-valley.authors="agsheeran@duck.com"
EXPOSE 3000/tcp
ENV MONGODB_URI="mongodb://mongodbserver:27017"
WORKDIR /srv/gwynedd-valley/
COPY ./gwynedd-valley /srv/gwynedd-valley/
ENTRYPOINT ["/srv/gwynedd-valley/gwynedd-valley"]