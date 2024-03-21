package solutions

import (
	"fmt"
	"math"
)

func doTheMath(time int, distance int) int {
	delta := float64(time*time - 4*distance)
	if delta < 0 {
		return 0
	}
	if delta == 1 {
		panic("DELTA=1")
	}
	sd := math.Sqrt(delta)

	x1 := (float64(-time) + sd) / -2.0
	x2 := (float64(-time) - sd) / -2.0
	start := math.Floor(x1)
	stop := math.Ceil(x2)

	return int(stop - start - 1)

}

func linearAdd(nums []int) int {
	total := 0
	p10 := 1
	for i := len(nums) - 1; i >= 0; i-- {

		total += nums[i] * p10
		ln := math.Ceil(math.Log10(float64(total)))
		p10 = int(math.Pow10(int(ln)))
		// fmt.Printf("total = %d / ln = %f / p10 = %d\n", total, ln, p10)
	}

	return total
}

func Solve6() {
	// time := []int{7, 15, 30}
	// distance := []int{9, 40, 200}
	time := []int{56, 97, 77, 93}
	distance := []int{499, 2210, 1097, 1440}

	pow := 1
	for i, t := range time {
		d := distance[i]
		ways := doTheMath(t, d)
		// fmt.Printf("Solution for race #%d = %d\n", i+1, ways)
		pow *= ways
	}

	fmt.Printf("Result#1 %d\n", pow)

	t := linearAdd(time)
	d := linearAdd(distance)
	fmt.Printf("Result#2: %d\n", doTheMath(t, d))
}
