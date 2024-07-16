PROFILE=yokohama
CLOUDWATCH_LOG_GROUP=/aws/apprunner/pointbreak/b42ade55f9204e8288b623defc15a00d

migration-run:
	sqlx migrate run --database-url postgres://postgres:password@localhost:/pointbreak

migration-revert:
	sqlx migrate revert --database-url postgres://postgres:password@localhost:/pointbreak

psql:
	PGPASSWORD=password psql -h localhost -p 5432 -U postgres -d pointbreak

aws-logs-service:
	aws logs tail $(CLOUDWATCH_LOG_GROUP)/service --follow --profile $(PROFILE)

aws-logs-app:
	aws logs tail $(CLOUDWATCH_LOG_GROUP)/application --follow --profile $(PROFILE)

curl-registration-user:
	curl -X POST -H "Content-Type: application/json" -d '{ "email":"hoge2@example.com", "password":"passpass" }' http://localhost:3000/user/registration

curl-create-session-user:
	curl -s -X POST -H 'Content-Type: application/json' -d '{"email":"hoge1@example.com","password":"passpass"}' http://localhost:3000/user/session/new

curl-protected-user:
	curl -s -H 'Content-Type: application/json' -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIyIiwiY3JlYXRlZF9hdCI6IjIwMjQtMDctMTZUMDM6Mjc6NDMuNzc4MDc0IiwiZXhwIjoyMDAwMDAwMDAwfQ.Sboy2Jh8DAW36MfXsVcaW7f6x0WBeZ_9f0OpmWuhYeg' http://localhost:3000/user/dashboard

curl-create-session-admin:
	curl -s -X POST -H 'Content-Type: application/json' -d '{"email":"hoge2@example.com","password":"passpass"}' http://localhost:3000/admin/session/new

curl-protected-admin:
	curl -s -H 'Content-Type: application/json' -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIzIiwiY3JlYXRlZF9hdCI6IjIwMjQtMDctMTZUMDM6Mjk6MTEuNDQ2MzE3IiwiZXhwIjoyMDAwMDAwMDAwfQ.ILRXO4CR8CWRB2rwo-XVedjyMGzsZ3qcMItMRNV8qtI' http://localhost:3000/admin/users
