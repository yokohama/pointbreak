#!/bin/bash
diesel migration run
exec "$@"
