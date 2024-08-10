package function

import (
	"context"
	"fmt"
	"net/http"
	"strings"

	"function/pkg/fuzz_emoji"
)

func Handle(ctx context.Context, res http.ResponseWriter, req *http.Request) {
	descriptions := strings.Split(req.URL.Query().Get("descriptions"), ",")
	fuzzer := fuzz_emoji.NewFuzzEmoji()
	result := fuzzer.GetEmojis(descriptions)
	for k, v := range result {
		_, _ = fmt.Fprintf(res, "%s: {%v}\n", k, v)
	}
}
