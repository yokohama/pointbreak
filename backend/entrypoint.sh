#!/bin/bash

echo "#### entrypoint.sh start #####"
diesel migration run
echo "#### entrypoint.sh end   #####"

exec "$@"
