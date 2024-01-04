#![recursion_limit = "12345678"]
use std::marker::PhantomData;

fn main() {
    println!(
        "{}",
        <(
            encode!(
                ***********************
                ***********************
                ***********************
            ),
            encode!(
                **************
                **************
                **************
            )
        ) as Sub>::Diff::VALUE
    );
}

#[macro_export]
/// Works until about 3200
macro_rules! encode {
    () => {
        Zero
    };
    ($_a:tt $_b:tt $($tail:tt)*) => {
        Succ<Succ<encode!($($tail)*)>>
    };
    ($_a:tt $($tail:tt)*) => {
        Succ<encode!($($tail)*)>
    };
}

#[derive(Debug)]
struct Zero;

#[derive(Debug)]
struct Succ<T>(PhantomData<T>);

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

impl<T> Sub for (Zero, Succ<T>) {
    type Diff = Zero;
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

// Division

trait GreaterThanEq {
    type Greater;
}

impl GreaterThanEq for (Zero, Zero) {
    type Greater = Succ<Zero>;
}

impl<T> GreaterThanEq for (Zero, Succ<T>) {
    type Greater = Zero;
}

impl<T> GreaterThanEq for (Succ<T>, Zero) {
    type Greater = Succ<Zero>;
}

impl<T, U> GreaterThanEq for (Succ<T>, Succ<U>)
where
    (T, U): GreaterThanEq,
{
    type Greater = <(T, U) as GreaterThanEq>::Greater;
}

trait Div {
    type Quotient;
}

// Instead of implementing for (T, Succ<Zero>), implement for (Succ<T>, Succ<Zero>)
// to avoid overlapping with the next impl on (Zero, Succ<Zero>)
impl<T> Div for (Succ<T>, Succ<Zero>) {
    type Quotient = Succ<T>;
}

impl<T> Div for (Zero, T) {
    type Quotient = Zero;
}

type RawQuotient<T, U> = <(
    Succ<Zero>,
    <(<(Succ<T>, Succ<Succ<U>>) as Sub>::Diff, Succ<Succ<U>>) as Div>::Quotient,
) as Add>::Sum;

impl<T, U> Div for (Succ<T>, Succ<Succ<U>>)
where
    (T, Succ<U>): Sub,
    (<(T, Succ<U>) as Sub>::Diff, Succ<Succ<U>>): Div,
    (
        Succ<Zero>,
        <(<(T, Succ<U>) as Sub>::Diff, Succ<Succ<U>>) as Div>::Quotient,
    ): Add,
    (
        <(Succ<T>, Succ<Succ<U>>) as GreaterThanEq>::Greater,
        <(
            Succ<Zero>,
            <(<(T, Succ<U>) as Sub>::Diff, Succ<Succ<U>>) as Div>::Quotient,
        ) as Add>::Sum,
    ): Mul,
    (Succ<T>, Succ<Succ<U>>): GreaterThanEq,
{
    // If x < y, return 0. We can do this by multiplying the "RawQuotient" by
    // the bool->int value of this condition.
    type Quotient = <(
        <(Succ<T>, Succ<Succ<U>>) as GreaterThanEq>::Greater,
        RawQuotient<T, U>,
    ) as Mul>::Product;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        assert_eq!(<(encode!(), encode!()) as Add>::Sum::VALUE, 0);
        assert_eq!(<(encode!(*), encode!()) as Add>::Sum::VALUE, 1);
        assert_eq!(<(encode!(), encode!(*)) as Add>::Sum::VALUE, 1);
        assert_eq!(<(encode!(**), encode!(***)) as Add>::Sum::VALUE, 5);
    }

    #[test]
    fn subtraction() {
        assert_eq!(<(encode!(), encode!()) as Sub>::Diff::VALUE, 0);
        assert_eq!(<(encode!(*), encode!()) as Sub>::Diff::VALUE, 1);
        // Subtraction is saturating to make division easier
        assert_eq!(<(encode!(), encode!(*)) as Sub>::Diff::VALUE, 0);
        assert_eq!(<(encode!(***), encode!(**)) as Sub>::Diff::VALUE, 1);
    }

    #[test]
    fn multiplication() {
        assert_eq!(<(encode!(), encode!()) as Mul>::Product::VALUE, 0);
        assert_eq!(<(encode!(*), encode!()) as Mul>::Product::VALUE, 0);
        assert_eq!(<(encode!(), encode!(*)) as Mul>::Product::VALUE, 0);
        assert_eq!(<(encode!(*), encode!(*)) as Mul>::Product::VALUE, 1);
        assert_eq!(<(encode!(**), encode!(***)) as Mul>::Product::VALUE, 6);
    }

    #[test]
    fn greater_than_eq() {
        assert_eq!(<(encode!(), encode!()) as GreaterThanEq>::Greater::VALUE, 1);
        assert_eq!(
            <(encode!(*), encode!()) as GreaterThanEq>::Greater::VALUE,
            1
        );
        assert_eq!(
            <(encode!(), encode!(*)) as GreaterThanEq>::Greater::VALUE,
            0
        );
        assert_eq!(
            <(encode!(**), encode!(**)) as GreaterThanEq>::Greater::VALUE,
            1
        );
        assert_eq!(
            <(encode!(***), encode!(**)) as GreaterThanEq>::Greater::VALUE,
            1
        );
    }

    #[test]
    fn division() {
        // We define 0 / 0 as 0
        assert_eq!(<(encode!(), encode!()) as Div>::Quotient::VALUE, 0);
        // Gives a compiler error! We can't divide by 0! Such power.
        // assert_eq!(<(church!(*), church!()) as Div>::Quotient::VALUE, 0);
        assert_eq!(<(encode!(), encode!(*)) as Div>::Quotient::VALUE, 0);
        assert_eq!(<(encode!(*), encode!(*)) as Div>::Quotient::VALUE, 1);
        assert_eq!(<(encode!(**), encode!(***)) as Div>::Quotient::VALUE, 0);
        assert_eq!(<(encode!(***), encode!(**)) as Div>::Quotient::VALUE, 1);
        assert_eq!(<(encode!(******), encode!(**)) as Div>::Quotient::VALUE, 3);
        assert_eq!(<(encode!(*******), encode!(**)) as Div>::Quotient::VALUE, 3);
    }
}
