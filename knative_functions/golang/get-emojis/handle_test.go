package function

import (
	"context"
	"io"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestHandle(t *testing.T) {
	var (
		w   = httptest.NewRecorder()
		req = httptest.NewRequest(
			"GET",
			"http://example.com/test?descriptions=flame,dog",
			nil)
		res *http.Response
	)

	Handle(context.Background(), w, req)
	res = w.Result()
	defer res.Body.Close()

	data := make([]byte, 512)
	n, err := res.Body.Read(data)
	if err != nil && err != io.EOF {
		t.Fatal(err)
	}

	expected := "flame: {fire, 🔥}\ndog: {dog, 🐶}\n"
	result := string(data[:n])

	if expected != result {
		t.Fatalf("Failed to return the fire emoji")
	}

	if res.StatusCode != 200 {
		t.Fatalf("unexpected response code: %v", res.StatusCode)
	}
}
