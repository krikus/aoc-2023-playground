package main

import (
	"aoc/solutions"
	"fmt"
	"os"
)

func main() {
	if len(os.Args) != 2 {
		panic("Expected arg with number of task to solve")
	}

	switch os.Args[1] {
	case "1":
		solutions.Solve1()
	case "2":
		solutions.Solve2()
	case "3":
		solutions.Solve3()
	case "4":
		solutions.Solve4()
	case "5":
		solutions.Solve5()
	case "6":
		solutions.Solve6()
	case "7":
		solutions.Solve7()
	default:
		panic(fmt.Sprintf("No such task as '%s'", os.Args[1]))
	}

}
