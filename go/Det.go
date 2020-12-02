package main

type stack []int

func (s *stack) isEmpty() bool {
	return len(*s) == 0
}
func (s *stack) push(n int) {
	*s = append(*s, n)
}
func (s *stack) pop() (int, bool) {
	if s.isEmpty() {
		return 0, false
	}
	i := len(*s) - 1
	n := (*s)[i]
	*s = (*s)[:i]
	return n, true
}
func (s *stack) ToSlice() []int {
	return *s
}

func Determinant(mat [][]int) int { //fonction récursive calculant le déterminant

	if len(mat) != len(mat[0]) {
		return 0
	}
	if len(mat) == 1 {
		return (mat[0][0])
	}
	if len(mat) == 2 {
		return (mat[0][0] * mat[1][1]) - (mat[0][1] * mat[1][0])
	}
	s := 0 // Determiannt final
	for i := 0; i < len(mat[0]); i++ {

		sm := subMat(mat[1:][:], i) // On enlève la première rangé a la matrice pour avoir une sous matrices
		z := Determinant(sm)        // On récupère le détermiannt de la sous matrice
		if i%2 != 0 {
			s -= mat[0][i] * z
		} else {
			s += mat[0][i] * z
		}
	}
	return s
}

func subMat(mat [][]int, p int) [][]int {
	stacks := make([]stack, len(mat))
	for n := range mat {
		stacks[n] = stack{}
		for j := range mat[n] {
			if j != p {
				stacks[n].push(mat[n][j])
			}
		}
	}
	out := make([][]int, len(mat))
	for k := range stacks {
		out[k] = stacks[k].ToSlice()
	}
	return out
}

