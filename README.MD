# The Rust Programming Language

## 3 common programming concepts

### 3.1 Variables and Mutability

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

### 3.2 Data Types

Rust is **statically type** language,  which means that it must know the types of all variables at compile time.  
The compiler can usually infer the type of variable based on the value and how we use it. In case when many types are possible, we must add a type annotation, like this:

```rs
#![allow(unused_variables)]
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
}
```

#### Scalar Types

A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

```rs
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

Rust’s char type is four bytes in size and represents a Unicode Scalar Value

#### Compound Types

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

##### The Tuple Type

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

access a tuple element by use period(.) follow by the index of the value:  

```rs
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

#### The Array Type

Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Arrays in Rust are different from arrays in some other languages because arrays in Rust have a fixed length, like tuples.

```rs
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

declare a array with type and length

```rs
#![allow(unused_variables)]
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
}
```

Here, `i32` is the type of each element. After the semicolon, the number 5 indicates the array contains five elements.

declare a array that contain the same value for each element

```rs
#![allow(unused_variables)]
fn main() {
let a = [3; 5];
}
```

The array named a will contain 5 elements that will all be set to the value 3 initially. This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.

##### Accessing Array Elements

Array index start with 0 in rust

```rs
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

### 3.3 Functions

In rust, keyword `fn` are used to declare a new function. Rust code uses snake case as the conventional style for function and variable names. In snake case, all letters are lowercase and underscores separate words.

```rs
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

function parameters

```rs
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```
The declaration of another_function has one parameter named x. The type of x is specified as i32.

Function bodies are made up of a series of statements optionally ending in an expression. Statements do not return values. Therefore, you can’t assign a let statement to another variable, as the following code tries to do; you’ll get an error:

```rs
fn main() {
    let x = (let y = 6);
}
```

Calling a function is an expression. Calling a macro is an expression. **The block that we use to create new scopes, {}, is an expression**, for example:

```rs
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

This expression:

```rs
{
    let x = 3;
    x + 1
}
```

is a block that, in this case, evaluates to 4. That value gets bound to y as part of the let statement. Note the x + 1 line without a semicolon at the end, which is unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value. 

Functions with Return Values

Functions can return values to the code that calls them. We don’t name return values, but we do declare their type after an arrow (->). In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly. Here’s an example of a function that returns a value:

```rs
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

There are no function calls, macros, or even let statements in the five function—just the number 5 by itself. That’s a perfectly valid function in Rust. Note that the function’s return type is specified too, as -> i32. The 5 in five is the function’s return value, which is why the return type is i32. 

### 3.5. Control Flow

#### 3.5.1 if

```rs
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

Using if in a let Statement  
Because if is an expression, we can use it on the right side of a let statement

```rs
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```

 This means the values that have the potential to be results from each arm of the if must be the same type

 ```rs
 fn main() {
    let condition = true;

    let number = if condition {
        5 // int32
    } else {
        "six" // string
    };

    println!("The value of number is: {}", number); //error: 
}
```

When we try to compile this code, we’ll get an error:x:. The if and else arms have value types that are incompatible

#### 3.5.2 loop

```rs
fn main() {
    loop {
        println!("again!");
    }
}
```

Returning Values from Loops

One of the uses of a loop is to retry an operation you know might fail, such as checking whether a thread has completed its job. However, you might need to pass the result of that operation to the rest of your code. To do this, you can add the value you want returned after the break expression you use to stop the loop; that value will be returned out of the loop so you can use it, as shown here:

```rs
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

after `break` the loop, we use a semicolon to end the statement that assigns the value to result

Conditional Loops with while

```rs
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

Looping Through a Collection with for

You could use the while construct to loop over the elements of a collection

```rs
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

this approach is error prone, we could cause the program to panic if the index length is incorrect. It’s also slow, because the compiler adds runtime code to perform the conditional check on every element on every iteration through the loop.

As a more concise alternative, you can use a `for` loop and execute some code for each item in a collection.

```rs
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

The safety and conciseness of `for` loops make them the most commonly used loop construct in Rust, Even in situations in which you want to run some code a certain number of times, most Rustaceans would use a for loop. The way to do that would be to use a Range, which is a type provided by the standard library that generates all numbers in sequence starting from one number and ending before another number.

```rs
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

## 4. Understanding Ownership

Ownership is Rust’s most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector. 

### 4.1. What is Ownership?

Rust’s central feature is ownership. Although the feature is straightforward to explain, it has deep implications for the rest of the language.

All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that constantly looks for no longer used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks at compile time. None of the ownership features slow down your program while it’s running.

>The Stack and the Heap  
>
>Both the stack and the heap are parts of memory that are available to your code to use at runtime, but they are structured in different ways. The stack LIFO(last in, first out).  
>All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead. The heap is less organized: when you put data on the heap, you request a certain amount of space. The operating system finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating. Pushing values onto the stack is not considered allocating. Because the pointer is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.  
>Pushing to the stack is faster than allocating on the heap because the operating system never has to search for a place to store new data; that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work, because the operating system must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.  
>Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. A processor can do its job better if it works on data that’s close to other data (as it is on the stack) rather than farther away (as it can be on the heap). Allocating a large amount of space on the heap can also take time.  
>When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.  
>Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses. Once you understand ownership, you won’t need to think about the stack and the heap very often, but knowing that managing heap data is why ownership exists can help explain why it works the way it does.

Ownership Rules

First, let’s take a look at the ownership rules:

* Each value in Rust has a variable that’s called its owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

Variable Scope

A scope is the range within a program for which an item is valid.  

```rs
#![allow(unused_variables)]
fn main() {
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
}
```

The `String` type
There are two type for string values in rust:  

1. String literals are immutable. e.g. let s = `hello world`
2. They second type of string is `String`, This type is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. You can create a String from a string literal using the from function, like so: `let s = String::from("hello");`

The second type of String can be mutated:

```rs
#![allow(unused_variables)]
fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}
```

So, what’s the difference here? Why can String be mutated but literals cannot? The difference is how these two types deal with memory.

Memory and Allocation

In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. Unfortunately, we can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.

With the `String` type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

* The memory must be requested from the operating system at runtime.
* We need a way of returning this memory to the operating system when we’re done with our String.

That first part is done by us: when we call `String::from`, its implementation requests the memory it needs. This is pretty much universal in programming languages.

However, the second part is different. In languages with a garbage collector (GC), the GC keeps track and cleans up memory that isn’t being used anymore, and we don’t need to think about it. Without a GC, it’s our responsibility to identify when memory is no longer being used and call code to explicitly return it, just as we did to request it. Doing this correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free.

Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope. Here’s a string created by `String` instead of a string literal:

```rs

#![allow(unused_variables)]
fn main() {
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                    // longer valid
}
```

There is a natural point at which we can return the memory our String needs to the operating system: when s goes out of scope. When a variable goes out of scope, Rust calls a special function for us. This function is called `drop`, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.

Ways Variables and Data Interact: Move

Multiple variables can interact with the same data in different ways in Rust

```rs
#![allow(unused_variables)]
fn main() {
    let x = 5;
    let y = x;
}
```
We can probably guess what this is doing: “bind the value 5 to x; then make a copy of the value in x and bind it to y.” We now have two variables, x and y, and both equal 5. This is indeed what is happening, **because integers are simple values with a known, fixed size, and these two 5 values are pushed onto the stack**.

Now let’s look at the String version:

```rs
#![allow(unused_variables)]
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
}
```

This looks very similar to the previous code, so we might assume that the way it works would be the same: that is, the second line would make a copy of the value in s1 and bind it to s2. **But this isn’t quite what happens.**

Take a look at Figure 4-1 to see what is happening to String under the covers. A String is made up of three parts, shown on the left: a pointer to the memory that holds the contents of the string, a length, and a capacity. This group of data is stored on the stack. On the right is the memory on the heap that holds the contents.

>![Alternate text](./static/img/trpl04-01.svg)
>Figure 4-1: Representation in memory of a String holding the value "hello" bound to s1

When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. **We do not copy the data on the heap that the pointer refers to**. In other words, the data representation in memory looks like Figure 4-2.

>![Alternate text](./static/img/trpl04-02.svg)
>Figure 4-2: Representation in memory of the variable s2 that has a copy of the pointer, length, and capacity of s1

The representation does not look like Figure 4-3, which is what memory would look like if Rust instead copied the heap data as well. If Rust did this, the operation s2 = s1 could be very expensive in terms of runtime performance if the data on the heap were large.

>![Alternate text](./static/img/trpl04-03.svg)
>Figure 4-3: Another possibility for what s2 = s1 might do if Rust copied the heap data as well 

Earlier, we said that when a variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory for that variable. But Figure 4-2 shows both data pointers pointing to the same location. This is a problem: when `s2` and `s1` go out of scope, they will both try to free the same memory. This is known as a double free error and is one of the memory safety bugs we mentioned previously. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

To ensure memory safety, there’s one more detail to what happens in this situation in Rust. **Instead of trying to copy the allocated memory, Rust considers `s1` to no longer be valid and, therefore, Rust doesn’t need to free anything when `s1` goes out of scope**. Check out what happens when you try to use `s1` after `s2` is created; it won’t work:

```rs
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

You’ll get an error like this because Rust prevents you from using the invalidated reference:

```
error[E0382]: use of moved value: `s1`
 --> src/main.rs:5:28
  |
3 |     let s2 = s1;
  |         -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`, which does
  not implement the `Copy` trait
```

If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. **But because Rust also invalidates the first variable**, instead of being called a shallow copy, **it’s known as a move**. In this example, we would say that `s1` was **moved** into `s2`. So what actually happens is shown in Figure 4-4.

>![Alternate text](./static/img/trpl04-04.svg)
>Figure 4-4: Representation in memory after s1 has been invalidated

That solves our problem! With only `s2` valid, when it goes out of scope, it alone will free the memory, and we’re done.

Ways Variables and Data Interact: Clone

If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called `clone`. 

```rs

#![allow(unused_variables)]
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

This works just fine and explicitly produces the behavior shown in Figure 4-3, where the heap data does get copied.

Stack-Only Data: Copy

There’s another wrinkle we haven’t talked about yet. This code using integers, part of which was shown in Listing 4-2, works and is valid:

```rs

#![allow(unused_variables)]
fn main() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
```

But this code seems to contradict what we just learned: we don’t have a call to clone, but x is still valid and wasn’t moved into y.

**The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from being valid after we create the variable y**. In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do anything different from the usual shallow copying and we can leave it out.

Rust has a special annotation called the `Copy` trait that we can place on types like integers that are stored on the stack (we’ll talk more about traits in Chapter 10). If a type has the `Copy` trait, an older variable is still usable after assignment. Rust won’t let us annotate a type with the `Copy` trait if the type, or any of its parts, has implemented the Drop trait. If the type needs something special to happen when the value goes out of scope and we add the `Copy` annotation to that type, we’ll get a compile-time error.

So what types are `Copy`? You can check the documentation for the given type to be sure, but as a general rule, any group of simple scalar values can be `Copy`, and nothing that requires allocation or is some form of resource is `Copy`. Here are some of the types that are `Copy`:

* All the integer types, such as u32.
* The Boolean type, bool, with values true and false.
* All the floating point types, such as f64.
* The character type, char.
* Tuples, if they only contain types that are also Copy. For example, (i32, i32) is * Copy, but (i32, String) is not.

Ownership and Functions

The semantics for passing a value to a function are similar to those for assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does. Listing 4-3 has an example with some annotations showing where variables go into and out of scope.

```rs
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

    Listing 4-3: Functions with ownership and scope annotated

Return Values and Scope

Returning values can also transfer ownership. Listing 4-4 is an example with similar annotations to those in Listing 4-3.

```rs
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

    Listing 4-4: Transferring ownership of return values

The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless the data has been moved to be owned by another variable.

Taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership? It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.

It’s possible to return multiple values using a tuple, as shown in Listing 4-5.

```rs
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

    Listing 4-5: Returning ownership of parameters

But this is too much ceremony and a lot of work for a concept that should be common. Luckily for us, Rust has a feature for this concept, called references.


### 4.2. References and Borrowing

The issue with the tuple code in Listing 4-5 is that we have to return the `String` to the calling function so we can still use the String after the call to `calculate_length`, because the String was moved into `calculate_length`.

Here is how you would define and use a `calculate_length` function that has a reference to an object as a parameter instead of taking ownership of the value:

```rs
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

First, notice that all the tuple code in the variable declaration and the function return value is gone. Second, note that we pass `&s1` into `calculate_length` and, in its definition, we take `&String` rather than `String`.

These ampersands are references, and they allow you to refer to some value without taking ownership of it. Figure 4-5 shows a diagram.

>![Alternate text](./static/img/trpl04-05.svg)
>Figure 4-5: A diagram of `&String s` pointing at String `s1`  

>Note: The opposite of referencing by using `&` is dereferencing, which is accomplished with the dereference operator, `*`. We’ll see some uses of the dereference operator in Chapter 8 and discuss details of dereferencing in Chapter 15.

Let’s take a closer look at the function call here:

```rs

#![allow(unused_variables)]
fn main() {
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
}
```

The `&s1` syntax lets us create a reference that refers to the value of s1 but does not own it. Because it does not own it, the value it points to will not be dropped when the reference goes out of scope.

Likewise, the signature of the function uses `&` to indicate that the type of the parameter s is a reference. Let’s add some explanatory annotations:

```rs
#![allow(unused_variables)]
fn main() {
    fn calculate_length(s: &String) -> usize { // s is a reference to a String
        s.len()
    } // Here, s goes out of scope. But because it does not have ownership of what
    // it refers to, nothing happens.
}
```

The scope in which the variable s is valid is the same as any function parameter’s scope, but we don’t drop what the reference points to when it goes out of scope because we don’t have ownership. **When functions have references as parameters instead of the actual values, we won’t need to return the values in order to give back ownership, because we never had ownership**

We call having references as function parameters *borrowing*. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back.

So what happens if we try to modify something we’re borrowing? Try the code in Listing 4-6. Spoiler alert: it doesn’t work!

```rs
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

    Listing 4-6: Attempting to modify a borrowed value


```
error[E0596]: cannot borrow immutable borrowed content `*some_string` as mutable
 --> error.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- use `&mut String` here to make mutable
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^ cannot borrow as mutable
```

Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.


Mutable References

We can fix the error in the code from Listing 4-6 with just a small tweak:

```rs
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

First, we had to change s to be mut. Then we had to create a mutable reference with &mut s and accept a mutable reference with some_string: &mut String.

