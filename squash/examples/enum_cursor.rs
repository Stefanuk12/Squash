use std::io::Cursor;

use squash::{u24, Color3, Result, SquashCursor, SquashObject};

#[derive(Debug, SquashObject)]
struct Bar {
    a: u8,
    b: u16,
    c: u24,
    d: u64,
}

#[derive(Debug, SquashObject)]
struct Baz {
    a: f32,
    b: f64,
    c: Color3,
}

#[derive(Debug, SquashObject)]
enum Foo {
    Bar(Bar),
    Baz(Baz),
}

fn main() -> Result<()> {
    let x = Foo::Baz(Baz {
        a: 2.0,
        b: 3.0,
        c: Color3 {
            r: 4,
            g: 5,
            b: 6,
        },
    });

    let mut cursor = Cursor::new(Vec::<u8>::new());
    cursor.push(x)?;
    println!("{:?}", cursor);
    println!("{:?}", cursor.pop::<Foo>()?);

    Ok(())
}
