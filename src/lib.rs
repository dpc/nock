#![feature(globs)]

extern crate gmp;

use std::num::{ zero, one, from_u64, ToPrimitive };

use gmp::Mpz;

#[deriving(Show, PartialEq, Eq, Clone, Hash)]
pub enum Number {
    Native(u64),
    Bignum(Mpz)
}

impl Number {
    fn equiv(&self, u: u64) -> bool {
        match self {
            &Native(n) => n == u,
            &Bignum(ref n) => {
                let n = n.to_u64();
                n.is_some() && n.unwrap() == u
            }
        }
    }
}

/// 1  ::    A noun is an atom or a cell.
#[deriving(Show, PartialEq, Eq, Clone, Hash)]
pub enum Noun {
    /// 2  ::    An atom is a natural number.
    Atom(Number),
    /// 3  ::    A cell is an ordered pair of nouns.
    Cell(Box<Noun>, Box<Noun>)
}

const YES: Noun = Atom(Native(0));
const NO: Noun = Atom(Native(1));

/// 5  ::    nock(a)          *a
pub fn nock(n: Noun) -> Noun { tar(n) }

pub fn wut(n: Noun) -> Noun {
    match n {
        /// 8  ::    ?[a b]           0
        Cell(_a, _b) => YES,
        /// 9  ::    ?a               1
        Atom(_a) => NO
    }
}

pub fn lus(n: Noun) -> Noun {
    match n {
        /// 10 ::    +[a b]           +[a b]
        Cell(_a, _b) => panic!("lus on Cell"),
        /// 11 ::    +a               1 + a
        Atom(a) => match a {
            Native(n) => Atom(Native(1 + n)),
            Bignum(n) => Atom(Bignum(one::<Mpz>() + n))
        }
    }
}

pub fn tis(n: Noun) -> Noun {
    match n {
        /// 12 ::    =[a a]           0
        /// 13 ::    =[a b]           1
        Cell(a, b) => if a == b { YES } else { NO },
        /// 14 ::    =a               =a
        Atom(_a) => panic!("tis on Atom")
    }
}


pub fn fas(n: Noun) -> Noun {
    match n {
        Cell(n, a) => match *n {
            Atom(ref n) => match *a {
                /// 16 ::    /[1 a]           a
                Atom(ref a) if n.equiv(1) => Atom(a.clone()),
                /// 17 ::    /[2 a b]         a
                Cell(box ref a, ref _b) if n.equiv(2) => (*a).clone(),
                /// 18 ::    /[3 a b]         b
                Cell(ref _a, box ref b) if n.equiv(3) => (*b).clone(),
                Atom(b) => match n {
                    /// 19 ::    /[(a + a) b]     /[2 /[a b]]
                    &Native(n) if n % 2 == 0 => fas(Cell(
                            box Atom(Native(2)),
                            box Cell(box Atom(Native(n / 2)), box Atom(b)))),
                    /// 20 ::    /[(a + a + 1) b] /[3 /[a b]]
                    &Bignum(ref n) if *n % from_u64::<Mpz>(2).unwrap() == zero() => fas(Cell(
                            box Atom(Native(2)),
                            box Cell(box Atom(Bignum(*n / from_u64::<Mpz>(2).unwrap())),
                                box Atom(b)))),
                            _ => panic!("fas on Cell with number where n not (a + a), (a + a +1)")
                },
                Cell(_, _) => panic!("fas on Cell with number where n not 1, 2, 3, (a + a), (a + a+ 1)")
            },
            Cell(_, _) => panic!("fas with Cell subject")
        },
        /// 21 ::    /a               /a
        Atom(_a) => panic!("fas on Atom")
    }
}


