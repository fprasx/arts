#![feature(associated_type_defaults)]
#![recursion_limit = "100000000"]

use std::marker::PhantomData;

fn main() {
    println!("{}, {}, {}, {}",
             <(church!(**), church!(***)) as Add>::Sum::VALUE,
             <(church!(), church!(***)) as Add>::Sum::VALUE,
             <(church!(**), church!()) as Add>::Sum::VALUE,
             <(church!(), church!()) as Add>::Sum::VALUE,
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

trait Add {
    type Sum;
}

impl Add for (Zero, Zero) {
    type Sum = Zero;
}

impl<T> Add for (Succ<T>, Zero) {
    type Sum = Succ<T>;
}

impl<T> Add for (Zero, Succ<T>) {
    type Sum = Succ<T>;
}

impl<T, U> Add for (Succ<T>, Succ<U>) where (T, Succ<Succ<U>>): Add  {
    type Sum = <(T, Succ<Succ<U>>) as Add>::Sum;
}
