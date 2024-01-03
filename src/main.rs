#![feature(associated_type_defaults)]
#![recursion_limit = "100000000"]

use std::marker::PhantomData;

fn main() {
    println!("{}",
            Succ::<Succ::<Zero>>::VALUE
    );
    println!("{}",
        <<<<<<<<
          Zero as Nat>
        ::Next as Nat>
        ::Next as Nat>
        ::Next as Nat>
        ::Next as Nat>
        ::Next as Nat>
        ::Next as Nat>
        ::Next as Nat>
        ::Next::VALUE
    );
    println!(
        "{}",
        <church!(
            ***********************
            ***********************
            ***********************
        )>::VALUE
    );
}

#[macro_export]
/// Works until about 3200
macro_rules! church {
    () => {
        Zero
    };
    ($_a:tt $_b:tt $($tail:tt)*) => {
        <<church!($($tail)*) as Nat>::Next as Nat>::Next
    };
    ($_a:tt $($tail:tt)*) => {
        <church!($($tail)*) as Nat>::Next
    };
}

#[derive(Debug)]
struct Zero;

#[derive(Debug)]
struct Succ<T>(PhantomData<T>);

// Need a function from type -> type. Implementing a trait and making an
// associated type does this!
trait Nat
where
    Self: Sized,
{
    type Next = Succ<Self>;
}

impl Nat for Zero {}
impl<T> Nat for Succ<T> {}

// Recursively evaluate through the type
trait Value {
    const VALUE: usize;
}
impl Value for Zero {
    const VALUE: usize = 0;
}
impl<T> Value for Succ<T>
where
    T: Value,
{
    const VALUE: usize = 1 + T::VALUE;
}
