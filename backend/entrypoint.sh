#!/bin/sh

echo "##########"
echo "##########"
echo "##########"
echo "##########"
echo "##########"
echo "##########"
echo "##########"
echo $DATABASE_URL
sqlx migrate run
if [ $? -ne 0 ]; then
	echo "***********"
else
	echo "+++++++++++"
fi
echo "##########"
echo "##########"
echo "##########"
echo "##########"
echo "##########"
echo "##########"
echo "##########"
echo "##########"
echo "##########"

exec "$@"
