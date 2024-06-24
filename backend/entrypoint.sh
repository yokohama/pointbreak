#!/bin/bash

echo "#### entrypoint.sh start #####"

echo "Current directory: $(pwd)"
echo "Listing current directory contents:"

echo "#### /app                #####"
ls -l

echo "#### /app/migrations     #####"
ls -l migrations/

echo "#### /app/src/schema.rs  #####"
ls -l src/schema.rs

echo "#### which diesel        #####"
which diesel

cd /app

diesel migration run

echo "#### entrypoint.sh end   #####"

exec "$@"