But mutable references have one big restriction: you can have only one mutable reference to a particular piece of data in a particular scope. This code will fail:

```rs
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```

Here’s the error:

```
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 |
7 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here
```

This restriction allows for mutation but in a very controlled fashion. It’s something that new Rustaceans struggle with, because most languages let you mutate whenever you’d like.

The benefit of having this restriction is that Rust can prevent data races at compile time. A *data race* is similar to a race condition and happens when these three behaviors occur:

* Two or more pointers access the same data at the same time.
* At least one of the pointers is being used to write to the data.
* There’s no mechanism being used to synchronize access to the data.

Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; Rust prevents this problem from happening because it won’t even compile code with data races!

As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:

```rs

#![allow(unused_variables)]
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;

    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}
```

A similar rule exists for combining mutable and immutable references. This code results in an error:

```rs
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```

Here's the error:

```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:14
  |
4 |     let r1 = &s; // no problem
  |              -- immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |              ^^^^^^ mutable borrow occurs here
7 |
8 |     println!("{}, {}, and {}", r1, r2, r3);
  |                                -- immutable borrow later used here
```

Whew! We also cannot have a mutable reference while we have an immutable one. Users of an immutable reference don’t expect the values to suddenly change out from under them! However, multiple immutable references are okay because no one who is just reading the data has the ability to affect anyone else’s reading of the data.

Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. For instance, this code will compile because the last usage of the immutable references occurs before the mutable reference is introduced:

```rs
#![allow(unused_variables)]
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

```

The scopes of the immutable references r1 and r2 end after the println! where they are last used, which is before the mutable reference r3 is created. These scopes don’t overlap, so this code is allowed.

Even though borrowing errors may be frustrating at times, remember that it’s the Rust compiler pointing out a potential bug early (at compile time rather than at runtime) and showing you exactly where the problem is. Then you don’t have to track down why your data isn’t what you thought it was.

Dangling References

In languages with pointers, it’s easy to erroneously create a dangling pointer, a pointer that references a location in memory that may have been given to someone else, by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

Let’s try to create a dangling reference, which Rust will prevent with a compile-time error:

```rs
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

Here’s the error:

```
error[E0106]: missing lifetime specifier
 --> main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is
  no value for it to be borrowed from
  = help: consider giving it a 'static lifetime
```

Let’s take a closer look at exactly what’s happening at each stage of our dangle code:

```rs
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

Because s is created inside dangle, when the code of dangle is finished, s will be deallocated. But we tried to return a reference to it. That means this reference would be pointing to an invalid String. That’s no good! Rust won’t let us do this.

The solution here is to return the String directly:

```rs

#![allow(unused_variables)]
fn main() {
    fn no_dangle() -> String {
        let s = String::from("hello");

        s
    }
}
```

This works without any problems. Ownership is moved out, and nothing is deallocated.

The Rules of References

Let’s recap what we’ve discussed about references:

* At any given time, you can have either one mutable reference or any number of immutable references.
* References must always be valid.




### 4.3. The Slice Type

Another data type that does not have ownership is the slice. Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

Image we have a function `first_word` that find the first word in a string, and return the index of first blank in the string.

```rs
fn first_word(s: &String) -> usize { // use &String, becuase we don't want the ownership of the passing in string
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

>Listing 4-8: Storing the result from calling the `first_word` function and then changing the String contents

This program compiles without any errors and would also do so if we used `word` after calling `s.clear()`. Because `word` isn’t connected to the state of `s` at all, `word` still contains the value `5`. We could use that value `5` with the variable `s` to try to extract the first word out, but this would be a bug because the contents of `s` have changed since we saved `5` in `word`.

We now have the index in word getting out of sync with the data in `s` and it is tedious and error prone. Luckily, Rust has a solution to this problem: string slices.

String Slices

A string slice is a reference to part of a `String`, and it looks like this:

```rs
#![allow(unused_variables)]
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}
```

This is similar to taking a reference to the whole `String` but with the extra `[0..5]` bit. Rather than a reference to the entire `String`, it’s a reference to a portion of the String.

We can create slices using a range within brackets by specifying `[starting_index..ending_index]`, where `starting_index` is the first position in the slice and `ending_index` is one more than the last position in the slice. Internally, the slice data structure stores the starting position and the length of the slice, which corresponds to ending_index minus starting_index. So in the case of `let world = &s[6..11];, world` would be a slice that contains a pointer to the 7th byte (counting from 1) of `s` with a length value of 5.

>![Alternate text](./static/img/trpl04-06.svg)
>Figure 4-6: String slice referring to part of a String

```rs

#![allow(unused_variables)]
fn main() {
    let s = String::from("hello");

    let slice = &s[0..2]; // slice the first two bytes
    let slice = &s[..2];  // same but short hand

    let len = s.len();

    let slice = &s[3..len]; // slice from the 4th to the end
    let slice = &s[3..];  // same but short hand

    let slice = &s[0..len]; // slice the whole string
    let slice = &s[..]; // same but short hand
}
```

>Note: String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error. For the purposes of introducing string slices, we are assuming ASCII only in this section; a more thorough discussion of UTF-8 handling is in the “Storing UTF-8 Encoded Text with Strings” section of Chapter 8.

With all this information in mind, let’s rewrite `first_word` to return a slice. The type that signifies “string slice” is written as `&str`:

```rs
#![allow(unused_variables)]
fn main() {
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
}
```

Now when we call first_word, we get back a single value that is tied to the underlying data. The value is made up of a reference to the starting point of the slice and the number of elements in the slice.

We now have a straightforward API that’s much harder to mess up, because the compiler will ensure the references into the String remain valid. Remember the bug in the program in Listing 4-8, when we got the index to the end of the first word but then cleared the string so our index was invalid? That code was logically incorrect but didn’t show any immediate errors. The problems would show up later if we kept trying to use the first word index with an emptied string. Slices make this bug impossible and let us know we have a problem with our code much sooner. Using the slice version of first_word will throw a compile-time error:

```rs
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
```

Here’s the compiler error:

```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:18:5
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
17 |
18 |     s.clear(); // error!
   |     ^^^^^^^^^ mutable borrow occurs here
19 |
20 |     println!("the first word is: {}", word);
   |                                       ---- immutable borrow later used here
```

Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference. Because clear needs to truncate the String, it needs to get a mutable reference. Rust disallows this, and compilation fails. 

String Literals Are Slices

Recall that we talked about string literals being stored inside the binary. Now that we know about slices, we can properly understand string literals:

```rs
#![allow(unused_variables)]
fn main() {
    let s = "Hello, world!";
}
```

The type of `s` here is `&str`: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; `&str` is an immutable reference.

String Slices as Parameters

Knowing that you can take slices of literals and String values leads us to one more improvement on `first_word`, and that’s its signature:

```rs
fn first_word(s: &str) -> &str {
```

>Listing 4-9: Improving the first_word function by using a string slice for the type of the s parameter

If we have a string slice, we can pass that directly. If we have a `String`, we can pass a slice of the entire `String`. Defining a function to take a string slice instead of a reference to a `String` makes our API more general and useful without losing any functionality:

```rs
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

Other Slices

String slices, as you might imagine, are specific to strings. But there’s a more general slice type, too. Consider this array:

```rs
#![allow(unused_variables)]
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
}
```

This slice has the type &[i32]. It works the same way as string slices do, by storing a reference to the first element and a length. 


Summary

The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time. The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.


## 5. Using Structs to Structure Related Data

A struct, or structure, is a custom data type that lets you name and package together multiple related values that make up a meaningful group. If you’re familiar with an object-oriented language, a struct is like an object’s data attributes

### 5.1. Defining and Instantiating Structs

Structs are similar to tuples. Like tuples, the pieces of a struct can be different types. Unlike with tuples, you’ll name each piece of data so it’s clear what the values mean. As a result of these names, structs are more flexible than tuples: you don’t have to rely on the order of the data to specify or access the values of an instance.

To define a struct, we enter the keyword struct and name the entire struct. Then, inside curly brackets, we define the names and types of the pieces of data, which we call fields. 

```rs

#![allow(unused_variables)]
fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
}
```
> Listing 5-1: A User struct definition

create *instance* of struct

```rs
#![allow(unused_variables)]
fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}
```

> Listing 5-2: Creating an instance of the User struct

To get a specific value from a struct, we can use dot notation. If we wanted just this user’s email address, we could use `user1.email` wherever we wanted to use this value. If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field. Listing 5-3 shows how to change the value in the `email` field of a mutable `User` instance.

```rs
#![allow(unused_variables)]
fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

> Listing 5-3: Changing the value in the email field of a User instance

Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable. 

Using the Field Init Shorthand when Variables and Fields Have the Same Name

When the parameter names and the struct field names are exactly the same, we can use the field init shorthand syntax to avoid repetition of email and username in comment out line, as shown in Listing 5-5.

```rs
#![allow(unused_variables)]
fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // fn build_user(email: String, username: String) -> User {
    //     User {
    //         email: email, // having to repeat the email and username field names and variables is a bit tedious
    //         username: username,
    //         active: true,
    //         sign_in_count: 1,
    //     }
    // }

    fn build_user(email: String, username: String) -> User {
        User {
            email, // field name are the same with parameter name, use field init shorthand
            username,
            active: true,
            sign_in_count: 1,
        }
    }
}
```

Creating Instances From Other Instances With Struct Update Syntax

It’s often useful to create a new instance of a struct that uses most of an old instance’s values but changes some. You’ll do this using *struct update syntax*.


```rs
#![allow(unused_variables)]
fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     username: String::from("anotherusername567"),
    //     active: user1.active,  // create a new instance with same active and sign_in_count value as instance user1 manually
    //     sign_in_count: user1.sign_in_count,
    // };

    let user2 = User {
        email: String::from("another@example.com"), // struct update syntax
        username: String::from("anotherusername567"),
        ..user1
    };
}
```

> Listing 5-7: Using struct update syntax to set new `email` and `username` values for a User instance but use the rest of the values from the fields of the instance in the `user1` variable

Using Tuple Structs without Named Fields to Create Different Types

You can also define structs that look similar to tuples, called tuple structs. Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name and make the tuple be a different type from other tuples, and naming each field as in a regular struct would be verbose or redundant.

To define a tuple struct, start with the `struct` keyword and the struct name followed by the types in the tuple. For example, here are definitions and usages of two tuple structs named `Color` and `Point`:

```rs

#![allow(unused_variables)]
fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

> Note that the black and origin values are different types, because they’re instances of different tuple structs. Each struct you define is its own type, even though the fields within the struct have the same types. For example, a function that takes a parameter of type Color cannot take a Point as an argument, even though both types are made up of three i32 values. Otherwise, tuple struct instances behave like tuples: you can destructure them into their individual pieces, you can use a . followed by the index to access an individual value, and so on.

Unit-Like Structs Without Any Fields

You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (), the unit type. Unit-like structs can be useful in situations in which you need to implement a trait on some type but don’t have any data that you want to store in the type itself.

> **Ownership of Struct Data**  
> In the User struct definition in Listing 5-1, we used the owned String type rather than the &str string slice type. This is a deliberate choice because we want instances of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.  
> It’s possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes, a Rust feature that we’ll discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is. Let’s say you try to store a reference in a struct without specifying lifetimes, like this, which won’t work:

```rs
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```

The compiler will complain that it needs lifetime specifiers:

```
error[E0106]: missing lifetime specifier
 -->
  |
2 |     username: &str,
  |               ^ expected lifetime parameter

error[E0106]: missing lifetime specifier
 -->
  |
3 |     email: &str,
  |            ^ expected lifetime parameter
```

### 5.2. An Example Program Using Structs

use struct instance as function parameter

```rs
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

> Listing 5-10: Defining a Rectangle struct

### 5.3. Method Syntax

*Methods* are similar to functions: they’re declared with the `fn` keyword and their name, they can have parameters and a return value, and they contain some code that is run when they’re called from somewhere else. However, methods are different from functions in that they’re defined within the context of a struct (or an enum or a trait object, which we cover in Chapters 6 and 17, respectively), and their first parameter is always `self`, which represents the instance of the struct the method is being called on.

Defining Methods

```rs
#![allow(used_methods)]
fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 20,
        height: 30,
    };
    println!("rect detail: {:?}", rect);
    println!("rect2 detail: {:?}", rect2);
    println!(
        "Rect can hold Rect2? {}.",
        rect.can_hold(&rect2)
    );
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // define method on Rectangle struct
    fn area(&self) -> u32 { // user &self reference here, because we don't want to take ownership
        self.width * self.width
    }

    fn can_hold(&self, another: &Rectangle) -> bool {
        self.width > another.width && self.height > another.height
    }
}
```

> Listing 5-13: Defining an area method on the Rectangle struct

To define the function within the context of Rectangle, we start an impl (implementation) block. Then we move the area function within the impl curly brackets and change the first (and in this case, only) parameter to be self in the signature and everywhere within the body. 


Associated Functions

Another useful feature of `impl` blocks is that we’re allowed to define functions within `impl` blocks that don’t take `self` as a parameter. These are called associated functions because they’re associated with the struct. They’re still functions, not methods, because they don’t have an instance of the struct to work with. You’ve already used the `String::from` associated function.

Associated functions are often used for constructors that will return a new instance of the struct.


```rs
fn main() {
    let square = Rectangle::square(20);
    println!("square detail: {:?}", square);
    println!(
        "The area of the square is {} square pixels.",
        square.area()
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // associated functions are associated with the struct.
    // They’re still functions, not methods, because they don’t have an instance of the struct to work with
    // usage: let sq = Rectangle::square(3)
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    // define method on Rectangle struct
    fn area(&self) -> u32 { // user &self reference here, because we don't want to take ownership
        self.width * self.width
    }

    fn can_hold(&self, another: &Rectangle) -> bool {
        self.width > another.width && self.height > another.height
    }
}
```

Multiple `impl` Blocks

Each struct is allowed to have multiple impl blocks. 


```rs
fn main() {}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // associated functions are associated with the struct.
    // They’re still functions, not methods, because they don’t have an instance of the struct to work with
    // usage: let sq = Rectangle::square(3)
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    // define method on Rectangle struct
    fn area(&self) -> u32 { // user &self reference here, because we don't want to take ownership
        self.width * self.width
    }
}

