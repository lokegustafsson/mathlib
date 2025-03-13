// TODO consider
// - Multivariate polynomials
// - Matrices
// - Rationals
// - Symbolic
// - Permutations?

// NOTE: Pain points
// - mutating functions, such as negate, interacting with Cow. Maybe enum(&T, &mut T) instead?

use mathlib::{DensePolynomial, I, Int, Mod, Structure, SuperStructure, Super2Structure};

fn main() {
    let fixed_int: I<32> = I;
    let mod13 = Mod::new(fixed_int.el(13));
    let a = mod13.el1(4);
    println!("{}", a.copy() * a.copy());
    println!("{}", a.copy() - a.copy());

    let xpoly = DensePolynomial::new_symb("x", &Int);
    let three = xpoly.el1(3);
    let mut p = xpoly.symb();

    for _ in 0..4 {
        println!("{p}");
        p += three.copy();
        p *= p.copy().extend_lifetime(&xpoly);
    }

    let mod13 = Mod::new(Int.el(13));
    let xpolymod = DensePolynomial::new_symb("x", &mod13);
    let three = xpolymod.el2(3);
    let mut q = xpolymod.symb();

    for _ in 0..4 {
        println!("{q}");
        q += three.copy();
        q *= q.copy().extend_lifetime(&xpolymod);
    }

    println!("{}", Int.el(12345));

    // TODO The code below will work after implementing polynomial rem
    /*
    let mod_num = Mod::new(Int.el(293));
    let xpoly = DensePolynomial::new_symb("x", &mod_num);
    let x = xpoly.symb();

    let mod_poly = (x + xpoly.el2(5)) * (x + xpoly.el2(3));
    let mpoly = Mod::new(mod_poly);
    let mut r = mpoly.symb();

    for _ in 0..4 {
        println!("{r}");
        r += mpoly.el3(3);
        r = r.clone() * r;
    }
    */
}
