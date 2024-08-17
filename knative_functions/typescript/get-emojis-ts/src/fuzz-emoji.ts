import emojilib from 'emojilib';
import axios from 'axios';

export class FuzzEmoji {
  private emojiDict: { [key: string]: string } = {};

  constructor() {
    // Check if emojilib is undefined
    if (!emojilib) {
      throw new Error('emojilib is not defined or imported correctly');
    }

    // Use emojilib to build the emoji dictionary
    for (const [emojiChar, keywords] of Object.entries(emojilib)) {
      if (keywords.length > 0) {
        // Use only the first keyword
        const firstKeyword = keywords[0];
        this.emojiDict[firstKeyword.toLowerCase()] = emojiChar;
      }
    }
  }

  private static async getSynonyms(word: string): Promise<string[]> {
    try {
      const response = await axios.get(`https://api.datamuse.com/words?rel_syn=${word}`);
      if (response.status === 200) {
        return response.data.map((wordData: { word: string }) => wordData.word);
      }
    } catch (error) {
      if (axios.isAxiosError(error) && error.response) {
        throw new Error(error.response.data || 'Error fetching synonyms');
      }
      throw new Error('Error fetching synonyms');
    }
    return [];
  }

  public async getEmoji(description: string): Promise<[string, string]> {
    description = description.toLowerCase();

    // Direct match
    if (description in this.emojiDict) {
      return [description, this.emojiDict[description]];
    }

    // Subset match
    for (const name in this.emojiDict) {
      if (name.includes(description)) {
        return [name, this.emojiDict[name]];
      }
    }

    const synonyms = await FuzzEmoji.getSynonyms(description);
    // Synonym match
    for (const syn of synonyms) {
      if (syn in this.emojiDict) {
        return [syn, this.emojiDict[syn]];
      }
    }
    return ['', ''];
  }

  public async getEmojis(descriptions: string[]): Promise<{ [key: string]: string }> {
    const result: { [key: string]: string } = {};
    for (const description of descriptions) {
      const emojiPair = await this.getEmoji(description);
      result[description] = emojiPair.toString();
    }
    return result;
  }
}