// one struct can have multiple impl block
impl Rectangle {
    fn can_hold(&self, another: &Rectangle) -> bool {
        self.width > another.width && self.height > another.height
    }
}
```

Summary

Structs let you create custom types that are meaningful for your domain. By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear. Methods let you specify the behavior that instances of your structs have, and associated functions let you namespace functionality that is particular to your struct without having an instance available.

But structs aren’t the only way you can create custom types: let’s turn to Rust’s enum feature to add another tool to your toolbox.

## 6. Enums and Pattern Matching

*Enumerations*, also referred to as *enums*. Enums allow you to define a type by enumerating its possible variants. 

### 6.1. Defining an Enum

Enums are defined with keyword `enum`

```rs
#![allow(unused_variables)]
fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4; // We can create instances of each of the enum variants of IpAddrKind like this:
    let six = IpAddrKind::V6;
}
```

we can define a function that takes a enum as an argument

```rs
#![allow(unused_variables)]
fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    fn route(ip_kind: IpAddrKind) { }

    route(IpAddrKind::V4); // we can call this function with either variant:
    route(IpAddrKind::V6);
}
```

we can putting data directly into each enum variant. This new definition of the IpAddr enum says that both V4 and V6 variants will have associated String values:

```rs
#![allow(unused_variables)]
fn main() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}
```

There’s another advantage to using an enum rather than a struct: **each variant can have different types and amounts of associated data**. Version four type IP addresses will always have four numeric components that will have values between 0 and 255. If we wanted to store `V4` addresses as four `u8` values but still express `V6` addresses as one `String` value, we wouldn’t be able to with a struct. Enums handle this case with ease:

```rs

#![allow(unused_variables)]
fn main() {
    enum IpAddr {
        V4(u8, u8, u8, u8), //each enum variant can have different types and amounts of associated data. 
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
```

we can also put any kind of data inside an enum variant: strings, numeric types, or structs, for example. You can even include another enum!

```rs

#![allow(unused_variables)]
fn main() {
    struct Ipv4Addr {
        // --snip--
    }

    struct Ipv6Addr {
        // --snip--
    }

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
}
```

Let’s look at another example of an enum in Listing 6-2: this one has a wide variety of types embedded in its variants.

```rs

#![allow(unused_variables)]
fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
}
```

> Listing 6-2: A Message enum whose variants each store different amounts and types of values

This enum has four variants with different types:

* Quit has no data associated with it at all.
* Move includes an anonymous struct inside it.
* Write includes a single String.
* ChangeColor includes three i32 values.

Defining an enum with variants such as the ones in Listing 6-2 is similar to defining different kinds of struct definitions, except the enum doesn’t use the struct keyword and all the variants are grouped together under the Message type. The following structs could hold the same data that the preceding enum variants hold:

```rs

#![allow(unused_variables)]
fn main() {
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct
}
```

But if we used the different structs, which each have their own type, we couldn’t as easily define a function to take any of these kinds of messages as we could with the Message enum defined in Listing 6-2, which is a single type.

There is one more similarity between enums and structs: just as we’re able to define methods on structs using `impl`, we’re also able to define methods on enums. Here’s a method named `call` that we could define on our `Message` enum:

```rs

#![allow(unused_variables)]
fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
```

The body of the method would use self to get the value that we called the method on. In this example, we’ve created a variable m that has the value Message::Write(String::from("hello")), and that is what self will be in the body of the call method when m.call() runs.

The `Option` Enum and Its Advantages Over Null Values

This enum is Option<T>, and it is defined by the standard library as follows:

```rs
#![allow(unused_variables)]
fn main() {
    enum Option<T> {
        Some(T),
        None,
    }
}
```

The `Option<T>` enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly. In addition, so are its variants: you can use `Some` and `None` directly without the `Option:: prefix`. The `Option<T>` enum is still just a regular enum, and `Some(T)` and None are still variants of type `Option<T>`. The `<T>` a generic type parameter in Rust. Here are some examples of using Option values to hold number types and string types:

```rs
#![allow(unused_variables)]
fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}
```

If we use `None` rather than `Some`, we need to tell Rust what type of `Option<T>` we have, because the compiler can’t infer the type that the Some variant will hold by looking only at a None value.

When we have a `Some` value, we know that a value is present and the value is held within the `Some`. When we have a `None` value, in some sense, it means the same thing as null: we don’t have a valid value. So why is having `Option<T>` any better than having null?

In short, because `Option<T>` and `T` (where `T` can be any type) are different types, the compiler won’t let us use an `Option<T>` value as if it were definitely a valid value. For example, this code won’t compile because it’s trying to add an `i8` to an `Option<i8>`:

```rs
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

If we run this code, we get an error message like this:

```
error[E0277]: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is
not satisfied
 -->
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + std::option::Option<i8>`
  |
```

Intense! In effect, this error message means that Rust doesn’t understand how to add an `i8` and an `Option<i8>`, because they’re different types. When we have a value of a type like `i8` in Rust, the compiler will ensure that we always have a valid value. We can proceed confidently without having to check for null before using that value. Only when we have an `Option<i8>` (or whatever type of value we’re working with) do we have to worry about possibly not having a value, and the compiler will make sure we handle that case before using the value.

In other words, you have to convert an `Option<T>` to a `T` before you can perform `T` operations with it. Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.

**Not having to worry about incorrectly assuming a not-null value helps you to be more confident in your code. In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value `Option<T>`. Then, when you use that value, you are required to explicitly handle the case when the value is null. Everywhere that a value has a type that isn’t an `Option<T>`, you can safely assume that the value isn’t null. This was a deliberate design decision for Rust to limit null’s pervasiveness and increase the safety of Rust code.**

In general, in order to use an `Option<T>` value, you want to have code that will handle each variant. You want some code that will run only when you have a `Some(T)` value, and this code is allowed to use the inner `T`. You want some other code to run if you have a `None` value, and that code doesn’t have a `T` value available. The match expression is a control flow construct that does just this when used with enums: it will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.

### 6.2. The match Control Flow Operator

Rust has an extremely powerful control flow operator called `match` that allows you to compare a value against a series of patterns and then execute code based on which pattern matches. Patterns can be made up of literal values, variable names, wildcards, and many other things; The power of `match` comes from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.

```rs
#![allow(unused_variables)]

#![allow(unused_variables)]
fn main() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

```

> Listing 6-3: An enum and a `match` expression that has the variants of the `enum` as its patterns

Let’s break down the `match` in the `value_in_cents` function. First, we list the match keyword followed by an expression, which in this case is the value coin. This seems very similar to an expression used with `if`, but there’s a big difference: with `if`, the expression needs to return a Boolean value, but here, it can be any type. The type of coin in this example is the `Coin` enum that we defined on line 1.

Next are the match arms. An arm has two parts: a pattern and some code. The first arm here has a pattern that is the value Coin::Penny and then the => operator that separates the pattern and the code to run. The code in this case is just the value 1. Each arm is separated from the next with a comma.

When the match expression executes, it compares the resulting value against the pattern of each arm, in order. If a pattern matches the value, the code associated with that pattern is executed. If that pattern doesn’t match the value, execution continues to the next arm

Patterns that Bind to Values

Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. This is how we can extract values out of enum variants.

In the match expression for the below code, we add a variable called `state` to the pattern that matches values of the variant `Coin::Quarter`. When a `Coin::Quarter` matches, the state variable will bind to the value of that quarter’s state. Then we can use `state` in the code for that arm, like so:

```rs

#![allow(unused_variables)]
fn main() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            },
        }
    }
}

```

If we were to call `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin` would be `Coin::Quarter(UsState::Alaska)`. When we compare that value with each of the match arms, none of them match until we reach `Coin::Quarter(state)`. At that point, the binding for state will be the value `UsState::Alaska`. We can then use that binding in the `println!` expression, thus getting the inner state value out of the `Coin` enum variant for `Quarter`.

Matching with Option<T>

In the previous section, we wanted to get the inner `T` value out of the Some case when using `Option<T>`; we can also handle `Option<T>` using `match` as we did with the `Coin` enum! Instead of comparing coins, we’ll compare the variants of `Option<T>`, but the way that the match expression works remains the same.

Let’s say we want to write a function that takes an `Option<i32>` and, if there’s a value inside, adds 1 to that value. If there isn’t a value inside, the function should return the `None` value and not attempt to perform any operations.

This function is very easy to write, thanks to match, and will look like Listing 6-5.

```rs
#![allow(unused_variables)]
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

> Listing 6-5: A function that uses a match expression on an Option<i32>

Combining match and enums is useful in many situations. You’ll see this pattern a lot in Rust code: match against an enum, bind a variable to the data inside, and then execute code based on it. It’s a bit tricky at first, but once you get used to it, you’ll wish you had it in all languages. It’s consistently a user favorite.

Matches Are Exhaustive

There’s one other aspect of `match` we need to discuss. Consider this version of our `plus_one` function that has a bug and won’t compile:

```rs
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

We didn’t handle the `None` case, so this code will cause a bug. Luckily, it’s a bug Rust knows how to catch. If we try to compile this code, we’ll get this error:

```
error[E0004]: non-exhaustive patterns: `None` not covered
 -->
  |
6 |         match x {
  |               ^ pattern `None` not covered
```

Rust knows that we didn’t cover every possible case and even knows which pattern we forgot! Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid. Especially in the case of `Option<T>`, when Rust prevents us from forgetting to explicitly handle the `None` case, it protects us from assuming that we have a value when we might have null

The `_` Placeholder

Rust also has a pattern we can use when we don’t want to list all possible values. For example, a u8 can have valid values of 0 through 255. If we only care about the values 1, 3, 5, and 7, we don’t want to have to list out 0, 2, 4, 6, 8, 9 all the way up to 255. Fortunately, we don’t have to: we can use the special pattern `_` instead:

```rs

#![allow(unused_variables)]
fn main() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
```

The `_` pattern will match any value. By putting it after our other arms, the `_` will match all the possible cases that aren’t specified before it. The `()` is just the unit value, so nothing will happen in the `_` case. As a result, we can say that we want to do nothing for all the possible values that we don’t list before the `_` placeholder.

However, the match expression can be a bit wordy in a situation in which we care about only one of the cases. For this situation, Rust provides `if let`.

### 6.3. Concise Control Flow with if let

The `if let` syntax lets you combine `if` and `let` into a less verbose way to handle values that match one pattern while ignoring the rest. Consider the program in Listing 6-6 that matches on an `Option<u8>` value but only wants to execute code if the value is 3.

```rs
#![allow(unused_variables)]
fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
}
```

> Listing 6-6: A `match` that only cares about executing code when the value is `Some(3)`

We want to do something with the `Some(3)` match but do nothing with any other `Some<u8>` value or the `None` value. To satisfy the `match` expression, we have to add `_ => ()` after processing just one variant, which is a lot of boilerplate code to add.

Instead, we could write this in a shorter way using `if let`. The following code behaves the same as the match in Listing 6-6:

```rs
#![allow(unused_variables)]
fn main() {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
```

The syntax `if let` takes a pattern and an expression separated by an equal sign. It works the same way as a `match`, where the expression is given to the `match` and the pattern is its first arm.

Using `if let` means less typing, less indentation, and less boilerplate code. However, you lose the exhaustive checking that `match` enforces. Choosing between `match` and `if let` depends on what you’re doing in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.

In other words, you can think of `if let` as syntax sugar for a `match` that runs code when the value matches one pattern and then ignores all other values.

We can include an `else` with an `if let`. The block of code that goes with the `else` is the same as the block of code that would go with the `_` case in the match expression that is equivalent to the `if let` and `else`. 

```rs
#![allow(unused_variables)]
fn main() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}
```

Or we could use an if let and else expression like this:

```rs

