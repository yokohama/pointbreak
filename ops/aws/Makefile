PROFILE=yokohama
CLOUDWATCH_LOG_GROUP=/aws/apprunner/pointbreak/b42ade55f9204e8288b623defc15a00d

aws-logs-service:
	aws logs tail $(CLOUDWATCH_LOG_GROUP)/service --follow --profile $(PROFILE)

aws-logs-app:
	aws logs tail $(CLOUDWATCH_LOG_GROUP)/application --follow --profile $(PROFILE)
