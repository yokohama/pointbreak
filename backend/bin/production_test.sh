URL="https://nps7ce8bkw.ap-northeast-1.awsapprunner.com"
EMAIL="hoge100@example.com"
PASSWORD="XYz1234&&&"

try() {
  echo "#-- $1"
}

next() {
  echo "\n"
  sleep 3
}

# Already created!
#try "POST user/registration"
#curl -X POST "$URL/user/registration" -H 'Content-Type: application/json' -d "{\"email\":\"$EMAIL\",\"password\":\"$PASSWORD\"}"
#next

try "POST user/session"
JWT=$(curl -s -X POST "$URL/user/session" -H 'Content-Type: application/json' -d "{\"email\":\"$EMAIL\",\"password\":\"$PASSWORD\"}" | jq -r '.access_token')
echo $JWT
if [ -z "$JWT" ]; then
    echo "Error: Failed to retrieve JWT token"
    exit 1
fi
next

try "GET user/dashboard"
curl -X GET "$URL/user/dashboard" -H 'Content-Type: application/json' -H "Authorization: Bearer $JWT"
next

try "POST user/point_conditions"
curl -X POST "$URL/user/point_conditions" -H 'Content-Type: application/json' -H "Authorization: Bearer $JWT" -d '{"end_date":"2024-08-13","lat":35.3741,"lon":140.3708,"start_date":"2024-08-13","timezone":"Asia/Tokyo"}'
next

try "GET user/point_conditions"
curl -X GET "$URL/user/point_conditions" -H 'Content-Type: application/json' -H "Authorization: Bearer $JWT"
next
