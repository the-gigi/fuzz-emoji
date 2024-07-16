This directory contains files and scripts to update the fuzz-emoji lambda function.  

# The original lambda function

The original lambda function was created using click-ops in the AWS console with 
a Gateway API HTTP trigger.

# Create ECR repo

Do this one time.

```
account=$(aws sts get-caller-identity --query Account --output text --profile private)
aws ecr get-login-password --profile private | docker login     \
  --username AWS                                                \
  --password-stdin "${account}.dkr.ecr.us-west-2.amazonaws.com"

aws ecr create-repository --repository-name fuzz-emoji \
  --profile private                                    \
  --image-scanning-configuration scanOnPush=true       \
  --image-tag-mutability MUTABLE
```

# Update function

Whenever you update the code run this command:

```
./update.sh
```

# Test locally

```
docker run --platform linux/arm64 -p 9000:8080 fuzz-emoji:latest lambda_function.handler
```

Then, using httpie and jq

```
http POST http://localhost:9000/2015-03-31/functions/function/invocations descriptions:='["flame", "Israel", confused"]' | jq -r '.body | fromjson'

{
  "flame": "🔥",
  "confused": "😕"
}
```

# Testing the lambda function


