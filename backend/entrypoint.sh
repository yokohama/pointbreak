#!/bin/sh

echo "#- entrypoint.sh -------#"

echo " - sqlx migrate run"
sqlx migrate run

echo "#----------------- end -#"

exec "$@"
