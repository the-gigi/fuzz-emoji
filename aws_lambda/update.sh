#!/usr/bin/env bash

IMAGE=fuzz-emoji:latest
BUILD_DIR="build"
REQUIREMENTS_FILE="../python/requirements.txt"
FUNCTION_FILE="../python/fuzz_emoji.py"

rm -rf $BUILD_DIR
mkdir -p $BUILD_DIR


cp Dockerfile "$BUILD_DIR"
cp lambda_function.py "$BUILD_DIR"
cp __init__.py "$BUILD_DIR"
cp "$FUNCTION_FILE" "$BUILD_DIR"
cp "$REQUIREMENTS_FILE" "$BUILD_DIR"

cd "$BUILD_DIR" || exit

docker build --platform linux/arm64 -t ${IMAGE} .

account=$(aws sts get-caller-identity --query Account --output text --profile private)
aws ecr get-login-password --profile private | docker login     \
  --username AWS                                                \
  --password-stdin "${account}.dkr.ecr.us-west-2.amazonaws.com"

REPO_NAME="fuzz-emoji"
REPO_URL=$(aws ecr describe-repositories --profile private                      \
          --query "repositories[?repositoryName=='${REPO_NAME}'].repositoryUri" \
          --output text)
TAG="${REPO_URL}:latest"
docker tag "$IMAGE" "$TAG"
docker push "$TAG"

# Update the lambda function
aws lambda update-function-code --function-name fuzz-emoji                   \
  --image-uri 422966116851.dkr.ecr.us-west-2.amazonaws.com/fuzz-emoji:latest \
  --profile private > /dev/null

# Clean up
rm -rf ../${BUILD_DIR}
