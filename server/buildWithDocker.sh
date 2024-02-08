#!/bin/bash
rm ./cis4000.tar
./build.sh
docker build -t cis4000:latest .
docker image save -o cis4000.tar cis4000:latest
docker image rm cis4000:latest