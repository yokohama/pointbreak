#!/bin/bash

echo "#### entrypoint.sh start #####"

#cd /app
diesel migration run

echo "#### entrypoint.sh end   #####"

exec "$@"
