import { FuzzEmoji } from './fuzz-emoji.ts';

async function main() {
  const descriptions = ['flame', 'confused', 'dog'];
  const fuzzer = new FuzzEmoji();
  const result = await fuzzer.getEmojis(descriptions);
  for (const [k, v] of Object.entries(result)) {
    console.log(`${k}: (${v})`);
  }
}

main().catch(console.error);