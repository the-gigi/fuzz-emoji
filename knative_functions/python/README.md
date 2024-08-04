# Create function

```
func create -l python get-emojis
```

This command generates a directory with multiple files

```
❯ ls -laGh get-emojis          
total 72
drwxr-xr-x@ 12 gigi  staff   384B Jul 21 15:56 .
drwxr-xr-x@  4 gigi  staff   128B Jul 21 15:56 ..
drwxr-xr-x@  3 gigi  staff    96B Jul 21 15:56 .func
-rw-r--r--@  1 gigi  staff   217B Jul 21 15:56 .funcignore
-rw-r--r--@  1 gigi  staff   235B Jul 21 15:56 .gitignore
-rw-r--r--@  1 gigi  staff    28B Jul 21 15:56 Procfile
-rw-r--r--@  1 gigi  staff   862B Jul 21 15:56 README.md
-rwxr-xr-x@  1 gigi  staff    55B Jul 21 15:56 app.sh
-rw-r--r--@  1 gigi  staff   1.7K Jul 21 15:56 func.py
-rw-r--r--@  1 gigi  staff    95B Jul 21 15:56 func.yaml
-rw-r--r--@  1 gigi  staff    28B Jul 21 15:56 requirements.txt
-rw-r--r--@  1 gigi  staff   258B Jul 21 15:56 test_func.py
```

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
