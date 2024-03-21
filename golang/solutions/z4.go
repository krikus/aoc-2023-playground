package solutions

import (
	"aoc/utils"
	"fmt"
	"math"
	"sort"
	"strconv"
	"strings"
)

type scratchCard struct {
	id      int
	numbers []int
	winning []int
	matched int
	copies  int
}

func (c *scratchCard) updateMatched() {

	sort.Slice(c.numbers[0:], func(i, j int) bool {
		return c.numbers[i] < c.numbers[j]
	})

	sort.Slice(c.winning[0:], func(i, j int) bool {
		return c.winning[i] < c.winning[j]
	})

	num := 0
	for _, v := range c.numbers {
		_, found := sort.Find(len(c.winning), func(i int) int {
			return v - c.winning[i]
		})
		if found {
			num += 1
		}
	}
	c.matched = num
}

func parseScratchCard(s string) *scratchCard {
	parts := strings.Split(s, ":")
	cardInfo := strings.Split(parts[0], " ")
	id, err := strconv.Atoi(cardInfo[len(cardInfo)-1])
	if err != nil {
		fmt.Printf("%+v", s)
		panic(err)
	}
	parsed := [2][]int{make([]int, 0), make([]int, 0)}

	for i, v := range strings.Split(parts[1], "|") {
		temp := strings.Trim(v, " ")
		for _, v2 := range strings.Split(temp, " ") {
			if v2 == "" || v2 == " " {
				continue
			}
			num, _ := strconv.Atoi(v2)
			parsed[i] = append(parsed[i], num)
		}
	}

	card := &scratchCard{
		id:      id,
		numbers: parsed[0],
		winning: parsed[1],
	}

	card.updateMatched()

	return card
}

func Solve4() {
	lines := utils.ReadLines("../inputs/z4.txt")
	cards := make([]*scratchCard, 0)
	total := 0
	for _, v := range lines {
		card := parseScratchCard(v)
		cards = append(cards, card)
		if card.matched > 0 {
			total += int(math.Pow(2, float64(card.matched-1)))
		}
	}

	for i, c := range cards {
		for k := 1; k <= c.matched; k++ {
			cards[k+i].copies += 1 + c.copies
		}
	}

	total2 := 0
	for _, c := range cards {
		total2 += c.copies
		total2++
	}

	fmt.Printf("Result#1 %d\n", total)
	fmt.Printf("Result#2 %d\n", total2)
}
