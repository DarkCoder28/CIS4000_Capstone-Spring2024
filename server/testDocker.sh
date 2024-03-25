#!/bin/bash

docker compose down
# Clear Build Cache
docker buildx prune -f 
./buildWithDocker.sh
docker image load -i ./gwynedd-valley.tar
docker compose up --remove-orphans