#![feature(proc_macro_hygiene)]

extern crate postgres_macros;

use postgres_macros::sql;

fn main() {
    let _ = sql!("SELECT foo FORM bar"); //~ ERROR syntax error at or near "bar"
}
