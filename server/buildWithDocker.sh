#!/bin/bash
rm ./gwynedd-valley.tar
docker image rm gwynedd-valley:latest
mkdir ./build_cache/
cp ../certs/* ./build_cache/
./build.sh
docker build -t gwynedd-valley:latest .
docker image save -o gwynedd-valley.tar gwynedd-valley:latest
rm -R ./build_cache/
docker image rm gwynedd-valley:latest