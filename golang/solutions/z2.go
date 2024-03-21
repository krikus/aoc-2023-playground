package solutions

import (
	"aoc/utils"
	"fmt"
	"math"
	"strconv"
	"strings"
)

type gameData struct {
	red   int
	green int
	blue  int
}

type gameInfo struct {
	id    int
	games []gameData
}

func parseGameInfo(line string) gameInfo {
	arr := strings.Split(line, ":")
	head := strings.Split(arr[0], " ")
	id, _ := strconv.Atoi(head[1])

	games := make([]gameData, 0)

	for _, game := range strings.Split(arr[1], ";") {
		colors := strings.Split(game, ",")
		gd := gameData{}
		for _, color := range colors {
			color = strings.Trim(color, " ")
			data := strings.Split(color, " ")
			switch data[1] {
			case "red":
				gd.red, _ = strconv.Atoi(data[0])
				continue
			case "blue":
				gd.blue, _ = strconv.Atoi(data[0])
				continue
			case "green":
				gd.green, _ = strconv.Atoi(data[0])
				continue
			}
		}
		games = append(games, gd)
	}

	return gameInfo{
		id:    id,
		games: games,
	}
}

func isValidGame(data []gameData) bool {
	limit := gameData{red: 12, green: 13, blue: 14}
	for _, game := range data {
		if game.blue > limit.blue || game.red > limit.red || game.green > limit.green {
			return false
		}
	}
	return true
}

func getMaxValue(data []gameData) gameData {
	result := gameData{}
	for _, game := range data {
		result.red = int(math.Max(float64(game.red), float64(result.red)))
		result.green = int(math.Max(float64(game.green), float64(result.green)))
		result.blue = int(math.Max(float64(game.blue), float64(result.blue)))
	}
	return result
}

func Solve2() {
	data := utils.ReadLines("../inputs/z2.txt")
	sum := 0
	pow := 0
	for _, line := range data {
		game := parseGameInfo(line)

		if isValidGame(game.games) {
			sum = game.id + sum
		}

		max := getMaxValue(game.games)
		pow += max.blue * max.green * max.red
	}

	fmt.Printf("Result#1 %d\n", sum)
	fmt.Printf("Result#2 %d\n", pow)
}
