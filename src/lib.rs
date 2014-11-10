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
/// 2  ::    An atom is a natural number.
/// 3  ::    A cell is an ordered pair of nouns.
#[deriving(Show, PartialEq, Eq, Clone, Hash)]
pub enum Noun {
    Atom(Number),
    Cell(Box<Noun>, Box<Noun>)
}

const YES: Noun = Atom(Native(0));
const NO: Noun = Atom(Native(1));

/// 5  ::    nock(a)          *a
//pub fn nock(n: Noun) -> Noun { tar(n) }

/// 6  ::    [a b c]          [a [b c]]
/// 7  ::
/// 8  ::    ?[a b]           0
/// 9  ::    ?a               1
pub fn wut(n: Noun) -> Noun {
    match n {
        Cell(_a, _b) => YES,
        Atom(_a) => NO
    }
}

/// 10 ::    +[a b]           +[a b]
/// 11 ::    +a               1 + a
pub fn lus(n: Noun) -> Noun {
    match n {
        Cell(_a, _b) => panic!("lus on Cell"),
        Atom(a) => match a {
            Native(n) => Atom(Native(1 + n)),
            Bignum(n) => Atom(Bignum(one::<Mpz>() + n))
        }
    }
}

/// 12 ::    =[a a]           0
/// 13 ::    =[a b]           1
/// 14 ::    =a               =a
pub fn tis(n: Noun) -> Noun {
    match n {
        Cell(a, b) => if a == b { YES } else { NO },
        Atom(_a) => panic!("tis on Atom")
    }
}

/// 16 ::    /[1 a]           a
/// 17 ::    /[2 a b]         a
/// 18 ::    /[3 a b]         b
/// 19 ::    /[(a + a) b]     /[2 /[a b]]
/// 20 ::    /[(a + a + 1) b] /[3 /[a b]]
/// 21 ::    /a               /a
pub fn fas(n: Noun) -> Noun {
    match n {
        Cell(n, a) => match *n {
            Atom(ref n) => match *a {
                Atom(ref a) if n.equiv(1) => Atom(a.clone()),
                Cell(ref a, ref _b) if n.equiv(2) => *a.clone(),
                Cell(ref _a, ref b) if n.equiv(3) => *b.clone(),
                Atom(b) => match n {
                    &Native(n) if n % 2 == 0 => fas(Cell(
                            box Atom(Native(2)),
                            box Cell(box Atom(Native(n / 2)), box Atom(b)))),
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
        Atom(_a) => panic!("fas on Atom")
    }
}


/*
23 ::    *[a [b c] d]     [*[a b c] *[a d]]
24 ::
25 ::    *[a 0 b]         /[b a]
26 ::    *[a 1 b]         b
27 ::    *[a 2 b c]       *[*[a b] *[a c]]
28 ::    *[a 3 b]         ?*[a b]
29 ::    *[a 4 b]         +*[a b]
30 ::    *[a 5 b]         =*[a b]
31 ::
32 ::    *[a 6 b c d]     *[a 2 [0 1] 2 [1 c d] [1 0] 2 [1 2 3] [1 0] 4 4 b]
33 ::    *[a 7 b c]       *[a 2 b 1 c]
34 ::    *[a 8 b c]       *[a 7 [[7 [0 1] b] 0 1] c]
35 ::    *[a 9 b c]       *[a 7 c 2 [0 1] 0 b]
36 ::    *[a 10 [b c] d]  *[a 8 c 7 [0 3] d]
37 ::    *[a 10 b c]      *[a c]
38 ::
39 ::    *a               *a
*/



