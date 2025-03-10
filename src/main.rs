// TODO consider
// - Multivariate polynomials
// - Matrices
// - Rationals
// - Symbolic
// - Permutations?

// NOTE: Pain points
// - explicit lifting (required due to run time types?)
// - cloning and memory management

use mathlib::{DensePolynomial, ElTExt, Int, Mod};

fn main() {
    let a = Mod::new(4i64, 13);
    println!("{}", a.clone() * a.clone());
    println!("{}", a.clone() - a);

    let mut p = DensePolynomial::<Int>::new_symb("x", &());
    let three = Int::from(3i32).lift(p.td());

    for _ in 0..4 {
        println!("{p}");
        p += &three;
        p = p.clone() * p;
    }

    let m = (Int::from(13), ());
    let mut q = DensePolynomial::<Mod<Int>>::new_symb("x", &m);
    let three = Int::from(3i32).lift::<Mod<Int>>(&m).lift(q.td());

    for _ in 0..4 {
        println!("{q}");
        q += &three;
        q = q.clone() * q;
    }

    // TODO The code below will work after implementing polynomial rem
    /*
    let mod_num = (Int::from(293), ());
    let x = DensePolynomial::<Mod<Int>>::new_symb("x", &mod_num);
    let five = Int::from(5).lift::<Mod<Int>>(&mod_num).lift(x.td());
    let three = Int::from(3).lift::<Mod<Int>>(&mod_num).lift(x.td());

    let mod_poly = (x + five) * (x + three);
    let x = Mod::new(x, mod_poly);
    let mut r = x;

    for _ in 0..4 {
        println!("{r}");
        r += &three;
        r = r.clone() * r;
    }
    */
}
