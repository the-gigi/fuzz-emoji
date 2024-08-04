package main

import (
	"fmt"
	"github.com/the-gigi/fuzz-emoji/pkg/fuzz_emoji"
)

func main() {
	descriptions := []string{"flame", "perplexed", "dolla"}
	fuzzer := fuzz_emoji.NewFuzzEmoji()
	result := fuzzer.GetEmojis(descriptions)
	for k, v := range result {
		fmt.Printf("%s: (%s)\n", k, v)
	}
}
