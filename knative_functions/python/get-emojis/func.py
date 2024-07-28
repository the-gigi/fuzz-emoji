import json

from parliament import Context
from fuzz_emoji import FuzzEmoji


def main(context: Context):
    descriptions = context.request.args.get('descriptions').split(',')
    fuzz_emoji = FuzzEmoji()
    result = fuzz_emoji.get_emojis(descriptions)
    return json.dumps(result, ensure_ascii=False), 200



