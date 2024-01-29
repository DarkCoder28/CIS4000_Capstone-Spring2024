FROM alpine:3
LABEL com.aidensheeran.recipe-book.authors="agsheeran@duck.com"
EXPOSE 3000/tcp
ENV MONGODB_URI="mongodb://mongodbserver:27017"
ENV PK_ID="your.domain.xyz"
ENV PK_ORIGIN="https://your.domain.xyz:443"
ENV INSECURE=
WORKDIR /srv/recipe-book/
COPY ./recipe-book /srv/recipe-book/
ENTRYPOINT ["/srv/recipe-book/recipe-book"]