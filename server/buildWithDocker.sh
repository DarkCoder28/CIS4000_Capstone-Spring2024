#!/bin/bash
rm ./recipe-book.tar
./build.sh
docker build -t recipe-book:latest .
docker image save -o recipe-book.tar recipe-book:latest
docker image rm recipe-book:latest