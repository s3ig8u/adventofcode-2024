package main

import (
	"fmt"
	"os"
	"time"

	"github.com/s3ig8u/adventofcode-2024/day1"
	"github.com/s3ig8u/adventofcode-2024/day2"
)

func printResult(day int, result [2]int, elapsed time.Duration) {
	fmt.Printf("Day %d - Part 1: %d\n", day, result[0])
	fmt.Printf("Day %d - Part 2: %d\n", day, result[1])
	fmt.Printf("Execution time for day %d: %dms\n", day, elapsed.Milliseconds())
}

func printDivider(day int) {
	fmt.Printf("########################## Day %d ##########################\n", day)
}

func readInputFile(day int) string {
	content, err := os.ReadFile(fmt.Sprintf("../input/day%d.txt", day))
	if err != nil {
		panic(err)
	}
	return string(content)
}

func main() {
	printDivider(1)
	start := time.Now()
	content := readInputFile(1)
	result1Part1, result1Part2 := day1.Day1(content)
	printResult(1, [2]int{result1Part1, result1Part2}, time.Since(start))

	printDivider(2)
	start = time.Now()
	content = readInputFile(2)
	result2Part1, result2Part2 := day2.Day2(content)
	printResult(2, [2]int{result2Part1, result2Part2}, time.Since(start))
}
