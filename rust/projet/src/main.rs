extern crate nalgebra as na;
extern crate rand;

use std::thread;
use std::time::Duration;
use std::time::Instant;

use na::{U10, Matrix, ArrayStorage};
use rand::Rng;


const N: usize = 10;
type Matrix10x10 = Matrix<f32, U10, U10, ArrayStorage<f32, U10, U10>>;

fn det(m: Matrix10x10) -> f32{
    return m.determinant();
}

fn carre(m: Matrix10x10) -> Matrix10x10{
    return m*m;
}

fn display_carree(m: Matrix10x10) {
    let now = Instant::now();

    println!("La matrice a la puissance 4 est: {:?}", carre(carre(m)));
    println!("Le temps d'execution de la fonction carre est : {}ms", now.elapsed().as_millis());
} 

fn display_det(m: Matrix10x10) {
    let now = Instant::now();

    println!("Le determinant est: {:?}", det(m).round());
    println!("Le temps d'execution de la fonction determiant est : {}ms", now.elapsed().as_millis());
}

fn main() {
    let now = Instant::now();
    
    let mut m: Matrix10x10 = Matrix10x10::zeros();
    for i in  0..N {
        for j in 0..N {
            m[(i, j)] = rand::thread_rng().gen_range(0.0, 100.0);
        }
    }
    thread::spawn(move ||{
        display_carree(m);
        thread::sleep(Duration::from_millis(1));
    });

    display_det(m);
    thread::sleep(Duration::from_millis(1));


    println!("Le temps d'execution du programme est : {}ms", now.elapsed().as_millis());
}
