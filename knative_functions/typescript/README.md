# Create function

```
func create -l typescript get-emojis-ts
```

This command generates a directory with multiple files

```
❯ ls -laGh get-emojis-ts
total 496
drwxr-xr-x@ 14 gigi  staff   448B Aug 17 13:00 .
drwxr-xr-x   4 gigi  staff   128B Aug 17 13:01 ..
-rw-r--r--@  1 gigi  staff   458B Aug 17 13:00 .eslintrc
drwxr-xr-x@  3 gigi  staff    96B Aug 17 13:00 .func
-rw-r--r--@  1 gigi  staff   217B Aug 17 13:00 .funcignore
-rw-r--r--@  1 gigi  staff   235B Aug 17 13:00 .gitignore
-rw-r--r--@  1 gigi  staff    90B Aug 17 13:00 .prettierrc
-rw-r--r--@  1 gigi  staff   5.1K Aug 17 13:00 README.md
-rw-r--r--@  1 gigi  staff   169B Aug 17 13:00 func.yaml
-rw-r--r--@  1 gigi  staff   209K Aug 17 13:00 package-lock.json
-rw-r--r--@  1 gigi  staff   1.3K Aug 17 13:00 package.json
drwxr-xr-x@  3 gigi  staff    96B Aug 17 13:00 src
drwxr-xr-x@  4 gigi  staff   128B Aug 17 13:00 test
-rw-r--r--@  1 gigi  staff   1.8K Aug 17 13:00 tsconfig.json
```

# Install the dependencies and build the function

```shell
cd get-emojis-ts
npm install
npm build
``` 

# Copy fuzz-emoji module

Due to the way Knative builds typescript functions into Docker images it is not possible to use a
symlink (as used in the Azure function). The fuzz-emoji module must be copied into the source tree.

```shell
(cd src && cp ../../../../typescript/fuzz-emoji.ts fuzz-emoji.ts)
```

# Add necessary dependencies

```shell
npm install emojilib axios
````

# Run the unit test

Knative generates a unit test and an integration test. You can run the tests like so:

```shell
npm run test
```

# Run the function locally

Type this (the generated README.md, which says `npm run local`, which is wrong):

```shell
func run start
```

```
❯ http -b 'http://localhost:8080/?descriptions=flame,face'
{
"face": "('angry_face', '😠')",
"flame": "('fire', '🔥')"
}
```

# Deploy the function to the cluster

```shell
func deploy --registry docker.io/g1g1

Warning: namespace chosen is 'default', but currently active namespace is 'knative-serving'.
Continuing with deployment to 'default'.
function up-to-date. Force rebuild with --build
Pushing function image to the registry "index.docker.io" using the "g1g1" user credentials
⬆️ Deploying function to the cluster
🎯 Creating Triggers on the cluster
✅ Function updated in namespace "default" and exposed at URL:
http://get-emojis.default.172.105.12.189.sslip.io

```

# Invoke the deployed function

```
❯ http -b 'http://get-emojis.default.172.105.12.189.sslip.io?descriptions=flame,lea,long'
{
"flame": "('fire', '🔥')",
"lea": "('fallen_leaf', '🍂')",
"long": "('long_drum', '🪘')"
}
```
