#!/bin/sh

sqlx migrate run

exec "$@"