#![allow(unused_variables)]
fn main() {
    #[derive(Debug)]
        enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
```

Summary

We’ve now covered how to use enums to create custom types that can be one of a set of enumerated values. We’ve shown how the standard library’s `Option<T>` type helps you use the type system to prevent errors. When enum values have data inside them, you can use `match` or `if let` to extract and use those values, depending on how many cases you need to handle.

Your Rust programs can now express concepts in your domain using structs and enums. Creating custom types to use in your API ensures type safety: the compiler will make certain your functions get only values of the type each function expects.

In order to provide a well-organized API to your users that is straightforward to use and only exposes exactly what your users will need, let’s now turn to Rust’s modules.


## 7. Managing Growing Projects with Packages, Crates, and Modules

when project grows and becomes complex you may need to: grouping functionality and encapsulating implementation.

A related concept is scope: the nested context in which code is written has a set of names that are defined as “in scope.” When reading, writing, and compiling code, programmers and compilers need to know whether a particular name at a particular spot refers to a variable, function, struct, enum, module, constant, or other item and what that item means. You can create scopes and change which names are in or out of scope. You can’t have two items with the same name in the same scope

Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the *module system*, include:

* **Packages:** A Cargo feature that lets you build, test, and share crates
* **Crates:** A tree of modules that produces a library or executable
* **Modules** and **use:** Let you control the organization, scope, and privacy of paths
* **Paths:** A way of naming an item, such as a struct, function, or module

### 7.1. Packages and Crates

A crate is a binary or library. The *crate root* is a source file that the Rust compiler starts from and makes up the root module of your crate. A package is one or more crates that provide a set of functionality. A package contains a Cargo.toml file that describes how to build those crates.

Several rules determine what a package can contain. A package *must* contain zero or one library crates, and no more. It can contain as many binary crates as you’d like, but it must contain at least one crate (either library or binary).

Let’s walk through what happens when we create a package. First, we enter the command `cargo new`:

```sh
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package. Likewise, If the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root. Cargo passes the crate root files to rustc to build the library or binary.

A crate will group related functionality together in a scope so the functionality is easy to share between multiple projects. For example, when we bring the `rand` crate into our project’s scope. All the functionality provided by the `rand` crate is accessible through the crate’s name, `rand`.

Keeping a crate’s functionality in its own scope clarifies whether particular functionality is defined in our crate or the rand crate and prevents potential conflicts.

### 7.2. Defining Modules to Control Scope and Privacy

In this section, we’ll talk about modules and other parts of the module system, namely *paths* that allow you to name items; the `use` keyword that brings a path into scope; and the `pub` keyword to make items public. We’ll also discuss the as keyword, external packages, and the glob operator. 

*Modules* let us organize code within a crate into groups for readability and easy reuse. Modules also control the *privacy* of items, which is whether an item can be used by outside code (*public*) or is an internal implementation detail and not available for outside use (*private*).

Modules can hold definitions for other items, such as structs, enums, constants, traits, functions and modules.

Let's take restaurant as an example, Some parts in a restaurant are referred to as *front of house* and others as *back of house*. Front of house is where customers are; this is where hosts seat customers, servers take orders and payment, and bartenders make drinks. Back of house is where the chefs and cooks work in the kitchen, dishwashers clean up, and managers do administrative work.

To structure our crate in the same way that a real restaurant works, we can organize our code into nested modules. Create a new library named `restaurant` by running `cargo new --lib restaurant`; then put the code in Listing 7-1 into src/lib.rs to define some modules and function signatures.

```rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

> Listing 7-1: A `front_of_house` module containing other modules that then contain functions

Here we define a module `front_of_house` and nested modules `hosting` and `serving`. an and the below is the module tree

```console
crate  # entire module tree is rooted under the implicit module named crate.
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

> Listing 7-2: The module tree for the code in Listing 7-1

Earlier, we mentioned that `src/main.rs` and `src/lib.rs` are called crate roots. The reason for their name is that the contents of either of these two files form a module named `crate` at the root of the crate’s module structure, known as the *module tree*.

The module tree might remind you of the filesystem’s directory tree on your computer; this is a very apt comparison! Just like directories in a filesystem, you use modules to organize your code. And just like files in a directory, we need a way to find our modules.

### 7.3. Paths for Referring to an Item in the Module Tree

To show Rust where to find an item in a module tree, we use a path in the same way we use a path when navigating a filesystem. If we want to call a function, we need to know its path.

A path can take two forms:

* An *absolute path* starts from a crate root by using a crate name or a literal `crate`.
* A *relative path* starts from the current module and uses `self`, `super`, or an identifier in the current module.
Both absolute and relative paths are followed by one or more identifiers separated by double colons (`::`).

Let’s try to compile Listing 7-3 and find out why it won’t compile yet! The error we get is shown in Listing 7-4.

```console
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: module `hosting` is private
 --> src/lib.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^

error[E0603]: module `hosting` is private
  --> src/lib.rs:12:21
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^
```

> Listing 7-4: Compiler errors from building the code in Listing 7-3

The error messages say that module `hosting` is private. In other words, we have the correct paths for the `hosting` module and the `add_to_waitlist` function, but Rust won’t let us use them because it doesn’t have access to the private sections.

Modules aren’t useful only for organizing your code. They also define Rust’s privacy boundary: the line that encapsulates the implementation details external code isn’t allowed to know about, call, or rely on. So, if you want to make an item like a function or struct private, you put it in a module.

The way privacy works in Rust is that all items (functions, methods, structs, enums, modules, and constants) are private by default. **Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules.** The reason is that child modules wrap and hide their implementation details, but the child modules can see the context in which they’re defined. 

But you can expose inner parts of child modules code to outer ancestor modules by using the pub keyword to make an item public.

Exposing Paths with the `pub` Keyword

```rs
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

> Listing 7-5: Declaring the hosting module as `pub` to use it from `eat_at_restaurant`

```console
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: function `add_to_waitlist` is private
 --> src/lib.rs:9:37
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                                     ^^^^^^^^^^^^^^^

error[E0603]: function `add_to_waitlist` is private
  --> src/lib.rs:12:30
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                              ^^^^^^^^^^^^^^^
```
> Listing 7-6: Compiler errors from building the code in Listing 7-5

Adding the pub keyword in front of mod hosting makes the module public. But the contents of hosting are still private; making the module public doesn’t make its contents public. The pub keyword on a module only lets code in its ancestor modules refer to it

Let’s also make the add_to_waitlist function public by adding the pub keyword before its definition, as in Listing 7-7.

```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
fn main() {}
```

Starting Relative Paths with `super`

In file system, we can access parent directory using .., likewise in rust we can access parent module using keyword `super`

```rs
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
fn main() {}
```

> Listing 7-8: Calling a function using a relative path starting with `super`

Making Structs and Enums Public

We can also use `pub` to designate structs and enums as public, but there are a few extra details. If we use `pub` before a struct definition, we make the struct public, but the struct’s fields will still be private. We can make each field public or not on a case-by-case basis.


```rs
#![allow(unused_variables)]
fn main() {
    mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }
    }

    pub fn eat_at_restaurant() {
        // Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");
    }
}
```

> Listing 7-9: A struct with some public fields and some private fields

In contrast, if we make an enum public, all of its variants are then public. We only need the `pub` before the `enum` keyword

```rs

#![allow(unused_variables)]
fn main() {
    mod back_of_house {
        pub enum Appetizer {
            Soup,
            Salad,
        }
    }

    pub fn eat_at_restaurant() {
        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
}
```

> Listing 7-10: Designating an `enum` as public makes all its variants public

Enums aren’t very useful unless their variants are public; it would be annoying to have to annotate all enum variants with `pub` in every case, so the default for enum variants is to be public. Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with `pub`.

### 7.4. Bringing Paths Into Scope with the `use` Keyword

Using path to refer to item in module tree is verbose and inconvenient. We can bring a path into a scope once and then use items in that path as if they're local items with the `use` keyword.

```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
fn main() {}
```

> Listing 7-11: Bringing a module into scope with `use`

Adding use and a path in a scope is similar to creating a symbolic link in the filesystem. 

Creating Idiomatic `use` Paths

```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist; // do this :)

use crate::front_of_house::hosting::add_to_waitlist; // don't do this :(

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
fn main() {}
```

> Listing 7-13: Bringing the `add_to_waitlist` function into scope with use, which is `unidiomatic`

Bringing the function’s parent module into scope with `use` so we have to specify the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path. The code in Listing 7-13 is unclear as to where `add_to_waitlist` is defined.

On the other hand, when bringing in structs, enums, and other items with `use`, it’s idiomatic to specify the full path. Listing 7-14 shows the idiomatic way to bring the standard library’s HashMap struct into the scope of a binary crate.

```rs
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```
> Listing 7-14: Bringing `HashMap` into scope in an idiomatic way

Providing New Names with the `as` Keyword

If we’re bringing two items with the same name into scope with use statements, we can use 'as' keyword after the path to create a new local name or alias for the item.

```rs
#![allow(unused_variables)]
fn main() {
    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function1() -> Result {
        // --snip--
        Ok(())
    }

    fn function2() -> IoResult<()> {
        // --snip--
        Ok(())
    }
}
```

> Listing 7-16: Renaming a type when it’s brought into scope with the `as` keyword

Re-exporting Names with `pub use`

When we bring a name into scope with the `use` keyword, the name available in the new scope is private. To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine `pub` and `use`. This technique is called *re-exporting* because we’re bringing an item into scope but also making that item available for others to bring into their scope.

```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
fn main() {}
```

> Listing 7-17: Making a name available for any code to use from a new scope with `pub use`

By using `pub use`, external code can now call the `add_to_waitlist` function using `hosting::add_to_waitlist`. If we hadn’t specified `pub use`, the `eat_at_restaurant` function could call `hosting::add_to_waitlist` in its scope, but external code couldn’t take advantage of this new path.

Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain. For example, in this restaurant metaphor, the people running the restaurant think about “front of house” and “back of house.” But customers visiting a restaurant probably won’t think about the parts of the restaurant in those terms. With pub use, we can write our code with one structure but expose a different structure. Doing so makes our library well organized for programmers working on the library and programmers calling the library.

Using External Packages

add external package dependency in *Cargo.toml*:  

```toml
[dependencies]
rand = "0.5.5"
```

Adding rand as a dependency in Cargo.toml tells Cargo to download the rand package and any dependencies from crates.io and make rand available to our project.

Then, to bring `rand` definitions into the scope of our package, we added a `use` line starting with the name of the package, `rand`, and listed the items we wanted to bring into scope. 

```rs
use rand::Rng;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
}
```

Note that the standard library (`std`) is also a crate that’s external to our package. Because the standard library is shipped with the Rust language, we don’t need to change *Cargo.toml* to include `std`. But we do need to refer to it with `use` to bring items from there into our package’s scope. 

Using Nested Paths to Clean Up Large `use` Lists

If we’re using multiple items defined in the same package or same module, listing each item on its own line can take up a lot of vertical space in our files.

```rs

#![allow(unused_variables)]
fn main() {
    use std::io;
    use std::cmp::Ordering;
    // ---snip---
}
```

```rs

#![allow(unused_variables)]
fn main() {
    use std::{cmp::Ordering, io};
    // ---snip---
}
```

> Listing 7-18: Specifying a nested path to bring multiple items with the same prefix into scope

The Glob Operator

If we want to bring all public items defined in a path into scope, we can specify that path followed by `*`, the glob operator:

```rs

#![allow(unused_variables)]
fn main() {
    use std::collections::*;
}
```

This `use` statement brings all public items defined in `std::collections` into the current scope. Be careful when using the glob operator! Glob can make it harder to tell what names are in scope and where a name used in your program was defined.

The glob operator is often used when testing to bring everything under test into the `tests` module;

### 7.5. Separating Modules into Different Files

So far, all the examples in this chapter defined multiple modules in one file. When modules get large, you might want to move their definitions to a separate file to make the code easier to navigate.

```rs
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

> Listing 7-21: Declaring the `front_of_house` module whose body will be in *src/front_of_house.rs*

And src/front_of_house.rs gets the definitions from the body of the front_of_house module, as shown in Listing 7-22.

```rs

#![allow(unused_variables)]
fn main() {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
```
> Listing 7-22: Definitions inside the `front_of_house` module in *src/front_of_house.rs*

Using a semicolon after `mod front_of_house` rather than using a block tells Rust to load the contents of the module from another file with the same name as the module. 

The `mod` keyword declares modules, and Rust looks in a file with the same name as the module for the code that goes into that module.

Summary

Rust lets you split a package into multiple crates and a crate into modules so you can refer to items defined in one module from another module. You can do this by specifying absolute or relative paths. These paths can be brought into scope with a `use` statement so you can use a shorter path for multiple uses of the item in that scope. Module code is private by default, but you can make definitions public by adding the `pub` keyword.

## 8. Common Collections

* A vector allows you to store a variable number of values next to each other.
* A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
* A hash map allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.

### 8.1. Storing Lists of Values with Vectors

`Vec<T>`, also known as a vector allows you to store more than one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the same type. 

Creating a New Vector

```rs
#![allow(unused_variables)]
fn main() {
    let v: Vec<i32> = Vec::new();
}
```

> Listing 8-1: Creating a new, empty vector to hold values of type `i32`

```rs
#![allow(unused_variables)]
fn main() {
    let v = vec![1, 2, 3]; // rust can infer that the type of element in vector v
}
```

> Listing 8-2: Creating a new vector containing values

Updating a Vector

add element by `push`

```rs
#![allow(unused_variables)]
fn main() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}
```

> Listing 8-3: Using the `push` method to add values to a vector

Dropping a Vector Drops Its Elements

a vector is freed when it goes out of scope

```rs
#![allow(unused_variables)]
fn main() {
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v

    } // <- v goes out of scope and is freed here
}
```
> Listing 8-4: Showing where the vector and its elements are dropped

When the vector gets dropped, all of its contents are also dropped, meaning those integers it holds will be cleaned up. This may seem like a straightforward point but can get a bit more complicated when you start to introduce references to the elements of the vector.

Reading Elements of Vectors

```rs
#![allow(unused_variables)]
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
```
> Listing 8-5: Using indexing syntax or the get method to access an item in a vector

Vectors are indexed by number, starting at zero

* by using & and [], which gives us a reference
* by using the get method with the index passed as an argument, which gives us an Option<&T>

When access a non-existent element from vector:

* &[] will cause panic
* get will return None

```rs
#![allow(unused_variables)]
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100]; // panic
    let does_not_exist = v.get(100); // return None
}
```

> Listing 8-6: Attempting to access the element at index 100 in a vector containing five elements

When the program has a valid reference, the borrow checker enforces the ownership and borrowing rules (covered in Chapter 4) to ensure this reference and any other references to the contents of the vector remain valid. Recall the rule that states you can’t have mutable and immutable references in the same scope. That rule applies in Listing 8-7, where we hold an immutable reference to the first element in a vector and try to add an element to the end, which won’t work.

```rs
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6); // may cause the re-allocate memory and the reference to first element will become invalid

println!("The first element is: {}", first);
```

> Listing 8-7: Attempting to add an element to a vector while holding a reference to an item

Compiling this code will result in this error:

```console
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let first = &v[0];
  |                  - immutable borrow occurs here
5 |
6 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println!("The first element is: {}", first);
  |                                          ----- immutable borrow later used here
```

The code in Listing 8-7 might look like it should work: why should a reference to the first element care about what changes at the end of the vector? This error is due to the way vectors work: adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector currently is. In that case, the reference to the first element would be pointing to deallocated memory. 

Iterating over the Values in a Vector

```rs

#![allow(unused_variables)]
fn main() {
let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i); // i is immutable reference
    }
}
```

> Listing 8-8: Printing each element in a vector by iterating over the elements using a for loop

We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements. The for loop in Listing 8-9 will add 50 to each element.

```rs
#![allow(unused_variables)]
fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}
```

> Listing 8-9: Iterating over mutable references to elements in a vector

To change the value that the mutable reference refers to, we have to use the dereference operator (`*`) to get to the value in i before we can use the `+=` operator. 

Using an Enum to Store Multiple Types

Vectors can only store values that are the same type. When we want to save elements of a different type in a vector, we can define an enum whose variants will hold the different value types, and then all the variants will be considered the same type. 

```rs

#![allow(unused_variables)]
fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
```

> Listing 8-10: Defining an `enum` to store values of different types in one vector


Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element. A secondary advantage is that we can be explicit about what types are allowed in this vector. If Rust allowed a vector to hold any type, there would be a chance that one or more of the types would cause errors with the operations performed on the elements of the vector. Using an enum plus a `match` expression means that Rust will ensure at compile time that every possible case is handled, as discussed in Chapter 6.

When you’re writing a program, if you don’t know the exhaustive set of types the program will get at runtime to store in a vector, the enum technique won’t work. Instead, you can use a trait object, which we’ll cover in Chapter 17.

### 8.2. Storing UTF-8 Encoded Text with Strings

 Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str`.

 The `String` type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type. When Rustaceans refer to “strings” in Rust, they usually mean the `String` and the string slice `&str` types, not just one of those types.

Creating a New String

```rs
#![allow(unused_variables)]
fn main() {
    let mut s = String::new(); //creates a new empty string called s, which we can then load data into. 
}
```
> Listing 8-11: Creating a new, empty `String`

```rs

