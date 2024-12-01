package main

import (
	"fmt"
	"time"

	"github.com/s3ig8u/aoc-24/day1"
	"github.com/s3ig8u/aoc-24/utils"
)

func main() {
	day := 1
	content, err := utils.ReadInputFile(fmt.Sprintf("../input/day%d.txt", day))
	if err != nil {
		fmt.Printf("Error reading input file %d: %s\n", day, err)
		return
	}

	start := time.Now()
	fmt.Println("Day1 Part1:", day1.Day1_Part1(content))
	fmt.Println("Day1 Part2:", day1.Day1_Part2(content))
	elapsed := time.Since(start)
	fmt.Printf("Execution time for day %d: %dms\n", day, elapsed.Milliseconds())
}
