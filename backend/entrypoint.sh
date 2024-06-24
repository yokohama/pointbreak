#!/bin/bash

echo "#### entrypoint.sh start #####"

echo "Current directory: $(pwd)"
echo "Listing current directory contents:"

echo "#### /app                #####"
ls -l

echo "#### /app/migrations     #####"
ls migrations/

echo "#### which diesel        #####"
which diesel

cd /app

diesel migration run

echo "#### entrypoint.sh end   #####"

exec "$@"
