package day1

import (
	"math"
	"sort"
	"strconv"
	"strings"
)

func get_columns(content string) ([]int, []int) {
	lines := strings.Split(content, "\n")
	n := len(lines)
	column1 := make([]int, 0, n) // allocate slice with capacity n
	column2 := make([]int, 0, n) // allocate slice with capacity n, this is more efficient than appending to a slice without a capacity

	for _, line := range lines {
		idx := strings.IndexByte(line, ' ')
		if idx == -1 {
			continue
		}

		val1, _ := strconv.Atoi(line[:idx])                                // convert string to int
		val2, _ := strconv.Atoi(line[strings.LastIndexByte(line, ' ')+1:]) // convert string to int
		column1 = append(column1, val1)                                    // append to slice
		column2 = append(column2, val2)                                    // append to slice
	}

	return column1, column2
}

func Day1_Part1(content string) int {
	column1, column2 := get_columns(content)
	sort.Ints(column1) // sort slice
	sort.Ints(column2) // sort slice

	sum := 0
	for i := range column1 {
		sum += int(math.Abs(float64(column1[i] - column2[i])))
	}

	return sum
}

func Day1_Part2(content string) int {
	column1, column2 := get_columns(content)
	countMap := make(map[int]int)

	for _, val := range column2 {
		countMap[val] += 1
	}

	sum := 0
	for _, val := range column1 {
		if _, exists := countMap[val]; exists {
			sum += val * countMap[val]
		}
	}

	return sum
}
