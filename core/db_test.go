package core

import (
	"fmt"
	"os"
	"path/filepath"
	"testing"

	"github.com/quantanotes/heisenberg/common"
)

func TestHeisenberg(t *testing.T) {
	wd, _ := os.Getwd()
	path := filepath.Join(wd, "/.tmp")
	db := NewDB(path)
	defer db.Close()
	err := db.NewCollection("c", 3, common.Cosine)
	if err != nil {
		panic(err)
	}
	err = db.Put("c", "k", []float32{1, 2, 3}, map[string]interface{}{"msg": "bruh"})
	if err != nil {
		panic(err)
	}
	v, err := db.Get("c", "k")
	if err != nil {
		panic(err)
	}
	fmt.Println(v.Value.Meta)
}
