#![feature(globs)]

extern crate nock;

use nock::*;

fn main() {
    println!("{}",
        tar(
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
        )
    );
}
