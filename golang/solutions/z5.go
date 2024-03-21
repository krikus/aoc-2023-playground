package solutions

import (
	"aoc/utils"
	"fmt"
	"sort"
	"strconv"
	"strings"
)

const MaxUint = ^uint(0)

type abmapper struct {
	src   uint
	dst   uint
	count uint
}

type bigMapper struct {
	mapper     []*abmapper
	nextMapper *bigMapper
}

func reverseMappers(prev *bigMapper, first *bigMapper) *bigMapper {
	if first == nil {
		return prev
	}
	first.reverseValues()
	tmp := first.nextMapper
	first.nextMapper = prev

	return reverseMappers(first, tmp)
}

func (b *bigMapper) reverseValues() {
	for _, a := range b.mapper {
		a.src, a.dst = a.dst, a.src
	}
	sort.Slice(b.mapper, func(i int, j int) bool {
		return b.mapper[i].dst >= b.mapper[j].dst
	})
}

func (b *bigMapper) pushValues(src uint, dst uint, count uint) {
	b.mapper = append(b.mapper, &abmapper{
		src,
		dst,
		count,
	})
	sort.Slice(b.mapper, func(i int, j int) bool {
		return b.mapper[i].dst >= b.mapper[j].dst
	})
}

func (b *bigMapper) find(src uint) uint {
	for b.nextMapper != nil {

		var newsrc uint

		index := sort.Search(len(b.mapper), func(i int) bool {
			return b.mapper[i].dst <= src
		})
		if index >= len(b.mapper) || !b.mapper[index].hasDst(src) {
			newsrc = src
		} else {
			newsrc = b.mapper[index].getDst(src)
		}

		src = newsrc
		b = b.nextMapper
	}
	return src
}

func newBigMapper() *bigMapper {
	return &bigMapper{
		mapper: make([]*abmapper, 0),
	}
}

func (m *abmapper) hasDst(src uint) bool {
	return m.dst <= src && m.dst+m.count > src
}

func (m *abmapper) getDst(src uint) uint {
	shift := src - m.dst
	return m.src + shift
}

func Solve5() {
	lines := utils.ReadLines("../inputs/z5-debug.txt")

	var first *bigMapper
	var last *bigMapper
	var cur *bigMapper
	seedsStr := strings.Split(lines[0], " ")
	seeds := make([]uint, 0, len(seedsStr)-1)
	for _, seed := range seedsStr {
		seed, err := strconv.Atoi(seed)
		if err == nil {
			seeds = append(seeds, uint(seed))
		}
	}

	for _, line := range lines[1:] {
		if strings.Contains(line, ":") {
			last = newBigMapper()
			if cur == nil {
				first = last
				cur = first
			} else {
				cur.nextMapper = last
				cur = last
			}
			continue
		}
		numsStr := strings.Split(line, " ")
		if len(numsStr) != 3 {
			continue
		}
		src, _ := strconv.Atoi(numsStr[0])
		dst, _ := strconv.Atoi(numsStr[1])
		count, _ := strconv.Atoi(numsStr[2])
		cur.pushValues(uint(src), uint(dst), uint(count))
	}

	var min uint = ^uint(0)
	for _, seed := range seeds {
		newMin := first.find(seed)
		if newMin < min {
			min = newMin
		}
	}
	fmt.Printf("Result#1: %d\n", min)

	fmt.Println("Reversing...")
	first = reverseMappers(nil, first)
	fmt.Println("Done")

	for _, x := range first.mapper {
		fmt.Printf("looking for mapper %+v\n", x)
		for i := x.dst; i < x.dst+x.count; i++ {
			seed := first.find(i)
			fmt.Printf("\t-Checking seed %d\n", seed)
			for j := 0; j < len(seeds); j += 2 {
				start := seeds[0]
				stop := seeds[0] + seeds[1]
				if seed >= start && seed < stop {
					fmt.Printf("Result#2: %d (seed = %d)\n", i, seed)
					return
				}
			}
		}
	}
	panic("NO ANSWER")

}
