#![feature(proc_macro_hygiene)]

extern crate postgres_macros;

use postgres_macros::sql;

fn main() {
    sql!("
        LOCK TABLE foo IN ACCESS EXCLUSIVE MODE
    ");

    sql!("
        ALTER TABLE foo
            ADD CONSTRAINT foo PRIMARY KEY (foo)
    ");

    sql!("
        ALTER TABLE foo
            ADD CONSTRAINT foo
                FOREIGN KEY (foo)
                REFERENCES foo (foo)
                ON DELETE RESTRICT
                ON UPDATE RESTRICT
    ");

    sql!("
        CREATE INDEX foo ON foo (foo)
    ");

    sql!("
        INSERT INTO foo VALUES ($1)
    ");

    sql!("
        LOCK TABLE foo IN ACCESS EXCLUSIVE MODE
    ");

    sql!("
        ALTER TABLE foo
            ADD CONSTRAINT foo PRIMARY KEY (foo)
    ");
}
