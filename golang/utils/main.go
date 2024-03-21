package utils

import (
	"bytes"
	"fmt"
	"os"
	"strings"
)

func ReadData(path string) string {
	buf, err := os.ReadFile(path)
	if err != nil {
		msg := fmt.Sprintf("utils#ReadData %v\n", err)
		panic(msg)
	}

	return bytes.NewBuffer(buf).String()
}

func ReadLines(path string) []string {
	data := ReadData(path)
	lines := strings.Split(data, "\n")

	linesFiltered := make([]string, 0)

	for _, line := range lines {
		if line != "" {
			linesFiltered = append(linesFiltered, line)
		}
	}

	return linesFiltered
}