#![allow(unused_variables)]
fn main() {
    let data = "initial contents";

    let s = data.to_string(); // will create a String value from the string literal

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
}
```
> Listing 8-12: Using the `to_string` method to create a `String` from a string literal

Updating a String

A `String` can grow in size and its contents can change, just like the contents of a `Vec<T>`, if you push more data into it. In addition, you can conveniently use the `+` operator or the `format!` macro to concatenate `String` values.

Appending to a String with `push_str` and `push`

```rs
#![allow(unused_variables)]
fn main() {
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2); // push_str take a string slice as parameter, hence doesn't take the ownership of passing in parameter
    println!("s2 is {}", s2);
}
```

> Listing 8-16: Using a string slice after appending its contents to a `String`

the `push` method takes a single character as parameter and adds it to `String`. 

```rs
#![allow(unused_variables)]
fn main() {
    let mut s = String::from("lo");
    s.push('l');
}
```

> Listing 8-17: Adding one character to a `String` value using `push`

Concatenation with the `+` Operator or the `format!` Macro

```rs
#![allow(unused_variables)]
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
}
```
> Listing 8-18: Using the `+` operator to combine two `String` values into a new `String` value

The `+` operator uses the `add` method, whose signature looks something like this:

```rs
    fn add(self, s: &str) -> String;
```

When we need to concatenate multiple strings, we can use `format!`

```rs
#![allow(unused_variables)]
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
}
```

Indexing into Strings

In rust we can't index a `String`, becauase `String`'s intenal representation is `Vec<u8>`, and some `String` contains muti-bytes characters.

```rs
let s1 = String::from("hello");
let h = s1[0];

// error[E0277]: the trait bound `std::string::String: std::ops::Index<{integer}>` is not satisfied
//  -->
//   |
// 3 |     let h = s1[0];
//   |             ^^^^^ the type `std::string::String` cannot be indexed by `{integer}`
//   |
//   = help: the trait `std::ops::Index<{integer}>` is not implemented for `std::string::String`

```

> Listing 8-19: Attempting to use indexing syntax with a `String`

Internal Representation

A `String` is a wrapper over a `Vec<u8>`. 

```rs
#![allow(unused_variables)]
fn main() {
    let len = String::from("Hola").len();
}
```

In this case, len will be 4, which means the vector storing the string “Hola” is 4 bytes long. Each of these letters takes 1 byte when encoded in UTF-8.

```rs
#![allow(unused_variables)]
fn main() {
    let len = String::from("世界").len();
    let hello = "世界";
    let answer = &hello[0]; // compile error, rust think it's meaningless to acces the first byte and may cause bugs
}
```

Bytes and Scalar Values and Grapheme Clusters! Oh My!

Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust’s perspective: as bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters).

A final reason Rust doesn’t allow us to index into a `String` to get a character is that indexing operations are expected to always take constant time (O(1)). But it isn’t possible to guarantee that performance with a `String`, because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.

Slicing Strings

Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice. Therefore, Rust asks you to be more specific if you really need to use indices to create string slices. To be more specific in your indexing and indicate that you want a string slice, rather than indexing using [] with a single number, you can use [] with a range to create a string slice containing particular bytes:

```rs
#![allow(unused_variables)]
fn main() {
    let hello = "世界";

    let s = &hello[0..5];

    let s1 = &hello[0..1]; // will panic at runtime, if the slice cause broken characters
}
```

You should use ranges to create string slices with caution, because doing so can crash your program.

Methods for Iterating Over Strings

Fortunately, you can access elements in a string in other ways.

```rs
#![allow(unused_variables)]
fn main() {
    for c in "世界".chars() { // iterate over char sequence for the string
        println!("{}", c);
    }

    for c in "世界".bytes() { // iterate over bytes sequence for the string
        println!("{}", c);
    }
}
```

Strings Are Not So Simple

### 8.3. Storing Keys with Associated Values in Hash Maps

Creating a New Hash Map

```rs
#![allow(unused_variables)]
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
```

> Listing 8-20: Creating a new hash map and inserting some keys and values

Another way of constructing a hash map is by using the `collect` method on a vector of tuples, where each tuple consists of a key and its value. The `collect` method gathers data into a number of collection types, including `HashMap`. 

```rs

#![allow(unused_variables)]
fn main() {
    use std::collections::HashMap;

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}
```

> Listing 8-21: Creating a hash map from a list of teams and a list of scores

The type annotation `HashMap<_, _>` is needed here because it’s possible to `collect` into many different data structures and Rust doesn’t know which you want unless you specify. For the parameters for the key and value types, however, we use underscores, and Rust can infer the types that the hash map contains based on the types of the data in the vectors.

Hash Maps and Ownership

For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map. For owned values like `String`, the values will be moved and the hash map will be the owner of those values, as demonstrated in Listing 8-22.

```rs
#![allow(unused_variables)]
fn main() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}
```

> Listing 8-22: Showing that keys and values are owned by the hash map once they’re inserted

We aren’t able to use the variables `field_name` and `field_value` after they’ve been moved into the hash map with the call to `insert`.

If we insert references to values into the hash map, the values won’t be moved into the hash map. The values that the references point to must be valid for at least as long as the hash map is valid.

Accessing Values in a Hash Map

```rs
#![allow(unused_variables)]
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
}
```

Iterate over a map

```rs
#![allow(unused_variables)]
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
```

Updating a Hash Map

need handle case when the key-value pair not exists

Overwriting a Value

when insert different value for the same key, map will only keep the last value for the key

```rs

#![allow(unused_variables)]
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // overwrites the previous value

    println!("{:?}", scores);
}
```

> Listing 8-24: Replacing a value stored with a particular key

Only Inserting a Value If the Key Has No Value

```rs

#![allow(unused_variables)]
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}
```

> Listing 8-25: Using the `entry` method to only insert if the key does not already have a value

Updating a Value Based on the Old Value

```rs

#![allow(unused_variables)]
fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // insert conditionaly and return a mutable reference to current value
        *count += 1;
    }

    println!("{:?}", map);
}
```

> Listing 8-26: Counting occurrences of words using a hash map that stores words and counts

Hashing Functions

By default, `HashMap` uses a “cryptographically strong”[1] hashing function that can provide resistance to Denial of Service (DoS) attacks. This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it. If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher. A *hasher* is a type that implements the `BuildHasher` trait.

[1] https://www.131002.net/siphash/siphash.pdf

## 9. Error Handling

Rust’s commitment to reliability extends to error handling. 

Rust groups errors into two major categories: `recoverable` and `unrecoverable` errors. For a recoverable error, such as a file not found error, it’s reasonable to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.

Rust has the type Result<T, E> for recoverable errors and the panic! macro that stops execution when the program encounters an unrecoverable error. 

### 9.1. Unrecoverable Errors with panic!

When the panic! macro executes, your program will print a failure message, unwind and clean up the stack, and then quit. 

```rs
fn main() {
    panic!("crash and burn");
}
```

```console
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

Using a `panic!` Backtrace

```rs
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

> Listing 9-1: Attempting to access an element beyond the end of a vector, which will cause a call to `panic!`

A *backtrace* is a list of all the functions that have been called to get to this point. 

```sh
$ RUST_BACKTRACE=1 cargo run
```

### 9.2. Recoverable Errors with Result

Rust use `Result<T,E> to represent a function call result that may succeed or fail(not panic)

```rs

#![allow(unused_variables)]
fn main() {
    enum Result<T, E> {
        Ok(T), // T represents the type of the value that will be returned in a success case within the Ok variant
        Err(E), // E represents the type of the error that will be returned in a failure case within the Err variant.
    }
}
```

Open a file that may succeed or fail

```rs
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };
}
```

> Listing 9-4: Using a `match` expression to handle the Result variants that might be returned


**Matching on Different Errors**

If the function can report different errors, we can have a inner match in the `Err` block

```rs
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
```

The type of the value that `File::open` returns inside the Err variant is `io::Error`, which is a struct provided by the standard library. This struct has a method kind that we can call to get an `io::ErrorKind` value. The enum `io::ErrorKind` is provided by the standard library and has variants representing the different kinds of errors that might result from an io operation. 

**Shortcuts for Panic on Error: `unwrap` and `expect`**

Using `match` works well enough, but it can be a bit verbose and doesn’t always communicate intent well. The `Result<T, E>` type has many helper methods defined on it to do various tasks. One of those methods, called `unwrap`, is a shortcut method that is implemented just like the match expression. If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`. If the `Result` is the `Err` variant, `unwrap` will call the `panic!` macro for us.

```rs
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

Another method, expect, which is similar to unwrap, lets us also choose the panic! error message. Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier.

```rs
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

**Propagating Errors**

When you’re writing a function whose implementation calls something that might fail, instead of handling the error within this function, you can return the error to the calling code so that it can decide what to do. This is known as *propagating* the error and gives more control to the calling code

For example, Listing 9-6 shows a function that reads a username from a file. If the file doesn’t exist or can’t be read, this function will return those errors to the code that called this function.

```rs
#![allow(unused_variables)]
fn main() {
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e), // propagating error to the caller
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),   // if succeed, return result to the caller
            Err(e) => Err(e), // propagating error to the caller
        }
    }
}

```

> Listing 9-6: A function that returns errors to the calling code using `match`

 Let’s look at the return type of the function first: `Result<String, io::Error>`. This means the function is returning a value of the type `Result<T, E> `. If this function succeeds without any problems, the code that calls this function will receive an `Ok` value that holds a String—-the username that this function read from the file. If this function encounters any problems, the code that calls this function will receive an `Err` value that holds an instance of `io::Error` that contains more information about what the problems were. 

This pattern of propagating errors is so common in Rust that Rust provides the question mark operator `?` to make this easier.

**A Shortcut for Propagating Errors: the `?` Operator**

```rs
#![allow(unused_variables)]
fn main() {
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?; // ? here, means that if succeed, the result will return from expression, if failed, error will be returned from the whole function
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
}
```

> Listing 9-7: A function that returns errors to the calling code using the `?` operator

If the value of the Result is an `Ok`, the value inside the `Ok` will get returned from this expression, and the program will continue. If the value is an `Err`, the `Err` will be returned from the whole function as if we had used the `return` keyword so the error value gets propagated to the calling code.

There is a difference between what the `match` expression from Listing 9-6 and the `?` operator do: There is a difference between what the match expression from Listing 9-6 and the `?` operator do: error values that have the `?` operator called on them go through the `from` function, defined in the `From` trait in the standard library, which is used to convert errors from one type into another. When the `?` operator calls the from function, the error type received is converted into the error type defined in the return type of the current function. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons. As long as each error type implements the from function to define how to convert itself to the returned error type, the `?` operator takes care of the conversion automatically.

The `?` operator eliminates a lot of boilerplate and makes this function’s implementation simpler. We could even shorten this code further by chaining method calls immediately after the `?`, as shown in Listing 9-8.

```rs
#![allow(unused_variables)]
fn main() {
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?; // if  open failed, then propagating error to caller, otherwise continue call read_to_string method

        Ok(s)
    }
}
```

> Listing 9-8: Chaining method calls after the `?` operator


```rs
#![allow(unused_variables)]
fn main() {
    use std::io;
    use std::fs;

    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
}
```

> Listing 9-9: Using `fs::read_to_string` instead of opening and then reading the file

**The `?` Operator Can Be Used in Functions That Return `Result`**

The `?` operator can be used in functions that have a return type of `Result`, because it is defined to work in the same way as the `match` expression we defined in Listing 9-6. The part of the match that requires a return type of `Result` is return `Err(e)`, so the return type of the function can be a `Result` to be compatible with this `return`.

 We’re only allowed to use the `?` operator in a function that returns `Result` or `Option` or another type that implements `std::ops::Try`

```rs
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?; // Error, the main function return type not statisfied the restriction on `?`
}
```

### 9.3. To panic! or Not To panic!

So how do you decide when you should call `panic!` and when you should return `Result`? When code panics, there’s no way to recover. You could call panic! for any error situation, whether there’s a possible way to recover or not, but then you’re making the decision on behalf of the code calling your code that a situation is unrecoverable. When you choose to return a `Result` value, you give the calling code options rather than making the decision for it. The calling code could choose to attempt to recover in a way that’s appropriate for its situation, or it could decide that an Err value in this case is unrecoverable, so it can call `panic!` and turn your recoverable error into an unrecoverable one. Therefore, returning `Result` is a good default choice when you’re defining a function that might fail.

**Guidelines for Error Handling**

It’s advisable to have your code panic when it’s possible that your code could end up in a *bad state*. In this context, a bad state is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code—plus one or more of the following:

* The bad state is not something that’s expected to happen occasionally.
* Your code after this point needs to rely on not being in this bad state.
* There’s not a good way to encode this information in the types you use.

If someone calls your code and passes in values that don’t make sense, the best choice might be to call `panic!` and alert the person using your library to the bug in their code so they can fix it during development. Similarly, `panic!` is often appropriate if you’re calling external code that is out of your control and it returns an invalid state that you have no way of fixing.

However, when failure is expected, it’s more appropriate to return a `Result` than to make a `panic!` call. Examples include a parser being given malformed data or an HTTP request returning a status that indicates you have hit a rate limit. In these cases, returning a `Result` indicates that failure is an expected possibility that the calling code must decide how to handle.

**Creating Custom Types for Validation**

```rs
loop {
    // --snip--

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 { // validation may happens anywhere
        println!("The secret number will be between 1 and 100.");
        continue;
    }

    match guess.cmp(&secret_number) {
    // --snip--
}
```

We can make a new type and put the validations in a function to create an instance of the type rather than repeating the validations everywhere. 

```rs
#![allow(unused_variables)]
fn main() {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess { // add documentation for the possibility of panic
            if value < 1 || value > 100 {  // add validation to the new function when creating a instance, to avoid vaildations everywhere
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess {
                value
            }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
```

Summary

Rust’s error handling features are designed to help you write more robust code. The panic! macro signals that your program is in a state it can’t handle and lets you tell the process to stop instead of trying to proceed with invalid or incorrect values. The Result enum uses Rust’s type system to indicate that operations might fail in a way that your code could recover from. You can use Result to tell code that calls your code that it needs to handle potential success or failure as well. 

## 10. Generic Types, Traits, and Lifetimes

### 10.1. Generic Data Types

We can use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types. 

**In Function Definitions**

When defining a function that uses generics, we place the generics in the signature of the function where we would usually specify the data types of the parameters and return value. 

```rs
fn largest<T>(list: &[T]) -> T { // T is generic type for parameter and return value
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list); // compile error: because character doesn't support > operator
    println!("The largest char is {}", result);
}
```

> Listing 10-5: A definition of the largest function that uses generic type parameters but doesn’t compile yet


**In Struct Definitions**

We can also define structs to use a generic type parameter in one or more fields using the <> syntax.

```rs
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

To define a Point struct where x and y are both generics but could have different types, we can use multiple generic type parameters.

```rs
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

**In Enum Definitions**

```rs
#![allow(unused_variables)]
fn main() {
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

}
```

**In Method Definitions**

We can implement methods on structs and enums (as we did in Chapter 5) and use generic types in their definitions, too.

```rs
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> { // By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

> Listing 10-9: Implementing a method named `x` on the `Point<T>` struct that will return a reference to the `x` field of type `T`

By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.

Generic type parameters in a struct definition aren’t always the same as those you use in that struct’s method signatures. For example, Listing 10-11 defines the method `mixup` on the `Point<T, U>` struct from Listing 10-8. The method takes another `Point` as a parameter, which might have different types from the `self Point` we’re calling `mixup` on. The method creates a new Point instance with the `x` value from the `self Point` (of type `T`) and the `y` value from the passed-in `Point` (of type `W`). 

```rs
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> { // other is another type of Point, return value is a third type of Point
        Point { // a mixed type of point from self and other
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

> Listing 10-11: A method that uses different generic types from its struct’s definition

**Performance of Code Using Generics**

Using generic will have no extra performance cost at runtime in Rust. Rust will turn generic code to specific code by filling in the concrete types at compile time. This process is called *Monomorphization *

### 10.2. Traits: Defining Shared Behavior

A `trait` tells the Rust compiler about functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic can be any type that has certain behavior.

> Note: Traits are similar to a feature often called `interfaces` in other languages, although with some differences.

**Defining a Trait**

A type’s behavior consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods on all of those types. Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

For example, we define a trait that have a method that can summarize news contents

```rs
#![allow(unused_variables)]
fn main() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }
}
```

> Listing 10-12: A Summary `trait` that consists of the behavior provided by a `summarize` method

**Implementing a Trait on a Type**

We can implements the trait for different kinds of news: NewsArticle, Tweet, etc.

```rs

#![allow(unused_variables)]
fn main() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
}
```

> Listing 10-13: Implementing the `Summary` trait on the `NewsArticle` and `Tweet` types

Implementing a trait on a type is similar to implementing regular methods. The difference is that after `impl`, we put the trait name that we want to implement, then use the `for` keyword, and then specify the name of the type we want to implement the trait for. 

After implementing the trait, we can call the methods on instances of `NewsArticle` and `Tweet` in the same way we call regular methods, like this:

```rs
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summarize());
```

One restriction to note with trait implementations is that we can implement a trait on a type only if either the trait or the type is local to our crate. That is we can’t implement external traits on external types. This restriction is part of a property of programs called *coherence*, and more specifically the *orphan rule*, so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.

**Default Implementations**

Sometimes it’s useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every type. Then, as we implement the trait on a particular type, we can keep or override each method’s default behavior.

Listing 10-14 shows how to specify a default string for the `summarize` method of the `Summary` trait instead of only defining the method signature, as we did in Listing 10-12.

```rs
#![allow(unused_variables)]
fn main() {
    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)") // default impelmentation
        }
    }
}
```

> Listing 10-14: Definition of a `Summary` trait with a default implementation of the `summarize` method

To use a default implementation to summarize instances of `NewsArticle` instead of defining a custom implementation, we specify an empty `impl` block with `impl Summary for NewsArticle {}`.

Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation. In this way, a trait can provide a lot of useful functionality and only require implementors to specify a small part of it.

```rs

