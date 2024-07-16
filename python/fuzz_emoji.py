from typing import List, Mapping, Tuple

import emoji
import requests


class FuzzEmoji:
    def __init__(self):
        self.emoji_dict = {}
        emoji_list = {name: data for name, data in emoji.EMOJI_DATA.items() if 'en' in data}
        for emoji_char, data in emoji_list.items():
            name = data['en'].strip(':')
            self.emoji_dict[name.lower()] = emoji_char

    @staticmethod
    def get_synonyms(word):
        response = requests.get(f"https://api.datamuse.com/words?rel_syn={word}")
        if response.status_code == 200:
            synonyms = [word_data['word'] for word_data in response.json()]
            return synonyms

        raise RuntimeError(response.content)

    def get_emoji(self, description) -> Tuple[str, str]:
        description = description.lower()
        # direct match
        if description in self.emoji_dict:
            return description, self.emoji_dict[description]

        # Subset match
        for name in self.emoji_dict:
            if description in name:
                return name, self.emoji_dict[name]

        synonyms = self.get_synonyms(description)
        # Synonym match
        for syn in synonyms:
            if syn in self.emoji_dict:
                return syn, self.emoji_dict[syn]

        # Subset match
        for name in self.emoji_dict:
            for syn in synonyms:
                if syn in name:
                    return syn, self.emoji_dict[name]
        return '', ''

    def get_emojis(self, descriptions: List[str]) -> Mapping[str, str]:
        return {d: str(self.get_emoji(d)) for d in descriptions}


def main():
    descriptions = ["flame", "confused", "green", "Israel"]
    fuzzer = FuzzEmoji()
    result = fuzzer.get_emojis(descriptions)
    for k, v in result.items():
        print(k, v)


if __name__ == '__main__':
    main()
