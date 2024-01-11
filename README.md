# Arithmetic in Rust's Type System

Based on some basic ideas from math, we use deeply nested genenics to represent
numbers and then apply traits like functions to do arithmetic! Here's how we'd
multiply two numbers:

```rust
trait Mul {
    type Product;
}

impl<T> Mul for (T, Zero) {
    type Product = Zero;
}

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
```

Then doing `<(encode!(***), encode!(****)) as Mul>::Product::VALUE` would give
the actual integer 12, even though everything in the previous expression is
a type!


Here's a blog post that explains it in much more detail:
[Doing First Grade Math in Rust's Type System](https://fprasx.github.io/articles/type-system-arithmetic/)

## License

This project (more like formula sheet) is licensed under the GNU General Public
License v3.0, because math is for everyone.
