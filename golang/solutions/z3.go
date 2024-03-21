package solutions

import (
	"aoc/utils"
	"fmt"
)

type mapa struct {
	data   []string
	height int
	width  int
}

func (m *mapa) Get(x int, y int) byte {
	if x < 0 || y < 0 {
		return 0
	}
	if y >= len(m.data) {
		return 0
	}

	if x >= len(m.data[y]) {
		return 0
	}

	return m.data[y][x]
}

func (m *mapa) SetAt(val byte, x int, y int) {
	line := m.data[y]
	line = line[0:x] + string(val) + line[x+1:]
	m.data[y] = line
}

func (m *mapa) GetNumberStartPosition(x int, y int) *point {
	var p *point
	if isNumber(m.Get(x, y)) {
		for {
			x--
			if !isNumber(m.Get(x, y)) {
				break
			}
		}
		p = &point{
			x: x + 1,
			y: y,
		}
	}
	return p
}

func (m *mapa) GetNumberAt(x int, y int) int {
	p := m.GetNumberStartPosition(x, y)
	if p == nil {
		return -1
	}

	num := 0
	for i := p.x; isNumber(m.Get(i, y)); i++ {
		num *= 10
		a := m.Get(i, y)
		num += int(a - '0')
	}

	return num
}

func (m *mapa) ClearNumberAt(x int, y int) {
	p := m.GetNumberStartPosition(x, y)
	if p == nil {
		return
	}
	for i := p.x; isNumber(m.Get(i, y)); i++ {
		m.SetAt('.', i, y)
	}
}

func newMapa(str []string) *mapa {
	m := mapa{
		data:   str,
		height: len(str),
		width:  len(str[0]),
	}

	return &m
}

func isNumber(b byte) bool {
	return b >= '0' && b <= '9'
}

func isSymbol(b byte) bool {
	return b != '.' && !isNumber(b)
}

type point struct {
	x int
	y int
}

func getNumbers(m *mapa, x int, y int) ([]int, []point) {
	numbers := make([]int, 0)
	usedPos := make(map[point]bool, 0)
	keys := make([]point, 0)

	for i := -1; i <= 1; i++ {
		for j := -1; j <= 1; j++ {
			posX := i + x
			posY := j + y
			if !(i == 0 && j == 0) {
				p := m.GetNumberStartPosition(posX, posY)
				if p != nil && usedPos[*p] == false {
					num := m.GetNumberAt(p.x, p.y)
					numbers = append(numbers, num)
					usedPos[*p] = true
					keys = append(keys, *p)
				}
			}
		}
	}

	return numbers, keys
}

func Solve3() {
	info := newMapa(utils.ReadLines("../inputs/z3.txt"))
	sum := 0
	gear := 0
	usedPosGlobal := make(map[point]bool, 0)
	for i := 0; i < info.width; i++ {
		for j := 0; j < info.height; j++ {
			if isSymbol(info.Get(i, j)) {
				nums, pos := getNumbers(info, i, j)
				for idx, val := range nums {
					if usedPosGlobal[pos[idx]] == false {
						sum += val
						usedPosGlobal[pos[idx]] = true
					}
				}
				if len(nums) == 2 && info.Get(i, j) == '*' {
					gear += nums[0] * nums[1]
				}
			}
		}
	}

	fmt.Printf("Result#1 %d\n", sum)
	fmt.Printf("Result#2 %d\n", gear)
}
