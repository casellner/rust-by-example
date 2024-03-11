/*
    Filename: hello.rs
    Author:   Rust By Example, modified by Colton Sellner
    Date:     10 March 2024
    Purpose:  Print to the screen, comment, and do math in Rust.
*/

fn main() {
    //an exclamation mark is used since println is a macro
    println!("Hello World!");

    let x = 5 + 5;
    println!("x is {}", x); //prints "x is 10"

    //integers inside '{}' are positional arguments
    println!("{0} and {1} ran up a hill.", "Jack", "Jill");
    //one can also use named arguments
    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");

    //a format character can be typed after a ':'
    println!("Base 10:               {}",   1234);
    println!("Base 2 (binary):       {:b}", 1234);
    println!("Base 8 (octal):        {:o}", 1234);
    println!("Base 16 (hexadecimal): {:x}", 1234);
    println!("Base 16 (hexadecimal): {:X}", 1234);

    //right justify with width of 5
    println!("{number:>5}",number=1);
    //right justify with width of 5 and pad with 0's
    println!("{number:0>5}",number=1);
    //left justify with width of 5 and pad with 0's
    println!("{number:0<5}",number=1);
    //format with an argument by appending a $
    println!("{number:0>width$}", number=1, width=10);

    // Rust checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    //#[allow(dead_code)] //disable 'dead_code' which warns against unused modules
    #[derive(Debug)] //easiest way to print struct (use {:#?} in println)
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    println!("This struct `{:#?}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi); //print with 3 digits after the decimal point
}