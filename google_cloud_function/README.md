# Overview

This directory contains files and scripts to update the fuzz-emoji Google cloud function

## Pre-requisites

- GCP Project
- gcloud CLI
- Enable the cloudfunctions.googleapis.com API

Follow the instructions here:
https://cloud.google.com/functions/docs/create-deploy-gcloud

Make sure to follow the source directory structure:
https://cloud.google.com/functions/docs/writing#directory-structure

## Sample code

Sample code for Golang is here:
https://github.com/GoogleCloudPlatform/golang-samples/tree/main/functions/functionsv2

## Local testing the function framework

```shell
go run cmd/main.go
```

You invoke the function locally with the name it is registered as the path:

```shell
http -b 'http://localhost:8080/get-emojis?descriptions=flame,hot'

flame: {fire, 🔥}
hot: {love_hotel, 🏩}
```

## Build and Deploy to GCP

```shell
gcloud functions deploy get-emojis \
--gen2 \
--runtime=go121 \
--region=us-east1 \
--entry-point=get-emojis \
--trigger-http \
--allow-unauthenticated
```

This will process for a while and eventually spit out a URL like:

https://us-east1-playground-161404.cloudfunctions.net/get-emojis

## Invoke the function

Now, you can invoke it using the aforementioned URL and pass emoji descriptions

```
http -b 'https://us-east1-playground-161404.cloudfunctions.net/get-emojis?descriptions=flame,bla'    
flame: {fire, 🔥}
bla: {eight_pointed_black_star, ✴️}
```