#![allow(unused_variables)]
fn main() {
    pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author()) // a default method implementation call another method in the same trait
        }
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

}
```

**Traits as Parameters**

```rs
pub fn notify(item: impl Summary) { // a method that accept a trait as parameter
    println!("Breaking news! {}", item.summarize());
}
```

Instead of a concrete type for the `item` parameter, we specify the `impl` keyword and the trait name. This parameter accepts any type that implements the specified trait. 

**Trait Bound Syntax**

The `impl Trait` syntax works for straightforward cases but is actually syntax sugar for a longer form, which is called a *trait bound*; it looks like this:

```rs
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
```

This longer form is equivalent to the example in the previous section but is more verbose. 

**Specifying Multiple Trait Bounds with the `+` Syntax**

We can also specify more than one trait bound. Say we wanted `notify` to use display formatting on `item` as well as the `summarize` method: we specify in the notify definition that item must implement both `Display` and `Summary`. We can do so using the `+` syntax:

```rs
pub fn notify(item: impl Summary + Display) {
```

The `+` syntax is also valid with trait bounds on generic types:

```rs
pub fn notify<T: Summary + Display>(item: T) {
```

**Clearer Trait Bounds with `where` Clauses**

Using too many trait bounds has its downsides. Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list, making the function signature hard to read. For this reason, Rust has alternate syntax for specifying trait bounds inside a where clause after the function signature. So instead of writing this:

```rs
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
```
we can use a `where` clause, like this:

```rs
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

This function’s signature is less cluttered: the function name, parameter list, and return type are close together, similar to a function without lots of trait bounds.

**Returning Types that Implement Traits**

We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait, as shown here:

```rs
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
```

By using `impl Summary` for the return type, we specify that the `returns_summarizable` function returns some type that implements the `Summary` trait without naming the concrete type. In this case, `returns_summarizable` returns a `Tweet`, but the code calling this function doesn’t know that.

The ability to return a type that is only specified by the trait it implements is especially useful in the context of closures and iterators, which we cover in Chapter 13. Closures and iterators create types that only the compiler knows or types that are very long to specify. The `impl Trait` syntax lets you concisely specify that a function returns some type that implements the `Iterator` trait without needing to write out a very long type.

However, you can only use `impl Trait` if you’re returning a single type. For example, this code that returns either a `NewsArticle` or a `Tweet` with the return type specified as `impl Summary` wouldn’t work :x:

```rs
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}
```

Returning either a `NewsArticle` or a `Tweet` isn’t allowed due to restrictions around how the `impl Trait` syntax is implemented in the compiler. We’ll cover how to write a function with this behavior in the “Using Trait Objects That Allow for Values of Different Types” section of Chapter 17.

**Fixing the `largest` Function with Trait Bounds**

Now that you know how to specify the behavior you want to use using the generic type parameter’s bounds, let’s return to Listing 10-5 to fix the definition of the `largest` function that uses a generic type parameter! Last time we tried to run that code, we received this error:

```console
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:5:12
  |
5 |         if item > largest {
  |            ^^^^^^^^^^^^^^
  |
  = note: an implementation of `std::cmp::PartialOrd` might be missing for `T`
```

In the body of `largest` we wanted to compare two values of type `T` using the greater than (`>`) operator. Because that operator is defined as a default method on the standard library trait `std::cmp::PartialOrd`, we need to specify `PartialOrd` in the trait bounds for `T` so the largest function can work on slices of any type that we can compare. 

```rs
fn largest<T: PartialOrd>(list: &[T]) -> T {
```

This time when we compile the code, we get a different set of errors:

```console
error[E0508]: cannot move out of type `[T]`, a non-copy slice
 --> src/main.rs:2:23
  |
2 |     let mut largest = list[0];
  |                       ^^^^^^^
  |                       |
  |                       cannot move out of here
  |                       help: consider using a reference instead: `&list[0]`

error[E0507]: cannot move out of borrowed content
 --> src/main.rs:4:9
  |
4 |     for &item in list.iter() {
  |         ^----
  |         ||
  |         |hint: to prevent move, use `ref item` or `ref mut item`
  |         cannot move out of borrowed content
```

The key line in this error is cannot move out of type `[T]`, a non-copy slice. With our non-generic versions of the `largest` function, we were only trying to find the largest `i32` or `char`. As discussed in the “Stack-Only Data: Copy” section in Chapter 4, types like `i32` and `char` that have a known size can be stored on the stack, so they implement the `Copy` trait. But **when we made the `largest` function generic, it became possible for the list parameter to have types in it that don’t implement the `Copy` trait. Consequently, we wouldn’t be able to move the value out of `list[0]` and into the `largest` variable**, resulting in this error.

To call this code with only those types that implement the `Copy` trait, we can add `Copy` to the trait bounds of `T`!

```rs
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

> Listing 10-15: A working definition of the `largest` function that works on any generic type that implements the `PartialOrd` and `Copy` traits

Another way we could implement `largest` is for the function to return a reference to a `T` value in the slice. If we change the return type to `&T` instead of `T`, thereby changing the body of the function to return a reference, we wouldn’t need the `Clone` or `Copy` trait bounds and we could avoid heap allocations.

**Using Trait Bounds to Conditionally Implement Methods**

By using a trait bound with an `impl` block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits. For example, the type `Pair<T>` in Listing 10-16 always implements the `new` function. But `Pair<T>` only implements the `cmp_display` method if its inner type `T` implements the `PartialOrd` trait that enables comparison and the Display trait that enables printing.

```rs

#![allow(unused_variables)]
fn main() {
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self {
                x,
                y,
            }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}
```

> Listing 10-16: Conditionally implement methods on a generic type depending on trait bounds

We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called *blanket implementations* and are extensively used in the Rust standard library. For example, the standard library implements the `ToString` trait on any type that implements the `Display` trait. The `impl` block in the standard library looks similar to this code:

```rs
impl<T: Display> ToString for T {
    // --snip--
}
```

Because the standard library has this blanket implementation, we can call the `to_string` method defined by the ToString trait on any type that implements the `Display` trait. 

Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior. The compiler can then use the trait bound information to check that all the concrete types used with our code provide the correct behavior. 

Another kind of generic that we’ve already been using is called *lifetimes*. Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be. Let’s look at how lifetimes do that.

### 10.3. Validating References with Lifetimes

Every reference in Rust has a lifetime, which is the scope for which that reference is valid. Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred. We must annotate types when multiple types are possible. In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways. Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

The concept of lifetimes is somewhat different from tools in other programming languages, arguably making lifetimes Rust’s most distinctive feature. 

**Preventing Dangling References with Lifetimes**

The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference. For example:

```rs
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r); // Dangling References
}
```

> Listing 10-17: An attempt to use a reference whose value has gone out of scope

The outer scope declares a variable named `r` with no initial value, and the inner scope declares a variable named `x` with the initial value of `5`. Inside the inner scope, we attempt to set the value of `r` as a reference to `x`. Then the inner scope ends, and we attempt to print the value in `r`. This code won’t compile because the value `r` is referring to has gone out of scope before we try to use it. Here is the error message:

```console
error[E0597]: `x` does not live long enough
  --> src/main.rs:7:5
   |
6  |         r = &x;
   |              - borrow occurs here
7  |     }
   |     ^ `x` dropped here while still borrowed
...
10 | }
   | - borrowed value needs to live until here
```

**The Borrow Checker**

The Rust compiler has a *borrow checker* that compares scopes to determine whether all borrows are valid. Listing 10-18 shows the same code as Listing 10-17 but with annotations showing the lifetimes of the variables.

```rs
{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

> Listing 10-18: Annotations of the lifetimes of `r` and `x`, named `'a` and `'b`, respectively

Here, we’ve annotated the lifetime of `r` with `'a` and the lifetime of `x` with `'b`. As you can see, the inner `'b` block is much smaller than the outer `'a` lifetime block. At compile time, Rust compares the size of the two lifetimes and sees that `r` has a lifetime of `'a` but that it refers to memory with a lifetime of `'b`. The program is rejected because `'b` is shorter than `'a`: the subject of the reference doesn’t live as long as the reference.

Listing 10-19 fixes the code so it doesn’t have a dangling reference and compiles without any errors.

```rs
#![allow(unused_variables)]
fn main() {
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
}
```

> Listing 10-19: A valid reference because the data has a longer lifetime than the reference

Here, `x` has the lifetime `'b`, which in this case is larger than `'a`. This means `r` can reference `x` because Rust knows that the reference in `r` will always be valid while `x` is valid.

Now that you know where the lifetimes of references are and how Rust analyzes lifetimes to ensure references will always be valid, let’s explore generic lifetimes of parameters and return values in the context of functions.

**Generic Lifetimes in Functions**

Let’s write a function that returns the longer of two string slices. This function will take two string slices and return a string slice.

```rs
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

```

> Listing 10-21: An implementation of the `longest` function that returns the longer of two string slices but does not yet compile

Instead, we get the following error that talks about lifetimes:

```console
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:33
  |
1 | fn longest(x: &str, y: &str) -> &str {
  |                                 ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the
signature does not say whether it is borrowed from `x` or `y`
```

The help text reveals that the return type needs a generic lifetime parameter on it because Rust can’t tell whether the reference being returned refers to `x` or `y`. Actually, we don’t know either, because the `if` block in the body of this function returns a reference to `x` and the else block returns a reference to `y`!

When we’re defining this function, we don’t know the concrete values that will be passed into this function, so we don’t know whether the `if` case or the `else` case will execute. We also don’t know the concrete lifetimes of the references that will be passed in, so we can’t look at the scopes as we did in Listings 10-18 and 10-19 to determine whether the reference we return will always be valid. The borrow checker can’t determine this either, because it doesn’t know how the lifetimes of `x` and `y` relate to the lifetime of the return value. To fix this error, we’ll add generic lifetime parameters that define the relationship between the references so the borrow checker can perform its analysis.

**Lifetime Annotation Syntax**

**Lifetime annotations don’t change how long any of the references live.** Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter. **Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes**.

Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start with an apostrophe (`'`) and are usually all lowercase and very short, like generic types. Most people use the name `'a`. We place lifetime parameter annotations after the `&` of a reference, using a space to separate the annotation from the reference’s type.

Here are some examples: a reference to an `i32` without a lifetime parameter, a reference to an `i32` that has a lifetime parameter named `'a`, and a mutable reference to an `i32` that also has the lifetime `'a`.

```rs
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

One lifetime annotation by itself doesn’t have much meaning, because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other. For example, let’s say we have a function with the parameter first that is a reference to an `i32` with lifetime `'a`. The function also has another parameter named second that is another reference to an `i32` that also has the lifetime `'a`. The lifetime annotations indicate that the references `first` and `second` must both live as long as that generic lifetime.

**Lifetime Annotations in Function Signatures**

Now let’s examine lifetime annotations in the context of the `longest` function. As with generic type parameters, we need to declare generic lifetime parameters inside angle brackets between the function name and the parameter list. The constraint we want to express in this signature is that all the references in the parameters and the return value must have the same lifetime. We’ll name the lifetime `'a` and then add it to each reference, as shown in Listing 10-22.

```rs
#![allow(unused_variables)]
fn main() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
```

> Listing 10-22: The `longest` function definition specifying that all the references in the signature must have the same lifetime `'a`

The function signature now tells Rust that for some lifetime `'a`, the function takes two parameters, both of which are string slices that live at least as long as lifetime `'a`. The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime `'a`. In practice, it means that the lifetime of the reference returned by the `longest` function is the same as the smaller of the lifetimes of the references passed in. These constraints are what we want Rust to enforce. Remember, when we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned. Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints. Note that the `longest` function doesn’t need to know exactly how long x and y will live, only that some scope can be substituted for `'a` that will satisfy this signature.

When annotating lifetimes in functions, the annotations go in the function signature, not in the function body. Rust can analyze the code within the function without any help. However, when a function has references to or from code outside that function, it becomes almost impossible for Rust to figure out the lifetimes of the parameters or return values on its own. The lifetimes might be different each time the function is called. This is why we need to annotate the lifetimes manually.

When we pass concrete references to `longest`, the concrete lifetime that is substituted for `'a` is the part of the scope of `x` that overlaps with the scope of `y`. In other words, the generic lifetime `'a` will get the concrete lifetime that is equal to the smaller of the lifetimes of `x` and `y`. Because we’ve annotated the returned reference with the same lifetime parameter `'a`, the returned reference will also be valid for the length of the smaller of the lifetimes of `x` and `y`.

Let’s look at how the lifetime annotations restrict the `longest` function by passing in references that have different concrete lifetimes. Listing 10-23 is a straightforward example.

```rs
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```

> Listing 10-23: Using the `longest` function with references to String values that have different concrete lifetimes

In this example, `string1` is valid until the end of the outer scope, `string2` is valid until the end of the inner scope, and `result` references something that is valid until the end of the inner scope. Run this code, and you’ll see that the borrow checker approves of this code; it will compile and print `The longest string is long string is long`.

Next, let’s try an example that shows that the lifetime of the reference in `result` must be the smaller lifetime of the two arguments. We’ll move the declaration of the `result` variable outside the inner scope but leave the assignment of the value to the `result` variable inside the scope with `string2`. Then we’ll move the `println!` that uses `result` outside the inner scope, after the inner scope has ended. The code in Listing 10-24 will not compile.

```rs
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

> Listing 10-24: Attempting to use `result` after `string2` has gone out of scope

When we try to compile this code, we’ll get this error:

```console
error[E0597]: `string2` does not live long enough
  --> src/main.rs:15:5
   |
14 |         result = longest(string1.as_str(), string2.as_str());
   |                                            ------- borrow occurs here
15 |     }
   |     ^ `string2` dropped here while still borrowed
