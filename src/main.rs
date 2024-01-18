#![recursion_limit = "12345678"]
use std::marker::PhantomData;

fn main() {
    println!(
        "{}",
        Sub::<
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
        >::VALUE
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

trait AddImpl {
    type Output;
}

type Add<L, R> = <(L, R) as AddImpl>::Output;

impl AddImpl for (Zero, Zero) {
    type Output = Zero;
}

impl<T> AddImpl for (Succ<T>, Zero) {
    type Output = Succ<T>;
}

impl<T> AddImpl for (Zero, Succ<T>) {
    type Output = Succ<T>;
}

impl<T, U> AddImpl for (Succ<T>, Succ<U>)
where
    (T, Succ<Succ<U>>): AddImpl,
{
    type Output = Add<T, Succ<Succ<U>>>;
}

// Subtraction

trait SubImpl {
    type Output;
}

type Sub<L, R> = <(L, R) as SubImpl>::Output;

impl SubImpl for (Zero, Zero) {
    type Output = Zero;
}

impl<T> SubImpl for (Succ<T>, Zero) {
    type Output = Succ<T>;
}

impl<T> SubImpl for (Zero, Succ<T>) {
    type Output = Zero;
}

impl<T, U> SubImpl for (Succ<T>, Succ<U>)
where
    (T, U): SubImpl,
{
    type Output = Sub<T, U>;
}

// Multiplication

trait MulImpl {
    type Output;
}

type Mul<L, R> = <(L, R) as MulImpl>::Output;

impl<T> MulImpl for (T, Zero) {
    type Output = Zero;
}

// Implementing for (Zero, T) would overlap with the previous impl, so we do
// (Zero, Succ<T>) to avoid the (Zero, Zero) case that overlaps
impl<T> MulImpl for (Zero, Succ<T>) {
    type Output = Zero;
}

impl<T, U> MulImpl for (Succ<T>, Succ<U>)
where
    (T, Succ<U>): MulImpl,
    (Succ<U>, Mul<T, Succ<U>>): AddImpl,
{
    // x * y = y + (x - 1) * y
    type Output = Add<Succ<U>, Mul<T, Succ<U>>>;
}

// Division

trait GreaterThanEqImpl {
    type Output;
}

type GreaterThanEq<L, R> = <(L, R) as GreaterThanEqImpl>::Output;

impl GreaterThanEqImpl for (Zero, Zero) {
    type Output = Succ<Zero>;
}

impl<T> GreaterThanEqImpl for (Zero, Succ<T>) {
    type Output = Zero;
}

impl<T> GreaterThanEqImpl for (Succ<T>, Zero) {
    type Output = Succ<Zero>;
}

impl<T, U> GreaterThanEqImpl for (Succ<T>, Succ<U>)
where
    (T, U): GreaterThanEqImpl,
{
    type Output = GreaterThanEq<T, U>;
}

trait DivImpl {
    type Output;
}

type Div<L, R> = <(L, R) as DivImpl>::Output;

// Instead of implementing for (T, Succ<Zero>), implement for (Succ<T>, Succ<Zero>)
// to avoid overlapping with the next impl on (Zero, Succ<Zero>)
impl<T> DivImpl for (Succ<T>, Succ<Zero>) {
    type Output = Succ<T>;
}

impl<T> DivImpl for (Zero, T) {
    type Output = Zero;
}

type RawQuotient<T, U> = Add<
    Succ<Zero>,
    Div<Sub<Succ<T>, Succ<Succ<U>>>, Succ<Succ<U>>>,
>;

impl<T, U> DivImpl for (Succ<T>, Succ<Succ<U>>)
where
    (T, Succ<U>): SubImpl,
    (Sub<T, Succ<U>>, Succ<Succ<U>>): DivImpl,
    (
        Succ<Zero>,
        Div<Sub<T, Succ<U>>, Succ<Succ<U>>>,
    ): AddImpl,
    (
        GreaterThanEq<Succ<T>, Succ<Succ<U>>>,
        Add<
            Succ<Zero>,
            Div<Sub<T, Succ<U>>, Succ<Succ<U>>>,
        >,
    ): MulImpl,
    (Succ<T>, Succ<Succ<U>>): GreaterThanEqImpl,
{
    // If x < y, return 0. We can do this by multiplying the "RawQuotient" by
    // the bool->int value of this condition.
    type Output = Mul<
        GreaterThanEq<Succ<T>, Succ<Succ<U>>>,
        RawQuotient<T, U>,
    >;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        assert_eq!(Add::<encode!(), encode!()>::VALUE, 0);
        assert_eq!(Add::<encode!(*), encode!()>::VALUE, 1);
        assert_eq!(Add::<encode!(), encode!(*)>::VALUE, 1);
        assert_eq!(Add::<encode!(**), encode!(***)>::VALUE, 5);
    }

    #[test]
    fn subtraction() {
        assert_eq!(Sub::<encode!(), encode!()>::VALUE, 0);
        assert_eq!(Sub::<encode!(*), encode!()>::VALUE, 1);
        // Subtraction is saturating to make division easier
        assert_eq!(Sub::<encode!(), encode!(*)>::VALUE, 0);
        assert_eq!(Sub::<encode!(***), encode!(**)>::VALUE, 1);
    }

    #[test]
    fn multiplication() {
        assert_eq!(Mul::<encode!(), encode!()>::VALUE, 0);
        assert_eq!(Mul::<encode!(*), encode!()>::VALUE, 0);
        assert_eq!(Mul::<encode!(), encode!(*)>::VALUE, 0);
        assert_eq!(Mul::<encode!(*), encode!(*)>::VALUE, 1);
        assert_eq!(Mul::<encode!(**), encode!(***)>::VALUE, 6);
    }

    #[test]
    fn greater_than_eq() {
        assert_eq!(GreaterThanEq::<encode!(), encode!()>::VALUE, 1);
        assert_eq!(
            GreaterThanEq::<encode!(*), encode!()>::VALUE,
            1
        );
        assert_eq!(
            GreaterThanEq::<encode!(), encode!(*)>::VALUE,
            0
        );
        assert_eq!(
            GreaterThanEq::<encode!(**), encode!(**)>::VALUE,
            1
        );
        assert_eq!(
            GreaterThanEq::<encode!(***), encode!(**)>::VALUE,
            1
        );
    }

    #[test]
    fn division() {
        // We define 0 / 0 as 0
        assert_eq!(Div::<encode!(), encode!()>::VALUE, 0);
        // Gives a compiler error! We can't divide by 0! Such power.
        // assert_eq!(Div::<encode!(*), encode!()>::VALUE, 0);
        assert_eq!(Div::<encode!(), encode!(*)>::VALUE, 0);
        assert_eq!(Div::<encode!(*), encode!(*)>::VALUE, 1);
        assert_eq!(Div::<encode!(**), encode!(***)>::VALUE, 0);
        assert_eq!(Div::<encode!(***), encode!(**)>::VALUE, 1);
        assert_eq!(Div::<encode!(******), encode!(**)>::VALUE, 3);
        assert_eq!(Div::<encode!(*******), encode!(**)>::VALUE, 3);
    }
}
