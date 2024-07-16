import json
import logging
from fuzz_emoji import FuzzEmoji

logger = logging.getLogger()
logger.setLevel(logging.INFO)


def handler(event, context):
    try:
        logger.info("Received event: %s", event)

        # Parse the JSON body of the event
        body = json.loads(event.get("body", "{}"))
        logger.info("Parsed body: %s", body)

        descriptions = body.get("descriptions", [])
        logger.info("Descriptions: %s", descriptions)

        fuzz_emoji = FuzzEmoji()
        result = fuzz_emoji.get_emojis(descriptions)
        response = {
            'statusCode': 200,
            'body': json.dumps(result)
        }
    except Exception as e:
        response = {
            'statusCode': 500,
            'body': json.dumps({'error': str(e)})
        }

    return response
