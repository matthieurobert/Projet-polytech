// extern crate rand;
// use bareiss::bareiss_determinant;
// use rand::Rng;
// use std::time::Instant;
// use std::slice;

// const N: usize = 10;

// // pub struct Stack<T> {
// //     maxsize: usize,
// //     stack: Vec<T>,
// // }

// pub struct Stack<T> {
//     stack: Vec<T>,
// }

// impl<T> Stack<T> {

//     pub fn with_capacity(maxsize: usize) -> Self {
//         Self {
//             stack: Vec::with_capacity(maxsize),
//         }
//     }
//     pub fn pop(&mut self) -> Option<T> {
//         self.stack.pop()
//     }
//     pub fn push(&mut self, item: T) {
//         self.stack.push(item);
//     }


// }

// fn Determinant(m:[[u64; N];N]) -> u64{
//     let mut z;
//     if m.len() != m[0].len() {
//         return 0;
//     }
//     if m.len() == 1 {
//         return m[0][0];
//     }
//     if m.len() == 2 {
//         return m[0][0] * m[1][1] - m[0][1] *m[1][0];
//     }
//     let mut s: u64 = 0;
//     // let mut len: usize = m.len();

//     for i in 0..N {
//         let mut sm = subMat(MatMinusRow(m),i);
//         z = Determinant(sm);
//         if i%2 != 0 {
//             s -= z*m[0][i]
//         }else{
//             s += z*m[0][i]
//         }     
//     }
//     return s;
// }

// fn MatMinusRow(m:[[u64; N];N])-> [[u64; N];N]{
    
//     let mut mat = [[0;N];N];
//     for i in 0..N{
//         for j in 1..N{
//             mat[i][j] = m[i][j]
//         }
//     }
//     return mat
// }

// fn subMat(m:[[u64; N];N],p: usize) -> [[u64; N];N]{

//     let mut stacks = [Stack { stack: Vec::new() };N];
//     for n in m.iter(){
//         for j in m.iter(){
//             if j != p.into() {
//                 let mut add: u64 = m[n][j];
//                 stacks.push(add);
//             }
//         }
//     }

//     // for x in 0..N {
//     //     let mut n = m[x];
//     //     for y in 0..N {
//     //         let mut j = m[x][y];
//     //         if j != p {
//     //             stacks.push(j);
//     //         }
//     //     }
//     // }
    
//     let foo = std::usize::MAX;
//     let mut bar: u64 = foo.into();

//     let mut out = [[0;N]; N];
    
//     for k in stacks.iter(){
//         out[k]=stacks[k];
//     }
//     return out;
// }

// fn displayDet(mat: [[u64;10];10]) {
//     let r = Determinant(mat);

//     println!("{:?}", r);
// }





// fn carre(m: [[u64; N]; N]) {
//     let now = Instant::now();

//     let mut tmp;
//     let mut tmp2;

//     let mut res = [[0; N]; N];

//     for i in  0..N {
//         for j in 0..N {
           
//             tmp = 0;

//             for k in 0..N {
//                 tmp2 = m[i][k] * m[k][j];

//                 tmp = tmp + tmp2;
//             }

//             res[i][j] = tmp;
//         }
//     }
//     println!("{:?}", res);
//     println!("{}", now.elapsed().as_nanos());
// }

// fn main() {
    
//     let mut m1 = [[0;N];N];

//     for i in  0..N {
//         for j in 0..N {
//             m1[i][j] = rand::thread_rng().gen_range(0, 100);
//         }
//     }
//     let mut m11 = m1.to_vec();
//     println!("{:?}", m1);
//     println!("{:?}",bareiss_determinant(&mut m1, N).unwrap());
//     carre(m1);
//     // displayDet(m1);
// }
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

    println!("{:?}", carre(m));
    println!("Le temps d'execution de la fonction carre est : {}ms", now.elapsed().as_millis());
} 

fn display_det(m: Matrix10x10) {
    let now = Instant::now();

    println!("{:?}", det(m).round());
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
