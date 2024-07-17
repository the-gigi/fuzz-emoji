package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"

	"github.com/enescakir/emoji"
)

type FuzzEmoji struct {
	emojiDict map[string]string
}

func NewFuzzEmoji() *FuzzEmoji {
	f := &FuzzEmoji{
		emojiDict: make(map[string]string),
	}
	for name, e := range emoji.Map() {
		name := strings.Trim(name, ":")
		f.emojiDict[strings.ToLower(name)] = e
	}
	return f
}

func (f *FuzzEmoji) getSynonyms(word string) ([]string, error) {
	url := fmt.Sprintf("https://api.datamuse.com/words?rel_syn=%s", word)
	resp, err := http.Get(url)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("failed to fetch synonyms: %s", resp.Status)
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}

	var words []struct {
		Word string `json:"word"`
	}
	if err := json.Unmarshal(body, &words); err != nil {
		return nil, err
	}

	synonyms := make([]string, len(words))
	for i, wordData := range words {
		synonyms[i] = wordData.Word
	}

	return synonyms, nil
}

func (f *FuzzEmoji) getEmoji(description string) (string, string) {
	description = strings.ToLower(description)

	// direct match
	if emojiChar, exists := f.emojiDict[description]; exists {
		return description, emojiChar
	}

	// Subset match
	for name, emojiChar := range f.emojiDict {
		if strings.Contains(name, description) {
			return name, emojiChar
		}
	}

	synonyms, err := f.getSynonyms(description)
	if err != nil {
		return "", ""
	}

	// Synonym match
	for _, syn := range synonyms {
		if emojiChar, exists := f.emojiDict[syn]; exists {
			return syn, emojiChar
		}
	}

	// Subset match
	for name, emojiChar := range f.emojiDict {
		for _, syn := range synonyms {
			if strings.Contains(name, syn) {
				return syn, emojiChar
			}
		}
	}

	return "", ""
}

func (f *FuzzEmoji) getEmojis(descriptions []string) map[string]string {
	result := make(map[string]string)
	for _, d := range descriptions {
		name, emojiChar := f.getEmoji(d)
		result[d] = fmt.Sprintf("%s, %s", name, emojiChar)
	}
	return result
}

func main() {
	descriptions := []string{"flame", "perplexed", "dolla"}
	fuzzer := NewFuzzEmoji()
	result := fuzzer.getEmojis(descriptions)
	for k, v := range result {
		fmt.Printf("%s: (%s)\n", k, v)
	}
}
