# Overview

This directory contains files and scripts to update the fuzz-emoji lambda function.

# Pre-requisites

- An Azure account
- azure CLI
- Azure functions core tools (`)


## Install Azure functions core tools

The brew link will fail because it tries to create a symlink called
`func`, which conflicts with Knative `func`.

The solution is to use a different link.
First temporarily move the Knative func symlink:

```
cd "$(brew --prefix )/bin"
mv func kn-func
```

Now, install the Azure functions core tools:

```
brew tap azure/functions
brew install azure-functions-core-tools@4
```

Now, rename the symlinks

```
mv func az-func
mv kn-func func
```

The end result is:

- Knative func is available as `func`
- Azure functions func is available as `az-func`



## Symlink the FuzzEmoji class

(cd src/functions &&  ln -s ../../../../typescript/FuzzEmoji.ts FuzzEmojiClass.ts)


## Create Azure function

```
az functionapp create -g the-group --consumption-plan-location eastus \
     --runtime node --runtime-version 18 --functions-version 4 \
     --name fuzz-emoji --storage-account sa14071789
```

Add this to the `tsconfig.json` file:

```
"esModuleInterop": true
```

This allows importing emojilib without resorting to `require`.

## Build

```shell
npm run build
```

## Test locally

Start the function locally:

```shell
az-func start

Azure Functions Core Tools
Core Tools Version:       4.0.5907 Commit hash: N/A +807e89766a92b14fd07b9f0bc2bea1d8777ab209 (64-bit)
Function Runtime Version: 4.834.3.22875

[2024-08-17T19:25:39.067Z] Worker process started and initialized.

Functions:

	fuzzEmoji: [GET,POST] http://localhost:7071/api/fuzzEmoji

For detailed output, run func with --verbose flag.
```

Then, access it with httpie:

```shell
http -b 'http://localhost:7071/api/fuzzEmoji?descriptions=flame,flo'

flame: (fire,🔥)
flo: (rolling_on_the_floor_laughing,🤣)
```

## Deploy

```shell
az-func azure functionapp publish fuzz-emoji

Getting site publishing info...
[2024-08-17T19:29:40.204Z] Starting the function app deployment...
Creating archive for current directory...
Uploading 2.46 MB [###############################################################################]
Upload completed successfully.
Deployment completed successfully.
[2024-08-17T19:30:00.730Z] Syncing triggers...
Functions in fuzz-emoji:
    fuzzEmoji - [httpTrigger]
        Invoke url: https://fuzz-emoji.azurewebsites.net/api/fuzzemoji
```

Test the deployed function:

```shell
http -b 'https://fuzz-emoji.azurewebsites.net/api/fuzzemoji?descriptions=flame,ice'

flame: (fire,🔥)
ice: (ice,🧊)
```

## Reference

https://learn.microsoft.com/en-us/azure/azure-functions/create-first-function-cli-typescript?tabs=macos%2Cazure-cli%2Cbrowser&pivots=nodejs-model-v4