#!/bin/bash

docker compose down
./buildWithDocker.sh
docker image load -i ./gwynedd-valley.tar
docker compose up