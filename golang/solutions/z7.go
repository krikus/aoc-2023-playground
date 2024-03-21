package solutions

import (
	"aoc/utils"
	"fmt"
	"sort"
	"strconv"
	"strings"
)

type camelCard struct {
	cardsMap map[byte]int
	original string
	cards    []byte
	value    int
}

type camelGame struct {
	games    ([]*camelCard)
	ordered  map[byte]byte
	wildcard byte
}

func (a camelGame) Len() int {
	return len(a.games)
}

func (a camelGame) Swap(i, j int) {
	a.games[i], a.games[j] = a.games[j], a.games[i]
}

func isCardLess(a, b byte, ordered map[byte]byte) bool {
	av, ok := ordered[a]
	if !ok {
		av = a
	}
	bv, ok := ordered[b]
	if !ok {
		bv = b
	}
	//fmt.Printf("%s(%d) < %s(%d) = %v\n", string(a), av, string(b), bv, av < bv)
	return av < bv
}

func (a camelGame) Less(i, j int) bool {
	// check groups
	g1 := a.games[i]
	g2 := a.games[j]
	gr1 := g1.groups(a.wildcard)
	gr2 := g2.groups(a.wildcard)

	for i := 0; i < len(gr1) && i < len(gr2); i++ {
		if gr1[i] != gr2[i] {
			//fmt.Printf("%v < %v = %d, %v\n", (gr1), (gr2), i, gr1[i] < gr2[i])
			return gr1[i] < gr2[i]
		}
	}

	for i := 0; i < len(g1.cards); i++ {
		if g1.cards[i] != g2.cards[i] {
			return isCardLess(g1.cards[i], g2.cards[i], a.ordered)
		}
	}
	panic("WTF! - takie same karty?")
}

func (c *camelCard) groups(withWildcard byte) []int {
	groups := make([]int, 0)
	wild := 0
	for k, val := range c.cardsMap {
		if k == withWildcard {
			wild = val
		} else {
			groups = append(groups, val)
		}
	}
	groups = append(groups, 0)
	sort.Slice(groups, func(i, j int) bool {
		return !(groups[i] < groups[j])
	})
	groups[0] += wild
	return groups
}

func newCamelCard(s string, value int) *camelCard {
	letters := strings.Split(s, "")
	cardsMap := make(map[byte]int, 0)
	cards := make([]byte, 0, 5)

	for _, letter := range letters {
		bcard := byte(letter[0])
		cardsMap[bcard] = cardsMap[bcard] + 1
		cards = append(cards, bcard)
	}

	c := &camelCard{
		cardsMap: cardsMap,
		original: s,
		cards:    cards,
		value:    value,
	}

	return c
}

func debugCard(g camelCard, i int) {
	fmt.Printf("Adding rank #%d [%s] with value %d\n", i+1, g.original, g.value)
}

func Solve7() {
	// lines := utils.ReadLines("../inputs/z7.txt")
	lines := utils.ReadLines("../inputs/z7.txt")
	game := camelGame{
		games: make([]*camelCard, 0),
		ordered: map[byte]byte{
			'A': '9' + 10,
			'K': '9' + 9,
			'Q': '9' + 8,
			'J': '9' + 7,
			'T': '9' + 1,
		},
	}
	for _, line := range lines {
		s := strings.Split(line, " ")
		value, err := strconv.Atoi(s[1])
		if err != nil {
			panic(err)
		}
		game.games = append(game.games, newCamelCard(s[0], value))
	}

	sort.Sort(game)

	total := 0
	for i, g := range game.games {
		//debugCard(*g, i)
		total += g.value * (i + 1)
	}
	fmt.Printf("Result#1: %d\n", total)

	game.wildcard = 'J'
	game.ordered['J'] = 1

	sort.Sort(game)

	total2 := 0
	for i, g := range game.games {
		fmt.Printf("group[0] = %d ", g.groups('J')[0])
		debugCard(*g, i)

		total2 += g.value * (i + 1)
	}
	fmt.Printf("Result#1: %d\n", total2)
}
