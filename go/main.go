package main

import (
	"fmt"
	"math/rand"
	"sync"
	"time"
)

var wg sync.WaitGroup

func carre(m [][]int, n int) {
	defer wg.Done()

	start := time.Now()

	var tmp int

	res := make([][]int, n)

	for i := 0; i < n; i++ {

		res[i] = make([]int, n)

		for j := 0; j < n; j++ {

			tmp = 0

			for k := 0; k < n; k++ {
				tmp = tmp + (m[i][k] * m[k][j])
			}

			res[i][j] = tmp
		}
	}

	finish := time.Since(start)
	// log.Printf("Mettre la matrice au carre a pris %s", finish)

	fmt.Println("La matrice au carre est:", res)
	fmt.Println("Le temps d'execution de la fonction est :", finish)
}

func displayDet(mat [][]int) {
	defer wg.Done()
	start := time.Now()

	r := Determinant(mat)

	finish := time.Since(start)
	fmt.Println("Le déterminant est :", r)
	fmt.Println("Le temps d'execution de la fonction est :", finish)
}

func main() {
	start := time.Now()
	const n = 10 //taille de la matrice

	m1 := make([][]int, n)

	for i := 0; i < n; i++ {

		m1[i] = make([]int, n)

		for j := 0; j < n; j++ {
			m1[i][j] = rand.Intn(10)

		}
	}

	fmt.Println("La matrice de base est :", m1)

	wg.Add(1)
	go displayDet(m1)

	wg.Add(1)
	go carre(m1, n)

	wg.Wait()
	finish := time.Since(start)
	fmt.Println("Le temps d'execution du programme est :", finish)
}
