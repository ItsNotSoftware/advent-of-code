package lib

import "fmt"

func MakeMat[T any](sizeL, sizeC int) [][]T {
	mat := make([][]T, sizeL)
	for i := 0; i < sizeL; i++ {
		mat[i] = make([]T, sizeC)
	}
	return mat
}

func PrintMat[T any](mat [][]T) {
	for _, row := range mat {
		fmt.Println(row)
	}
	fmt.Println()
}

func PrintBoolMat(mat [][]bool) {
	for _, row := range mat {
		for _, b := range row {
			if b {
				fmt.Print("#")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}
	fmt.Println()
}

func PrintRuneMat(mat [][]rune) {
	fmt.Print("   ")
	for i := range len(mat[0]) {
		fmt.Printf("%d", i%10)
	}
	fmt.Println()

	for i, row := range mat {
		fmt.Printf("%02d ", i)
		for _, cell := range row {
			fmt.Printf("%c", cell)
		}
		fmt.Println()
	}
	fmt.Println()
}

func OrBoolMats(dest, other *[][]bool) {
	for i := range *other {
		for j := range (*other)[i] {
			(*dest)[i][j] = (*dest)[i][j] || (*other)[i][j]
		}
	}
}

func InBoundsMat[T any](mat [][]T, l, c int) bool {
	return l >= 0 && l < len(mat) && c >= 0 && c < len(mat[0])
}

func InBoundsArray[T any](arr []T, i int) bool {
	return i >= 0 && i < len(arr)
}

func MatCount[T comparable](mat [][]T, value T) int {
	count := 0
	for _, row := range mat {
		for _, cell := range row {
			if cell == value {
				count++
			}
		}
	}
	return count
}

func ArrayCount[T comparable](arr []T, value T) int {
	count := 0
	for _, cell := range arr {
		if cell == value {
			count++
		}
	}
	return count
}