pub fn tar(n: Noun) -> Noun {
    // I'm so sorry.
    match n {
        /// 23 ::    *[a [b c] d]     [*[a b c] *[a d]]
        Cell(ref a, box Cell(box Cell(ref b, ref c), ref d)) =>
            Cell(box tar(Cell((*a).clone(), box Cell((*b).clone(), (*c).clone()))),
                 box tar(Cell((*a).clone(), (*d).clone()))),
        Cell(ref a, box Cell(ref n, box ref b)) => match *n {
            box Atom(ref n) => match b {
                /// 25 ::    *[a 0 b]         /[b a]
                _ if n.equiv(0) => fas(Cell(box b.clone(), a.clone())),
                /// 26 ::    *[a 1 b]         b
                _ if n.equiv(1) => (*b).clone(),
                /// 27 ::    *[a 2 b c]       *[*[a b] *[a c]]
                &Cell(ref b, ref c) if n.equiv(2) => tar(Cell(
                    box tar(Cell((*a).clone(), (*b).clone())),
                    box tar(Cell((*a).clone(), (*c).clone())))),
                /// 28 ::    *[a 3 b]         ?*[a b]
                _ if n.equiv(3) => wut(tar(Cell((*a).clone(), box b.clone()))),
                /// 29 ::    *[a 4 b]         +*[a b]
                _ if n.equiv(4) => lus(tar(Cell((*a).clone(), box b.clone()))),
                /// 30 ::    *[a 5 b]         =*[a b]
                _ if n.equiv(5) => tis(tar(Cell((*a).clone(), box b.clone()))),
                /// 32 ::    *[a 6 b c d]     *[a 2 [0 1] 2 [1 c d] [1 0] 2 [1 2 3] [1 0] 4 4 b]
                &Cell(ref b, box ref e) if n.equiv(6) => match e {
                    &Cell(ref c, ref d) => tar(Cell((*a).clone(), box Cell(box Atom(Native(2)), box Cell(box Cell(box Atom(Native(0)), box Atom(Native(1))), box Cell(box Atom(Native(2)), box Cell(box Cell(box Atom(Native(1)), box Cell((*c).clone(), (*d).clone())), box Cell(box Cell(box Atom(Native(1)), box Atom(Native(0))), box Cell(box Atom(Native(2)), box Cell(box Cell(box Atom(Native(1)), box Cell(box Atom(Native(2)), box Atom(Native(3)))), box Cell(box Cell(box Atom(Native(1)), box Atom(Native(0))), box Cell(box Atom(Native(4)), box Cell(box Atom(Native(4)), (*b).clone())))))))))))),
                    &Atom(_) => panic!("I lost track...")
                },
                /// 33 ::    *[a 7 b c]       *[a 2 b 1 c]
                &Cell(ref b, ref c) if n.equiv(7) => tar(Cell((*a).clone(), box Cell(box Atom(Native(2)), box Cell((*b).clone(), box Cell(box Atom(Native(1)), (*c).clone()))))),
                /// 34 ::    *[a 8 b c]       *[a 7 [[7 [0 1] b] 0 1] c]
                &Cell(ref b, ref c) if n.equiv(8) => tar(Cell((*a).clone(), box Cell(box Atom(Native(7)), box Cell(box Cell(box Cell(box Atom(Native(7)), box Cell(box Cell(box Atom(Native(0)), box Atom(Native(1))), (*b).clone())), box Cell(box Atom(Native(0)), box Atom(Native(1)))), (*c).clone())))),
                /// 35 ::    *[a 9 b c]       *[a 7 c 2 [0 1] 0 b]
                &Cell(ref b, ref c) if n.equiv(9) => tar(Cell((*a).clone(), box Cell(box Atom(Native(7)), box Cell((*c).clone(), box Cell(box Atom(Native(2)), box Cell(box Cell(box Atom(Native(0)), box Atom(Native(1))), box Cell(box Atom(Native(0)), (*b).clone()))))))),
                /// 36 ::    *[a 10 [b c] d]  *[a 8 c 7 [0 3] d]
                &Cell(box ref e, ref d) if n.equiv(10) => match e {
                    &Cell(ref _b, ref c) => tar(Cell((*a).clone(), box Cell(box Atom(Native(8)), box Cell((*c).clone(), box Cell(box Atom(Native(7)), box Cell(box Cell(box Atom(Native(0)), box Atom(Native(3))), (*d).clone())))))),
                    &Atom(_) => panic!("I lost track...")
                },
                /// 37 ::    *[a 10 b c]      *[a c]
                &Cell(ref _b, ref c) if n.equiv(10) => tar(Cell((*a).clone(), (*c).clone())),
                _ => panic!("tar on invalid Cell")
            },
            box Cell(ref _a, ref _b) => panic!("tar on Cell with invalid arguments")
        },
        Cell(_, _) => panic!("tar on invalid Cell"),
        /// 39 ::    *a               *a
        Atom(_a) => panic!("tar on Atom")
    }
}


mod bench {
    extern crate test;
    use super::*;
    use self::test::Bencher;

    #[bench]
    fn simple_eval(b: &mut Bencher) {
        b.iter(|| {
            let n = tar(
                Cell(
                    box Cell(
                        box Atom(Native(19)),
                        box Atom(Native(42))
                    ),
                    box Cell(
                        box Cell(
                            box Atom(Native(0)),
                            box Atom(Native(3))
                        ),
                        box Cell(
                            box Atom(Native(0)),
                            box Atom(Native(2))
                        )
                    )
               )
            );
        });
    }
}
