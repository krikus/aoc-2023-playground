package solutions

import (
	"aoc/utils"
	"fmt"
	"strings"
)

func lineToNums(line string) []int8 {
	nums := make([]int8, 0)
	for i := 0; i < len(line); i++ {
		if line[i] >= '0' && line[i] <= '9' {
			nums = append(nums, int8(line[i]-'0'))
		}
	}

	return nums
}

func convertToNum(str string) int8 {
	strNums := []string{"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"}

	for idx, matcher := range strNums {
		if len(str) >= len(matcher) && strings.EqualFold(matcher, str[0:len(matcher)]) {
			return int8(idx + 1)
		}
	}

	return 0
}

func lineToNumsWithString(line string) []int8 {
	nums := make([]int8, 0)
	for i := 0; i < len(line); i++ {
		if line[i] >= '0' && line[i] <= '9' {
			nums = append(nums, int8(line[i]-'0'))
		} else {
			num := convertToNum(line[i:])
			if num > 0 {
				nums = append(nums, num)
			}
		}
	}

	return nums

}

func firstAndLastNum(nums []int8) [2]int8 {
	firstAndLast := [2]int8(make([]int8, 2))
	firstAndLast[0] = nums[0]
	firstAndLast[1] = nums[len(nums)-1]
	return firstAndLast
}

func calculateFromArray(nums []int8) int32 {
	fl := firstAndLastNum(nums)
	num := int32(fl[0]*10 + fl[1])
	return num
}

func Solve1() {
	path := "../inputs/z1.txt"
	var result2 int32
	var result1 int32
	for _, line := range utils.ReadLines(path) {
		result1 = result1 + calculateFromArray(lineToNums(line))
		result2 = result2 + calculateFromArray(lineToNumsWithString(line))
	}
	fmt.Printf("Result#1 %v\n", result1)
	fmt.Printf("Result#2 %v\n", result2)
}
