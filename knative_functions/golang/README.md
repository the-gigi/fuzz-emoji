# Create function

```
func create -l go get-emojis-go
```

This command generates a directory with multiple files

```
❯ ls -laGh get-emojis-go
total 56
drwxr-xr-x@ 10 gigi  staff   320B Aug  3 19:22 .
drwxr-xr-x@  4 gigi  staff   128B Aug  3 19:22 ..
drwxr-xr-x@  3 gigi  staff    96B Aug  3 19:22 .func
-rw-r--r--@  1 gigi  staff   217B Aug  3 19:22 .funcignore
-rw-r--r--@  1 gigi  staff   235B Aug  3 19:22 .gitignore
-rw-r--r--@  1 gigi  staff   611B Aug  3 19:22 README.md
-rw-r--r--@  1 gigi  staff    91B Aug  3 19:22 func.yaml
-rw-r--r--@  1 gigi  staff    25B Aug  3 19:22 go.mod
-rw-r--r--@  1 gigi  staff   854B Aug  3 19:22 handle.go
-rw-r--r--@  1 gigi  staff   539B Aug  3 19:22 handle_test.go
```

The `go.mod` file uses Go 1.14. I bumped it to 1.21

In func.yaml change the function name to `get-emojis-go` to avoid conflicts with the Python `get-emojis` function

# Run the unit test

Knative generates a unit test file. Unfortunately, it doesn't work out of the box.
The function expects a parliament Context object that includes a Flask request.
The original unit test simply passed an empty dict. I created a GitHub issue for the Knative func
developers:
https://github.com/knative/func/issues/2448

I modified the unit test to conform to the correct signature and added a mock request.

Now, you can run it with

```
python test_func.py

============================= test session starts ==============================
collecting ... collected 1 item

test_func.py::TestFunc::test_func 

============================== 1 passed in 0.79s ===============================
PASSED                                 [100%]
Process finished with exit code 0
```

# Build the function

```
func build --registry docker.io/g1g1

Building function image
Still building
Still building
Yes, still building
Don't give up on me
Still building
This is taking a while
🙌 Function built: docker.io/g1g1/get-emojis-go:latest
```

# Run the function locally


```
func run

function up-to-date. Force rebuild with --build
Running on host port 8080
Initializing HTTP function
listening on http port 8080
```

Check it out:

```
http -b localhost:8080

GET / HTTP/1.1 localhost:8080
  User-Agent: HTTPie/3.2.2
  Accept-Encoding: gzip, deflate
  Accept: */*
  Connection: keep-alive
```

# Deploy the function to the cluster



```
❯ func deploy

function up-to-date. Force rebuild with --build
Pushing function image to the registry "index.docker.io" using the "g1g1" user credentials
🎯 Creating Triggers on the cluster
✅ Function updated in namespace "knative-serving" and exposed at URL:
   http://get-emojis-go.knative-serving.172.105.12.189.sslip.io
```

# Invoke the deployed function (default implementation)

```
http -b http://get-emojis-go.knative-serving.172.105.12.189.sslip.io/

GET / HTTP/1.1 get-emojis-go.knative-serving.172.105.12.189.sslip.io
  User-Agent: HTTPie/3.2.2
  Forwarded: for=10.2.1.6;proto=http
  X-Forwarded-For: 10.2.1.6, 10.2.0.130
  Accept: */*
  Accept-Encoding: gzip, deflate
  K-Proxy-Request: activator
  X-Forwarded-Proto: http
  X-Request-Id: ffd25d2e-c805-4810-ba0e-c75b72cf57dd```
