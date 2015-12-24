#![feature(plugin)]
#![plugin(herbie_lint)]

#[deny(herbie)]

fn main() {
    let (a, b, c) = (0., 0., 0.);
    (a/a + a) * a; //~ERROR
    (a/b + a) * a; //~ERROR
    (a/b + c) * a; //~ERROR

    (0./1. + 2.) * 1.; //~ERROR
    (1./1. + 2.) * 1.; //~ERROR
    (1./1. + 1.) * 1.; //~ERROR

    (0./1. + a) * 1.; //~ERROR
    (0./a + 2.) * a; //~ERROR
}
