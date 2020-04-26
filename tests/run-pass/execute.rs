#![feature(proc_macro_hygiene)]

// ignore-test

extern crate postgres_macros;

use postgres_macros::sql;

struct Connection;

impl Connection {
    fn execute(&self, _: &str, _: &[&i32]) {}
}

fn main() {
    execute!(Connection, "SELECT foo FROM bar");

    execute!(Connection, "SELECT foo FROM bar WHERE baz = $1", &1);

    execute!(Connection, "SELECT foo FROM bar WHERE baz = $1 AND buz = $2", &1, &2);
}
