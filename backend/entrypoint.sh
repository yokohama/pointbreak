#!/bin/bash

echo "#### entrypoint.sh start #####"

echo "Current directory: $(pwd)"
echo "Listing current directory contents:"
ls -l

diesel migration run

echo "#### entrypoint.sh end   #####"

exec "$@"
