#![feature(proc_macro_hygiene)]

extern crate postgres_macros;

use postgres_macros::sql;

fn main() {
    let s = sql!("SELECT foo FROM bar");
    assert_eq!("SELECT foo FROM bar", s);
}
