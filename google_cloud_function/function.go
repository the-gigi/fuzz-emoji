package google_cloud_function

import (
	"fmt"
	"github.com/GoogleCloudPlatform/functions-framework-go/functions"
	"github.com/the-gigi/get-emojis-google-cloud-function/pkg/fuzz_emoji"
	"net/http"
	"strings"
)

func init() {
	functions.HTTP("get-emojis", getEmojis)
}

func getEmojis(w http.ResponseWriter, r *http.Request) {
	descriptions := strings.Split(r.URL.Query().Get("descriptions"), ",")
	fuzzer := fuzz_emoji.NewFuzzEmoji()
	result := fuzzer.GetEmojis(descriptions)
	for k, v := range result {
		_, _ = fmt.Fprintf(w, "%s: {%v}\n", k, v)
	}
}
