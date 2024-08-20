# C_rust_calculator
Assignment given by Electrolux. Also my first "project" in rust.


What was the assignment?

1. Implement a simple C library that provides basic arithmetic operations.

The library should have the following functions:

int add(int a, int b); 

int subtract(int a, int b); 

int multiply(int a, int b); 

double divide(int a, int b);

 

2. Create a Rust application that links to the C library and uses its functions.

 

Requirements:  

 

Use bindgen to generate the Rust bindings for the C library. 

Generation should be a part of the build process

 

The Rust application should:

Provide a safe Rust interface to the C functions.

Handle any potential errors gracefully.

Rust app functionality is up to you.  

 

3. Build process

Include a build.rs file to automate the process of compiling the C code and linking it with the Rust code.

Use bindgen to generate the Rust bindings for the C library. 

Generation should be a part of the build process

 

4. Tests are optional. 

Unit tests are optional but beneficial. (But I am adding them.)


===============================================================================================================================================

Usage:

cargo build => builds the application

cargo test => tests the application (so far just the add,subtract,multiply and divide functionalities)

cargo run -- <functionality> <num1> <num2> => gives the output