16 |     println!("The longest string is {}", result);
17 | }
   | - borrowed value needs to live until here
```

The error shows that for `result` to be valid for the `println!` statement, `string2` would need to be valid until the end of the outer scope. Rust knows this because we annotated the lifetimes of the function parameters and return values using the same lifetime parameter `'a`.

**Thinking in Terms of Lifetimes**

The way in which you need to specify lifetime parameters depends on what your function is doing. For example, if we changed the implementation of the `longest` function to always return the first parameter rather than the longest string slice, we wouldn’t need to specify a lifetime on the y parameter. The following code will compile:

```rs

#![allow(unused_variables)]
fn main() {
    fn longest<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }
}
```

In this example, we’ve specified a lifetime parameter `'a` for the parameter x and the return type, but not for the parameter `y`, because the lifetime of `y` does not have any relationship with the lifetime of `x` or the return value.

**When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.** If the reference returned does not refer to one of the parameters, it must refer to a value created within this function, which would be a dangling reference because the value will go out of scope at the end of the function. Consider this attempted implementation of the `longest` function that won’t compile:

```rs
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```

Here, even though we’ve specified a lifetime parameter `'a` for the return type, this implementation will fail to compile because the return value lifetime is not related to the lifetime of the parameters at all. Here is the error message we get:

```console
error[E0597]: `result` does not live long enough
 --> src/main.rs:3:5
  |
3 |     result.as_str()
  |     ^^^^^^ does not live long enough
4 | }
  | - borrowed value only lives until here
  |
note: borrowed value must be valid for the lifetime 'a as defined on the
function body at 1:1...
 --> src/main.rs:1:1
  |
1 | / fn longest<'a>(x: &str, y: &str) -> &'a str {
2 | |     let result = String::from("really long string");
3 | |     result.as_str()
4 | | }
  | |_^
```

The problem is that `result` goes out of scope and gets cleaned up at the end of the `longest` function. We’re also trying to return a reference to `result` from the function. There is no way we can specify lifetime parameters that would change the dangling reference, and Rust won’t let us create a dangling reference. In this case, the best fix would be to return an owned data type rather than a reference so the calling function is then responsible for cleaning up the value.

Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.

**Lifetime Annotations in Struct Definitions**

So far, we’ve only defined structs to hold owned types. It’s possible for structs to hold references, but in that case we would need to add a lifetime annotation on every reference in the struct’s definition.


```rs
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}
```

> Listing 10-25: A struct that holds a reference, so its definition needs a lifetime annotation

This struct has one field, `part`, that holds a string slice, which is a reference. As with generic data types, we declare the name of the generic lifetime parameter inside angle brackets after the name of the struct so we can use the lifetime parameter in the body of the struct definition. This annotation means an instance of `ImportantExcerpt` can’t outlive the reference it holds in its part field.

The `main` function here creates an instance of the `ImportantExcerpt` struct that holds a reference to the first sentence of the `String` owned by the variable `novel`. The data in `novel` exists before the `ImportantExcerpt` instance is created. In addition, `novel` doesn’t go out of scope until after the `ImportantExcerpt` goes out of scope, so the reference in the `ImportantExcerpt` instance is valid.

**Lifetime Elision**

You’ve learned that every reference has a lifetime and that you need to specify lifetime parameters for functions or structs that use references. However, in Chapter 4 we had a function in Listing 4-9, which is shown again in Listing 10-26, that compiled without lifetime annotations.

```rs
#![allow(unused_variables)]
fn main() {
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
}
```

Listing 10-26: A function we defined in Listing 4-9 that compiled without lifetime annotations, even though the parameter and return type are references

The reason this function compiles without lifetime annotations is historical: in early versions (pre-1.0) of Rust, this code wouldn’t have compiled because every reference needed an explicit lifetime. At that time, the function signature would have been written like this:

```rs
fn first_word<'a>(s: &'a str) -> &'a str {
```

After writing a lot of Rust code, the Rust team found that Rust programmers were entering the same lifetime annotations over and over in particular situations. These situations were predictable and followed a few deterministic patterns. The developers programmed these patterns into the compiler’s code so the borrow checker could infer the lifetimes in these situations and wouldn’t need explicit annotations.

The patterns programmed into Rust’s analysis of references are called the *lifetime elision rules*. 

The elision rules don’t provide full inference. If Rust deterministically applies the rules but there is still ambiguity as to what lifetimes the references have, the compiler won’t guess what the lifetime of the remaining references should be. In this case, instead of guessing, the compiler will give you an error that you can resolve by adding the lifetime annotations that specify how the references relate to each other.

Lifetimes on function or method parameters are called *input lifetimes*, and lifetimes on return values are called *output lifetimes*.

**The compiler uses three rules to figure out what lifetimes references have when there aren’t explicit annotations.** The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes. If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error. These rules apply to `fn` definitions as well as `impl` blocks.

1.  The first rule is that each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32)`; a function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`; and so on.
2. The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`.
3. The third rule is if there are multiple input lifetime parameters, but one of them is `&self` or `&mut` self because this is a method, the lifetime of `self` is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

**Lifetime Annotations in Method Definitions**

Lifetime names for struct fields always need to be declared after the `impl` keyword and then used after the struct’s name, because those lifetimes are part of the struct’s type.

In method signatures inside the `impl` block, references might be tied to the lifetime of references in the struct’s fields, or they might be independent. In addition, the lifetime elision rules often make it so that lifetime annotations aren’t necessary in method signatures. Let’s look at some examples using the struct named ImportantExcerpt that we defined in Listing 10-25.

First, we’ll use a method named `level` whose only parameter is a reference to `self` and whose return value is an `i32`, which is not a reference to anything:

```rs

#![allow(unused_variables)]
fn main() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }
}
```

The lifetime parameter declaration after `impl` and its use after the type name are required, but we’re not required to annotate the lifetime of the reference to `self` because of the first elision rule.

Here is an example where the third lifetime elision rule applies:

```rs
#![allow(unused_variables)]
fn main() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
}
```

There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both `&self` and `announcement` their own lifetimes. Then, because one of the parameters is `&self`, the return type gets the lifetime of `&self`, and all lifetimes have been accounted for.

**The Static Lifetime**

One special lifetime we need to discuss is `'static`, which means that this reference can live for the entire duration of the program. All string literals have the `'static` lifetime, which we can annotate as follows:

```rs
#![allow(unused_variables)]
fn main() {
    let s: &'static str = "I have a static lifetime.";
}
```

The text of this string is stored directly in the program’s binary, which is always available. Therefore, the lifetime of all string literals is `'static`.

You might see suggestions to use the `'static` lifetime in error messages. But before specifying `'static` as the lifetime for a reference, think about whether the reference you have actually lives the entire lifetime of your program or not. You might consider whether you want it to live that long, even if it could. Most of the time, the problem results from attempting to create a dangling reference or a mismatch of the available lifetimes. In such cases, the solution is fixing those problems, not specifying the `'static` lifetime.

**Generic Type Parameters, Trait Bounds, and Lifetimes Together**

Let’s briefly look at the syntax of specifying generic type parameters, trait bounds, and lifetimes all in one function!

```rs

#![allow(unused_variables)]
fn main() {
    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        where T: Display
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
```

This is the `longest` function from Listing 10-22 that returns the longer of two string slices. But now it has an extra parameter named `ann` of the generic type `T`, which can be filled in by any type that implements the `Display` trait as specified by the where clause. This extra parameter will be printed before the function compares the lengths of the string slices, which is why the `Display` trait bound is necessary. Because lifetimes are a type of generic, the declarations of the lifetime parameter `'a` and the generic type parameter `T` go in the same list inside the angle brackets after the function name.

**Summary**

Generic type parameters let you apply the code to different types. Traits and trait bounds ensure that even though the types are generic, they’ll have the behavior the code needs. You learned how to use lifetime annotations to ensure that this flexible code won’t have any dangling references. And all of this analysis happens at compile time, which doesn’t affect runtime performance!

## 11. Writing Automated Tests

### 11.1. How to Write Tests



```rs
fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

the `#[test]` annotation before the `fn` line: this attribute indicates this is a test function, so the test runner knows to treat this function as a test. We could also have non-test functions in the tests module to help set up common scenarios or perform common operations, so we need to indicate which functions are tests by using the `#[test]` attribute.

**Checking Results with the `assert!` Macro**

```rs
fn main() {}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }
}

```

Because the `tests` module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module. We use a glob here so anything we define in the outer module is available to this `tests` module.


Testing Equality with the `assert_eq!` and `assert_ne!` Macros

Adding Custom Failure Messages

```rs
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`", result
    );
}
```

When we run the test, we’ll get a more informative error message:

```console
---- tests::greeting_contains_name stdout ----
thread 'tests::greeting_contains_name' panicked at 'Greeting did not
contain name, value was `Hello!`', src/lib.rs:12:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

Checking for Panics with `should_panic`

Adding attribute `should_panic`, to our test function makes a test pass if the code inside the function panics; the test will fail if the code inside the function doesn’t panic.

```rs
fn main() {}
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

> Listing 11-8: Testing that a condition will cause a `panic!`

Tests that use `should_panic` can be imprecise because they only indicate that the code has caused some panic. A `should_panic` test would pass even if the test panics for a different reason from the one we were expecting to happen. To make `should_panic` tests more precise, we can add an optional `expected` parameter to the `should_panic` attribute. The test harness will make sure that the failure message contains the provided text. 

```rs
fn main() {}
pub struct Guess {
    value: i32,
}

// --snip--

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

> Listing 11-9: Testing that a condition will cause a `panic!` with a particular panic message

**Using Result<T, E> in Tests**

So far, we’ve written tests that panic when they fail. We can also write tests that use `Result<T, E>!`

```rs
#![allow(unused_variables)]
fn main() {
    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() -> Result<(), String> {
            if 2 + 2 == 4 {
                Ok(())
            } else {
                Err(String::from("two plus two does not equal four"))
            }
        }
    }
}
```

The `it_works` function now has a return type, `Result<(), String>`. In the body of the function, rather than calling the `assert_eq!` macro, we return `Ok(())` when the test passes and an Err with a `String` inside when the test fails.

Writing tests so they return a `Result<T, E>` enables you to use the question mark operator in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an `Err` variant.

You can’t use the `#[should_panic]` annotation on tests that use `Result<T, E>`. Instead, you should return an `Err` value directly when the test should fail.

### 11.2. Controlling How Tests Are Run

cargo test compiles your code in test mode and runs the resulting test binary.

**Running Tests in Parallel or Consecutively**

When you run multiple tests, by default they run in parallel using threads. If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads used, you can send the --test-threads flag and the number of threads you want to use to the test binary. Take a look at the following example:

```sh
$ cargo test -- --test-threads=1
```

We set the number of test threads to 1, telling the program not to use any parallelism.

**Showing Function Output**

By default, if a test passes, Rust’s test library captures anything printed to standard output. For example, if we call println! in a test and the test passes, we won’t see the println! output in the terminal; we’ll see only the line that indicates the test passed. If a test fails, we’ll see whatever was printed to standard output with the rest of the failure message.

If we want to see printed values for passing tests as well, we can tell Rust to also show the output of successful tests at the end with `--show-output`.

**Running a Subset of Tests by Name**

Sometimes, running a full test suite can take a long time. If you’re working on code in a particular area, you might want to run only the tests pertaining to that code. You can choose which tests to run by passing `cargo test` the name or names of the test(s) you want to run as an argument.

```rs

#![allow(unused_variables)]
fn main() {
    pub fn add_two(a: i32) -> i32 {
        a + 2
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn add_two_and_two() {
            assert_eq!(4, add_two(2));
        }

        #[test]
        fn add_three_and_two() {
            assert_eq!(5, add_two(3));
        }

        #[test]
        fn one_hundred() {
            assert_eq!(102, add_two(100));
        }
    }
}
```

> Listing 11-11: Three tests with three different names

**Running Single Tests**

