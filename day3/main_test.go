package main

import (
	"testing"
)

func Test_find12(t *testing.T) {
	tests := []struct {
		input  string
		output int
	}{
		{
			"987654321111111",
			987654321111,
		},
		{
			"811111111111119",
			811111111119,
		},
		{
			"234234234234278",
			434234234278,
		},
		{
			"818181911112111",
			888911112111,
		},
	}
	for _, test := range tests {
		actual := findLargestTwelveDigitNumber(test.input)
		if test.output != actual {
			t.Error("bad output", actual)
		}
	}
}
