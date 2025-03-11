// TODO consider
// - Multivariate polynomials
// - Matrices
// - Rationals
// - Symbolic
// - Permutations?

// NOTE: Pain points
// - multiple lifting `a.lift(&mid).lift(&top)` (seems impossible to avoid)
// - mutating functions, such as negate, interacting with Cow. Maybe enum(&T, &mut T) instead?

use mathlib::{DensePolynomial, I, Int, Mod, Structure};

fn main() {
    let fixed_int: I<32> = I;
    let mod13 = Mod::new(fixed_int.el(13));
    let a = fixed_int.el(4).lift(&mod13);
    println!("{}", a.copy() * a.copy());
    println!("{}", a.copy() - a.copy());

    let xpoly = DensePolynomial::new_symb("x", &Int);
    let three = Int.el(3).lift(&xpoly);
    let mut p = xpoly.symb();

    for _ in 0..4 {
        println!("{p}");
        p += three.copy();
        p *= p.copy().extend_lifetime(&xpoly);
    }

    let mod13 = Mod::new(Int.el(13));
    let xpolymod = DensePolynomial::new_symb("x", &mod13);
    let three = Int.el(3).lift(&mod13).lift(&xpolymod);
    let mut q = xpolymod.symb();

    for _ in 0..4 {
        println!("{q}");
        q += three.copy();
        q *= q.copy().extend_lifetime(&xpolymod);
    }

    println!("{}", Int.el(12345));

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
