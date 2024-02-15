#!/bin/bash
rm ./gwynedd-valley.tar
docker image rm gwynedd-valley:latest
./build.sh
docker build -t gwynedd-valley:latest .
docker image save -o gwynedd-valley.tar gwynedd-valley:latest
docker image rm gwynedd-valley:latest