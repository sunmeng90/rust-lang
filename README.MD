# ch03 common programming concepts

## ch03-01-variables-and-mutability

**variables in rust is immutable by default**  
variables in rust is immutable by default, e.g. `let x = 5`, you can't assign a new value to `x`.
To declare a mutable variable, add `mut` in front of the variable name. `let mut x = 5`

**const vs immutable**  
const are always immutable  
const are decleared by keyword `const`  
const may be set only to a const expression, not the result of a function call or any other value that could only be computed at runtime  

**shadowing**  
you can declare a **new variable** with the same name as a previous variable, and the new variable shadows the previous variable. and the new variable can have different type with the previous variable

```rs
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
```

Shadowing is different from marking a variable as mut, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword.   
The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.

```rs
#![allow(unused_variables)]
fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
}
```

## ch03-02-data-type

Rust is **statically type** language,  which means that it must know the types of all variables at compile time.  
The compiler can usually infer the type of variable based on the value and how we use it. In case when many types are possible, we must add a type annotation, like this:

```rs
#![allow(unused_variables)]
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
}
```

### Scalar Types

A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

```rs
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

Rust’s char type is four bytes in size and represents a Unicode Scalar Value

### Compound Types

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

#### The Tuple Type

A tuple is a general way of grouping together a number of values with **a variety of types** into one compound type. Tuples have **a fixed length**: once declared, they cannot grow or shrink in size.

```rs
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

destructing tuple: 

```rs
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```


# Q & A


1. How to organize code:  
When the code base is getting larger, you might need to work with multiple crates on the same project. Rust supports this via Workspaces, run example by: `(cd examples/hello && cargo run)`