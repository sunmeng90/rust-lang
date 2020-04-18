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

```rs

```

## Q & A


1. How to organize code:  
When the code base is getting larger, you might need to work with multiple crates on the same project. Rust supports this via Workspaces, run example by: `(cd examples/hello && cargo run)`


Refs:
https://doc.rust-lang.org/book/title-page.html