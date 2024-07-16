# Setup Virtual Environment

```
python -m venv ./venv
```

# Activate virtual environment

```
source venv/bin/activate
```

# Install dependencies

```
pip install -r requirements.txt
```

# Usage

```
❯ python fuzz_emoji.py
flame ('fire', '🔥')
confused ('confused_face', '😕')
green ('evergreen_tree', '🌲')
```


# AWS Lambda function

The `aws_lambda` directories shows how to package the `get_emojis()` function as an AWS lambda
function

