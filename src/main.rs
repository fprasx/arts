#![feature(associated_type_defaults)]
#![recursion_limit = "256"]

use std::marker::PhantomData;

fn main() {
    println!("{}, {}, {}, {}",
             <(church!(**), church!(***)) as Add>::Sum::VALUE,
             <(church!(), church!(***)) as Add>::Sum::VALUE,
             <(church!(**), church!()) as Add>::Sum::VALUE,
             <(church!(), church!()) as Add>::Sum::VALUE,
    );
    println!("{}, {}, {}",
             <(church!(*****), church!(***)) as Sub>::Diff::VALUE,
             <(church!(**), church!()) as Sub>::Diff::VALUE,
             <(church!(), church!()) as Sub>::Diff::VALUE,

             // Gives a compiler error! We can't underflow! Such power.
             // <(church!(), church!(***)) as Subtract>::Diff::VALUE,
    );
    println!(
        "{}, {}, {}, {}, {}, {}",
        <(church!(*****), church!(***)) as Mul>::Product::VALUE,
        <(church!(***), church!(*)) as Mul>::Product::VALUE,
        <(church!(*), church!(*****)) as Mul>::Product::VALUE,
        <(church!(***), church!()) as Mul>::Product::VALUE,
        <(church!(), church!(*****)) as Mul>::Product::VALUE,
        <(church!(), church!()) as Mul>::Product::VALUE,
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

// Evaluation

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

// Addition

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

impl<T, U> Add for (Succ<T>, Succ<U>)
where
    (T, Succ<Succ<U>>): Add,
{
    type Sum = <(T, Succ<Succ<U>>) as Add>::Sum;
}

// Subtraction

trait Sub {
    type Diff;
}

impl Sub for (Zero, Zero) {
    type Diff = Zero;
}

impl<T> Sub for (Succ<T>, Zero) {
    type Diff = Succ<T>;
}

impl<T, U> Sub for (Succ<T>, Succ<U>)
where
    (T, U): Sub,
{
    type Diff = <(T, U) as Sub>::Diff;
}

// Multiplication

trait Mul {
    type Product;
}

impl<T> Mul for (T, Zero) {
    type Product = Zero;
}

// Implementing for (Zero, T) would overlap with the previous impl, so we do
// (Zero, Succ<T>) to avoid the (Zero, Zero) case that overlaps
impl<T> Mul for (Zero, Succ<T>) {
    type Product = Zero;
}

impl<T, U> Mul for (Succ<T>, Succ<U>)
where
    (T, Succ<U>): Mul,
    (Succ<U>, <(T, Succ<U>) as Mul>::Product): Add,
{
    // x * y = x + (x - 1) * y
    type Product = <(Succ<U>, <(T, Succ<U>) as Mul>::Product) as Add>::Sum;
}