We can pass the name of any test function to `cargo test` to run only that test:

```sh
$ cargo test one_hundred
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-06a75b4a1f2515e9

running 1 test
test tests::one_hundred ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out
```

**Filtering to Run Multiple Tests**

We can specify part of a test name, and any test whose name matches that value will be run.

```sh
$ cargo test add
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/deps/adder-06a75b4a1f2515e9

running 2 tests
test tests::add_two_and_two ... ok
test tests::add_three_and_two ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out
```

**Ignoring Some Tests Unless Specifically Requested**

Sometimes a few specific tests can be very time-consuming to execute, so you might want to exclude them during most runs of `cargo test`. Rather than listing as arguments all tests you do want to run, you can instead annotate the time-consuming tests using the ignore attribute to exclude them, as shown here:

```rs

#![allow(unused_variables)]
fn main() {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
```

After `#[test]` we add the `#[ignore]` line to the test we want to exclude. Now when we run our tests, `it_works` runs, but `expensive_test` doesn’t:

The `expensive_test` function is listed as ignored. If we want to run only the ignored tests, we can use `cargo test -- --ignored`

### 11.3. Test Organization

 The Rust community thinks about tests in terms of two main categories: unit tests and integration tests. Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces. Integration tests are entirely external to your library and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test.

**Unit Tests**

You’ll put unit tests in the src directory in each file with the code that they’re testing. The convention is to create a module named `tests` in each file to contain the test functions and to annotate the module with `cfg(test)`.

**The Tests Module and `#[cfg(test)]`**

The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build. You’ll see that because integration tests go in a different directory, they don’t need the `#[cfg(test)]` annotation. However, because unit tests go in the same files as the code, you’ll use `#[cfg(test)]` to specify that they shouldn’t be included in the compiled result.

**Testing Private Functions**

Rust’s privacy rules do allow you to test private functions.

```rs
fn main() {}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

> Listing 11-12: Testing a private function

Note that the `internal_adder` function is not marked as `pub`, but because tests are just Rust code and the `tests` module is just another module, you can bring `internal_adder` into a test’s scope and call it.

**Integration Tests**

In Rust, integration tests are entirely external to your library. They use your library in the same way any other code would, which means they can only call functions that are part of your library’s public API. Their purpose is to test whether many parts of your library work together correctly. 

**The *tests* Directory**

We create a tests directory at the top level of our project directory, next to *src*. Cargo knows to look for integration test files in this directory. 

Filename: tests/integration_test.rs

```rs
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

> Listing 11-13: An integration test of a function in the `adder` crate

We’ve added `use adder` at the top of the code, which we didn’t need in the unit tests. The reason is that each file in the `tests` directory is a separate crate, so we need to bring our library into each test crate’s scope.

We don’t need to annotate any code in *tests/integration_test.rs* with `#[cfg(test)]`. Cargo treats the `tests` directory specially and compiles files in this directory only when we run `cargo test`.

```sh
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running target/debug/deps/adder-abcabcabc

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-ce99bcc2479f4607

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The three sections of output include the unit tests, the integration test, and the doc tests. The first section for the unit tests

The integration tests section starts with the line Running target/debug/deps/integration_test-ce99bcc2479f4607 (the hash at the end of your output will be different). 

Doc-tests

**Submodules in Integration Tests**

As you add more integration tests, you might want to make more than one file in the tests directory to help organize them; for example, you can group the test functions by the functionality they’re testing.

Treating each integration test file as its own crate is useful to create separate scopes that are more like the way end users will be using your crate. However, this means files in the *tests* directory don’t share the same behavior as files in *src* do

The different behavior of files in the tests directory is most noticeable when you have a set of helper functions that would be useful in multiple integration test files and you try to follow the steps in the “Separating Modules into Different Files” section of Chapter 7 to extract them into a common module. For example, if we create *tests/common.rs* and place a function named `setup` in it, we can add some code to `setup` that we want to call from multiple test functions in multiple test files:

Filename: tests/common.rs

```rs
#![allow(unused_variables)]
pub fn setup() {
    // setup code specific to your library's tests would go here
}
```

When we run the tests again, we’ll see a new section in the test output for the *common.rs* file, even though this file doesn’t contain any test functions nor did we call the `setup` function from anywhere:

```sh
running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/common-b8b07b6f1be2db70

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-d993c68b431d39df

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Having `common` appear in the test results with `running 0 tests` displayed for it is not what we wanted. We just wanted to share some code with the other integration test files.

To avoid having `common` appear in the test output, instead of creating *tests/common.rs*, we’ll create tests/common/mod.rs. This is an alternate naming convention that Rust also understands. Naming the file this way tells Rust not to treat the `common` module as an integration test file. When we move the setup function code into *tests/common/mod.rs* and delete the *tests/common.rs* file, the section in the test output will no longer appear. Files in subdirectories of the tests directory don’t get compiled as separate crates or have sections in the test output.

**Integration Tests for Binary Crates**

If our project is a binary crate that only contains a *src/main.rs* file and doesn’t have a *src/lib.rs* file, we can’t create integration tests in the tests directory and bring functions defined in the *src/main.rs* file into scope with a `use` statement. Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.

This is one of the reasons Rust projects that provide a binary have a straightforward* src/main.rs* file that calls logic that lives in the src/lib.rs file. Using that structure, integration tests can test the library crate with `use` to make the important functionality available. If the important functionality works, the small amount of code in the *src/main.rs* file will work as well, and that small amount of code doesn’t need to be tested.

## 12. An I/O Project: Building a Command Line Program

### 12.1. Accepting Command Line Arguments

```rs
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```

> Listing 12-1: Collecting the command line arguments into a vector and printing them

**Saving the Argument Values in Variables**

```rs
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
```

> Listing 12-2: Creating variables to hold the query argument and filename argument

**Reading a File**

```rs
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    // --snip--
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```

> Listing 12-4: Reading the contents of the file specified by the second argument

### 12.3. Refactoring to Improve Modularity and Error Handling

**Separation of Concerns for Binary Projects**

 the Rust community has developed a process to use as a guideline for splitting the separate concerns of a binary program when main starts getting large. The process has the following steps:

* Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
* As long as your command line parsing logic is small, it can remain in main.rs.
* When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

The responsibilities that remain in the `main` function after this process should be limited to the following:

* Calling the command line parsing logic with the argument values
* Setting up any other configuration
* Calling a run function in lib.rs
* Handling the error if run returns an error

This pattern is about separating concerns: *main.rs* handles running the program, and *lib.rs* handles all the logic of the task at hand.

**Extracting the Argument Parser**

We’ll extract the functionality for parsing arguments into a function that `main` will call to prepare for moving the command line parsing logic to *src/lib.rs.*. Listing 12-5 shows the new start of main that calls a new function `parse_config`, which we’ll define in *src/main.rs* for the moment.

```rs
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    // --snip--
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
```

> Listing 12-5: Extracting a `parse_config` function from `main`

**Grouping Configuration Values**

```rs
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    // --snip--
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone(); // make a full copy of the data for the Config instance to own
    let filename = args[2].clone();

    Config { query, filename }
}
```

> Listing 12-6: Refactoring `parse_config` to return an instance of a `Config` struct

**Creating a Constructor for Config**

```rs
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    // --snip--
}

struct Config {
    query: String,
    filename: String,
}

// --snip--

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
```

**Fixing the Error Handling**

**Improving the Error Message**


```rs
// --snip--
fn new(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    // --snip--
```

> Listing 12-8: Adding a check for the number of arguments

**Returning a `Result` from new Instead of Calling `panic!`**

```rs
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
```

> Listing 12-9: Returning a `Result` from `Config::new`

**Calling `Config::new` and Handling Errors**

```rs
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
```

> Listing 12-10: Exiting with an error code if creating a new `Config` fails

**Extracting Logic from `main`**

Now that we’ve finished refactoring the configuration parsing, let’s turn to the program’s logic.

Listing 12-11 shows the extracted `run` function. For now, we’re just making the small, incremental improvement of extracting the function. We’re still defining the function in *src/main.rs*.

```rs
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// --snip--
```

> Listing 12-11: Extracting a `run` function containing the rest of the program logic

**Returning Errors from the `run` Function**

With the remaining program logic separated into the `run` function, we can improve the error handling, as we did with `Config::new` in Listing 12-9.

```rs
use std::error::Error;

// --snip--

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
```

> Listing 12-12: Changing the `run` function to return `Result`

We’ve made three significant changes here. First, we changed the return type of the `run` function to `Result<(), Box<dyn Error>>`. This function previously returned the unit type, `()`, and we keep that as the value returned in the `Ok` case. For the error type, we used the *trait* object `Box<dyn Error>`. We’ll cover *trait* objects in Chapter 17. For now, just know that `Box<dyn Error>` means the function will return a type that implements the `Error` trait, but we don’t have to specify what particular type the return value will be. This gives us flexibility to return error values that may be of different types in different error cases. The `dyn` keyword is short for “dynamic.”

Second, we’ve removed the call to expect in favor of the `?` operator. Rather than `panic!` on an error, `?` will return the error value from the current function for the caller to handle.

Third, the `run` function now returns an `Ok` value in the success case. We’ve declared the `run` function’s success type as `()` in the signature, which means we need to wrap the unit type value in the `Ok` value. This `Ok(())` syntax might look a bit strange at first, but using `()` like this is the idiomatic way to indicate that we’re calling run for its side effects only; it doesn’t return a value we need.

When you run this code, it will compile but will display a warning:

```rs
warning: unused `std::result::Result` that must be used
  --> src/main.rs:17:5
   |
17 |     run(config);
   |     ^^^^^^^^^^^^
   |
   = note: #[warn(unused_must_use)] on by default
   = note: this `Result` may be an `Err` variant, which should be handled
```

Rust tells us that our code ignored the `Result` value and the `Result` value might indicate that an error occurred. But we’re not checking to see whether or not there was an error, and the compiler reminds us that we probably meant to have some error-handling code here! Let’s rectify that problem now.

**Handling Errors Returned from `run` in `main`**

```rs
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
```

We use `if let` rather than `unwrap_or_else` to check whether run returns an Err value and call `process::exit(1)` if it does. The run function doesn’t return a value that we want to unwrap in the same way that `Config::new` returns the Config instance. Because run returns `()` in the success case, we only care about detecting an error, so we don’t need `unwrap_or_else` to return the unwrapped value because it would only be `()`.

The bodies of the `if let` and the `unwrap_or_else` functions are the same in both cases: we print the error and exit.

**Splitting Code into a Library Crate**

Our minigrep project is looking good so far! Now we’ll split the *src/main.rs* file and put some code into the *src/lib.rs* file so we can test it and have a *src/main.rs* file with fewer responsibilities.

Let’s move all the code that isn’t the main function from *src/main.rs* to *src/lib.rs*:

* The `run` function definition
* The relevant `use` statements
* The definition of `Config`
* The `Config::new` function definition

### 12.4. Developing the Library’s Functionality with Test Driven Development

**Writing a Failing Test**

```rs

#![allow(unused_variables)]
fn main() {
    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        vec![]
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn one_result() {
            let query = "duct";
            let contents = "\
    Rust:
    safe, fast, productive.
    Pick three.";

            assert_eq!(
                vec!["safe, fast, productive."],
                search(query, contents)
            );
        }
    }
}
```

> Listing 12-15: Creating a failing test for the `search` function we wish we had

Because `contents` is the argument that contains all of our text and we want to return the parts of that text that match, we know `contents` is the argument that should be connected to the return value using the lifetime syntax.

**Writing Code to Pass the Test**

Currently, our test is failing because we always return an empty vector. To fix that and implement search, our program needs to follow these steps:

* Iterate through each line of the contents.
* Check whether the line contains our query string.
* If it does, add it to the list of values we’re returning.
* If it doesn’t, do nothing.
* Return the list of results that match.

```rs
#![allow(unused_variables)]

use std::env;
use std::process;
use list12_15::Config;
use list12_15::run;

// cargo run --color=always --package list12-15 --bin list12-15 frog .\examples\list12-15\poem.txt
fn main() {
    // collect turn the iterator to vector
    let args: Vec<String> = env::args().collect(); //Collecting the command line arguments into a vector
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // run(config); // an Err may return here, need to handle it
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
```


```rs
#![allow(unused_variables)]

use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // return Result instead of panic
        // constructor
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &content) {
        println!("{}", line);
    }

    Ok(())
}

// Because `contents` is the argument that contains all of our text and we want to return the parts
// of that text that match, we know `contents` is the argument that should be connected to the
// return value using the lifetime syntax.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
```

### 12.5. Working with Environment Variables

**Writing a Failing Test for the Case-Insensitive `search` Function**

```rs
#![allow(unused_variables)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
```

> Listing 12-20: Adding a new failing test for the case-insensitive function we’re about to add

**Implementing the `search_case_insensitive` Function**


```rs
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}
```

> Listing 12-21: Defining the `search_case_insensitive` function to lowercase the query and the line before comparing them


```rs

#![allow(unused_variables)]
fn main() {
use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
     vec![]
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
     vec![]
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
}
```

> Listing 12-22: Calling either `search` or `search_case_insensitive` based on the value in `config.case_sensitive`

The functions for working with environment variables are in the `env` module in the standard library, we’ll use the `var` function from the `env` module to check for an environment variable named `CASE_INSENSITIVE`.

```rs

#![allow(unused_variables)]
fn main() {
use std::env;
struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

// --snip--

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
}
```

> Listing 12-23: Checking for an environment variable named `CASE_INSENSITIVE`


The `env::var` function returns a `Result` that will be the successful `Ok` variant that contains the value of the environment variable if the environment variable is set. It will return the `Err` variant if the environment variable is not set.

We’re using the `is_err` method on the `Result` to check whether it’s an error and therefore unset, which means it should do a case-sensitive search. If the `CASE_INSENSITIVE` environment variable is set to anything, `is_err` will return false and the program will perform a case-insensitive search. We don’t care about the value of the environment variable, just whether it’s set or unset, so we’re checking `is_err` rather than using `unwrap`, `expect`, or any of the other methods we’ve seen on `Result`.

```console
$ CASE_INSENSITIVE=1 cargo run to poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

Excellent, we also got lines containing “To”! 

### 12.6. Writing Error Messages to Standard Error Instead of Standard Output

































## Q & A


1. How to organize code:  
When the code base is getting larger, you might need to work with multiple crates on the same project. Rust supports this via Workspaces, run example by: `(cd examples/hello && cargo run)`


Refs:
https://doc.rust-lang.org/book/title-page.html