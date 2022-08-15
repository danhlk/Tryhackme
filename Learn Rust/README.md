> # Learn Rust - Tryhackme

# Summary
* [Task 1 - What is Rust?](#task-1---what-is-rust)
* [Task 2 - Installing &amp; Tooling](#task-2---installing--tooling)
* [Task 3 - Hello, World!](#task-3---hello-world)
* [Task 4 - Variables](#task-4---variables)
* [Task 5 - Constant Variables](#task-5---constant-variables)
* [Task 6 - Data Structures](#task-6---data-structures)
* [Task 7 - Functions](#task-7---functions)
* [Task 8 - Loops](#task-8---loops)
* [Task 9 - Zero Cost Abstractions](#task-9---zero-cost-abstractions)
* [Task 10 - Rayon](#task-10---rayon)
* [Task 11 - If Statements](#task-11---if-statements)
* [Task 12 - Error Handling](#task-12---error-handling)
* [Task 13 - Challenge](#task-13---challenge)

## Task 1 - What is Rust?

```python
squares = (val * val for val in range(100))
print(min(squares))                
print(max(squares))
```
Kết quả
```python
>>> print(min(squares))
0
>>> print(max(squares))
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
ValueError: max() arg is an empty sequence
```
Hàm `min` thay đổi biến squares dù ta chỉ muốn lấy giá trị nhỏ nhất trong đó 

Code tương tự trong Rust
```rust
fn main() {
   let squares = (0..100).map(|val| val * val);
   println!("{:?}", squares.min());  
   println!("{:?}", squares.max());     
}    
```

Khi biên dịch thông báo lỗi rõ ràng hơn.
```rust
error[E0382]: use of moved value: `squares`
 --> ownership.rs:4:21
  |
3 |    println!("{:?}", squares.min());
  |                     ------- value moved here
4 |    println!("{:?}", squares.max());
  |                     ^^^^^^^ value used here after move
  |
  = note: move occurs because `squares` has type `std::iter::Map<std::ops::Range<i32>, [closure@ownership.rs:2:31: 2:40]>`, which does not implement the `Copy` trait
```

> ***Python allows functions to alter variables they do not own, whereas Rust doesn't***


1. What other language is Rust similar to in terms of performance?<br>
    > Rust aims to be similar in terms of performance to C++.

    **Answer:** C++

1. What famous company switched from Go to Rust, mentioned in this task?<br>
    > Go, a high level programming language similar syntactically to Python but is fast & compiled, uses garbage collection. This caused a massive overhead at Discord, which forced them to switch from Go to Rust.
    
    **Answer:** Discord

1. Microsoft Security Centre reports what percentage of CVE's they assign are memory safety issues? Include the % sign.<br>
    > The Microsoft Security Response Centre states that 70% of all CVE's MSRC assigns are memory safety issues. 

    **Answer:** 70%

1. What is Rust's version of NPM or PyPi?<br>
    **Answer:** Cargo

## Task 2 - Installing & Tooling
1. What is the tool we used to install Rust called?<br>
    > Rust recommends using the tool rustup to manage multiple versions of Rust.

    **Answer:** rustup

1. How do we install the package rustscan using cargo?<br>
    **Answer:** cargon install rustscan

1. What command do we run to format our code?<br>
    **Answer:** cargo fmt

## Task 3 - Hello, World!
1. How do we initialise a new Rust project?<br>
    **Answer:** cargo init

1. What character represents a macro?<br>
    **Answer:** !

1. What does every Rust project need as a file?<br>
    > Our main.rsfile in the folder src is the main file where we write our code. 

    **Answer:** main.rs

1. If we wanted to add a dependency to our Rust project, what file would we edit?<br>
    > cargo.toml is the configuration file for our Rust project. It includes our dependencies, project name, authors, the version of Rust we are using and more.

    **Answer:** Cargo.toml

1. How do we run our Rust project?<br>
    **Answer:** cargo run

1. How do we build the project RustScan with the release profile (most optimised)?<br>
    **Answer:** cargo build --release

1. What folder are the release binaries stored in?<br>
    **Answer:** ./target/release/

1. How many release profiles does Rust have using optimisation level?<br>
    From this link [https://doc.rust-lang.org/book/ch14-01-release-profiles.html#:~:text=The%20Rust%20Programming%20Language,-Customizing%20Builds%20with&text=In%20Rust%2C%20release%20profiles%20are,configured%20independently%20of%20the%20others.](https://doc.rust-lang.org/book/ch14-01-release-profiles.html#:~:text=The%20Rust%20Programming%20Language,-Customizing%20Builds%20with&text=In%20Rust%2C%20release%20profiles%20are,configured%20independently%20of%20the%20others.).<br>
    > The opt-level setting controls the number of optimizations Rust will apply to your code, with a range of 0 to 3.

    **Answer:** 4

## Task 4 - Variables
Mặc định giá trị biến của Rust là không thể thay đổi, muốn thay đổi phải thêm từ khóa `mut` trước tên biến. <br>

Question 1
```rust
fn main() {
    let x: u32 = 5;
    println!("The value of x is: {}", x);
    x = "hello";
    println!("The value of x is: {}", x);
}
```

Question 2
```rust
fn main() {
    let x: u32 = 5;
    println!("The value of x is: {}", x);
    x = 5;
    println!("The value of x is: {}", x);
}
```

1. In question 1, does this code compile? T(rue) or F(alse)<br>
    It will cause an error because of assignment &str to u32.<br>
    **Answer:** F

1. What is the error code returned by question 1?<br>
    Compile to find error code.<br>
    ```rust
    $ cargo run
   Compiling Rust v0.1.0 (/Rust)
    error[E0308]: mismatched types
    --> src/main.rs:42:9
    |
    40 |     let x: u32 = 5;
    |            --- expected due to this type
    41 |     println!("The value of x is: {}", x);
    42 |     x = "hello";
    |         ^^^^^^^ expected `u32`, found `&str`

    For more information about this error, try `rustc --explain E0308`.
    error: could not compile `Rust` due to previous error
    ```
    **Answer:** E0308

1. Does the code in question 2 compile? T(rue) or F(alse)<br>
    It also cause an error because we assin twice to immutable variable.<br>
    **Answer:** F

1. What is the error message returned?<br>
    Compile code in question 2.<br>
    ```rust
    $ cargo run
   Compiling Rust v0.1.0 (/Rust)
    error[E0384]: cannot assign twice to immutable variable `x`
    --> src/main.rs:42:5
    |
    40 |     let x: u32 = 5;
    |         -
    |         |
    |         first assignment to `x`
    |         help: consider making this binding mutable: `mut x`
    41 |     println!("The value of x is: {}", x);
    42 |     x = 5;
    |     ^^^^^ cannot assign twice to immutable variable

    For more information about this error, try `rustc --explain E0384`.
    error: could not compile `Rust` due to previous error
    ```
    **Answer:** cannot assign twice to immutable variable

## Task 5 - Constant Variables
Có thể dùng dấu gạch dưới `_` để đại diện cho dấu cách trong số mà không ảnh hưởng đến giá trị của nó. <br>

1. How do we define a constant in Rust?<br>
    **Answer:** const

1. Can we shadow a constant? T(rue) or F(alse)<br>
    > These are values that aren't just immutable by default, but are always immutable.

    **Answer:** F

1. What do we use to change the type of an immutable variable once it has been defined?<br>
    > The first variable is shadowed by the second

    **Answer:** shadowed

1. Will the code "CONST word = "yes"" compile? T(rue) or F(alse)<br>
    **Answer:** F

1. We have "let word = "hello"", how do we get the length of the variable?
    **Answer:** word.len();

## Task 6 - Data Structures
1. Given the number -6, is this signed or unsigned?<br>
    **Answer:** signed

1. Given the number 65536, what is the smallest unsigned datatype we can fit this into?<br>
    **Answer:** u32

1. What's the smallest sized signed integer in rust?<br>
    **Answer:** i16

1. Create a mutable u32 variable called "tryhackme" and assign it the number 9<br>
    **Answer:** let mut tryhackme: u32 = 9

1. What data type is used to represent a string slice?<br>
    **Answer:** &str

1. Let's say you had a variable, X. You wanted to typehint the variable as a string. What would you write? Include X in the variable but not the let or = parts.<br>
    **Answer:** X: String

## Task 7 - Functions
Question 1
```rust
fn hello(){
    8172192: u16;
}
```

Question 2
```rust
fn return(){
    6;
}
```

Question 3
```rust
fn test(name) {
    println!("{}", name);
}
test("bee");
```

1. Will question 1 return 8172192? T(rue) or F(alse)<br>
    Not a return statelment;
    **Answer:** F

1. Will example 2 run? T(rue) or F(alse)<br>
    No need semicolon.<br>
    **Answer:** F

1. What type should we give to the argument for question 3?<br>
    **Answer:** &str

1. The last expression in a function (the return) needs to have a semicolon. T(rue) or F(alse)<br>
    **Answer:** F

1. Every function need to return something. T(rue) or F(alse)<br>
    **Answer:** F

1. Functions in Rust can be nested within other functions. T(rue) or F(alse)<br>
    **Answer:** T

1. What keyword do we use to return early from a function?<br>
    **Answer:** return

1. You nest a function named main, inside another function named main. Will this run? T(rue) or F(alse)<br>
    **Answer:** F


## Task 8 - Loops
1. How do we break out of a loop?<br>
    **Answer:** break

1. Simplest keyword to make an infinite loop?<br>
    **Answer:** loop

1. Turn let a = [10, 20]; into something we can iterate over.<br>
    **Answer:** a.iter();

1. While loops can also be infinite. T(rue) or F(alse).<br>
    **Answer:** T

## Task 9 - Zero Cost Abstractions
1. Iterators are lazy. T(rue) or F(alse).<br>
    > This is because iterators are lazy. You have to tell them to do something to get values from them.

    **Answer:** T

1. For loops are explicitly mentioned in the Rust book as zero cost abstractions. T(rue) or F(alse).<br>
    > Let’s say to use a for loop, the language needs to have some massive 1gb file that slows down everything else.

    **Answer:** F

1. Zero Cost Abstractions are common in high level languages like Python or JavaScript T(rue) or F(alse).<br>
    **Answer:** F

## Task 10 - Rayon
1. What crate do we use to easily make an iter multi threaded?<br>
    **Answer:** rayon

1. How do we tell Rust to include an external crate into our program? What file do we place this information in?<br>
    **Answer:** cargo.toml

1. Turn a.iter() into a multi threaded parallel iter using Rayon<br>
    **Answer:** a.par_iter()

1. What website do we go to for Crates?<br>
    **Answer:** crates.io

## Task 11 - If Statements
1. We can assign variables based on an if statement on one line T(rue) or F(alse)<br>
    > Rust lets us do cool things with if statements. Such as assigning to a variable based on an if statement.

    **Answer:** T

## Task 12 - Error Handling
1. What is the data type returned from opening a file?<br>
    **Answer:** Result

1. Write the datatype of a generic Result with type hints<br>
    **Answer:** Result <T, E>

1. We're in a function and we get given a Result enum. If the Result is okay we want to continue working on it in this function. If the result is Err we want to return to the parent function with Err. What should we use?<br>
    **Answer:** ?

1. We're certain our result will always return Ok, what should we use?<br>
    **Answer:** unwrap

## Task 13 - Challenge
From flow of encryption, you can use any online tools to decrypt the ciphertext. But we're in the Rust, so I will write a program using Rust language to decrypt it.<br>

Write a function to decrypt rot13.<br>
```rust
pub fn rot13 (cipher: String) -> String {
    let mut result = vec![];

    for c in cipher.chars() {
        let c_code = c as u8;

        if c.is_uppercase() {
            result.push(((c_code + 13 - 65) % 26 + 65) as char);
        }
        else if c.is_lowercase() {
            result.push(((c_code + 13 - 97) % 26 + 97) as char);
        }
        else if c == '\n' {
            continue;
        }
        else {
            result.push(c);
        }
    }
    return result.iter().collect::<String>();
}
```

Function decode base64 string
```rust
pub fn decode_base64 (cipher: String) -> String {
    let bytes = base64::decode(cipher.clone()).unwrap();
    let result = bytes.iter().map(|&e| e as char).collect::<Vec<char>>();
    return result.iter().collect::<String>();
}
```

You can see all source code in [Challenge](Challenge/) folder.<br>
Execute `cargo run` to retrive the plaintext.<br>
```
$ cargo run
   Compiling Rust v0.1.0 (/Tryhackme/Learn Rust/Challenge)
    Finished dev [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/Rust`
"THM{RUST}"
```

**Answer:** THM{RUST}
