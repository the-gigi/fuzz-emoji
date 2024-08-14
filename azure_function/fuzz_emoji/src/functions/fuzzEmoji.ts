import { app, HttpRequest, HttpResponseInit, InvocationContext } from "@azure/functions";
import { FuzzEmoji } from './FuzzEmojiClass';


export async function fuzzEmoji(request: HttpRequest, context: InvocationContext): Promise<HttpResponseInit> {
    context.log(`Http function processed request for url "${request.url}"`);
    const descriptionsParam = request.query.get('descriptions');
    const descriptions = descriptionsParam.split(',');

    const fuzzer = new FuzzEmoji();
    const result = await fuzzer.getEmojis(descriptions);

    const body = Object.entries(result)
        .map(([k, v]) => `${k}: (${v})`)
        .join('\n');
    return { body };
};


app.http('fuzzEmoji', {
    methods: ['GET', 'POST'],
    authLevel: 'anonymous',
    handler: fuzzEmoji
});
