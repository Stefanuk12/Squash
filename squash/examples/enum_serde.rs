use serde::Serialize;
use squash::{deserialize, impl_serde_for_enum, serialize, u24, Color3, Result, ReverseDeserialize};

#[derive(Debug, Serialize, ReverseDeserialize)]
struct Bar {
    a: u8,
    b: u16,
    c: u24,
    d: u64,
}

#[derive(Debug, Serialize, ReverseDeserialize)]
struct Baz {
    a: f32,
    b: f64,
    c: Color3,
}

#[derive(Debug)]
enum Foo {
    Bar(Bar),
    Baz(Baz),
}
impl_serde_for_enum!(Foo, Bar = 0, Baz = 1);

fn main() -> Result<()> {
    let x = Foo::Bar(Bar {
        a: 2,
        b: 3,
        c: u24::new(4).unwrap(),
        d: 5,
    });

    let mut bytes = serialize(&x)?;
    println!("{:?}", bytes);
    let des = deserialize::<Foo>(&mut bytes)?;
    println!("{:?}", des);

    Ok(())
}
