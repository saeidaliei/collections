// ============================================================================
// RUST BY EXAMPLE — PART 1
// ============================================================================
#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    basics();
    control_flow();
    ownership_and_borrowing();
    structs_and_methods();
    enums_and_pattern_matching();
    collections();
    error_handling();
    generics_and_traits();
    closures_and_iterators();
    smart_pointers();
    concurrency();
    modules_demo::run();
}

// ============================================================================
// 1. BASICS: variables, mutability, shadowing, types, tuples, arrays
// ============================================================================
fn basics() {
    println!("\n--- 1. BASICS ---");

    let x = 5;              // Immutable by default. This is the Rust philosophy:
                             // safe-by-default, opt-in to mutability.
    // x = 6;                // ERROR: cannot assign twice to immutable variable

    let mut y = 5;           // `mut` opts into mutability
    y += 1;

    let x = x + 1;            // "Shadowing": rebinding the same name, allowed,
    let x = x * 2;            // creates a NEW variable each time (can change type too)

    let a: i32 = -5;          // Explicit type annotation. Integers default to i32.
    let b: u64 = 100;         // u = unsigned, i = signed, sizes: 8/16/32/64/128/size
    let f: f64 = 3.14;        // Floats default to f64
    let is_ready: bool = true;
    let letter: char = 'z';   // char is 4 bytes, a unicode scalar value

    // Tuples: fixed-size, can mix types
    let tup: (i32, f64, char) = (500, 6.4, 'x');
    let (t1, t2, t3) = tup;    // destructuring
    println!("tuple.0 = {}", tup.0); // access by index

    // Arrays: fixed-size, SAME type, size known at compile time, stack-allocated
    let arr: [i32; 3] = [1, 2, 3];
    let zeros = [0; 5];        // [value; length] -> [0,0,0,0,0]
    println!("arr[0] = {}, zeros.len() = {}", arr[0], zeros.len());

    // Strings: &str (borrowed string slice, usually literal) vs String (owned, heap)
    let s_slice: &str = "hello";      // immutable view into string data
    let mut s_owned: String = String::from("hello"); // growable, owned
    s_owned.push_str(", world");
    println!("{s_owned}");             // note: {var} inline formatting works directly

    println!("x={x}, y={y}, a={a}, b={b}, f={f}, ready={is_ready}, letter={letter}");

    // Functions are expressions-based: last expr (no `;`) is the return value
    fn add_one(n: i32) -> i32 {
        n + 1   // no semicolon = this is returned. Add `;` and it becomes `()`.
    }
    println!("add_one(5) = {}", add_one(5));
}

// ============================================================================
// 2. CONTROL FLOW: if, loop, while, for, match, loop labels
// ============================================================================
fn control_flow() {
    println!("\n--- 2. CONTROL FLOW ---");

    let n = 7;
    if n % 2 == 0 {
        println!("even");
    } else {
        println!("odd");
    }

    // `if` is an EXPRESSION — can be used to assign a value
    let parity = if n % 2 == 0 { "even" } else { "odd" };

    // `loop`: infinite loop, `break` can return a value from it
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2;   // loop "returns" 10 here
        }
    };
    println!("loop result = {result}");

    // labeled loops let you break/continue an OUTER loop from an inner one
    let mut count = 0;
    'outer: loop {
        loop {
            if count == 3 { break 'outer; }
            count += 1;
        }
    }

    while counter > 0 {
        counter -= 1;
    }

    for i in 0..5 {          // 0..5 is a Range, exclusive of 5 -> 0,1,2,3,4
        print!("{i} ");
    }
    println!();
    for i in 0..=5 {         // 0..=5 is inclusive -> 0,1,2,3,4,5
        print!("{i} ");
    }
    println!();

    let arr = [10, 20, 30];
    for val in arr.iter() {  // iterate over a collection directly
        print!("{val} ");
    }
    println!();

    // `match`: like switch but exhaustive (must cover every case) and can
    // destructure/bind values. This is one of Rust's most-used features.
    let num = 3;
    match num {
        1 => println!("one"),
        2 | 3 => println!("two or three"),   // multiple patterns
        4..=6 => println!("four through six"), // range pattern
        n if n < 0 => println!("negative: {n}"), // guard clause
        _ => println!("something else"),      // `_` = catch-all, required for exhaustiveness
    }
}

// ============================================================================
// 3. OWNERSHIP & BORROWING — Rust's headline feature. No garbage collector,
// no manual free(); the compiler enforces memory safety via these rules:
//   - Each value has exactly one "owner" variable.
//   - When the owner goes out of scope, the value is dropped (freed).
//   - You can have EITHER many immutable borrows OR exactly one mutable
//     borrow of a value at a time (never both) — checked at compile time.
// ============================================================================
fn ownership_and_borrowing() {
    println!("\n--- 3. OWNERSHIP & BORROWING ---");

    let s1 = String::from("hello");
    let s2 = s1;                  // s1 is MOVED into s2. s1 is no longer valid.
    // println!("{s1}");          // ERROR: value borrowed after move
    println!("{s2}");

    let s3 = s2.clone();          // explicit deep copy if you actually want two owners
    println!("s2={s2}, s3={s3}");

    // Simple scalar types (i32, bool, char...) implement `Copy`, so they're
    // copied instead of moved — no ownership drama for them.
    let x = 5;
    let y = x; // copy, x still valid
    println!("x={x}, y={y}");

    // Borrowing: pass a REFERENCE (&) instead of transferring ownership
    fn calculate_length(s: &String) -> usize { // borrows s, doesn't own it
        s.len()
    } // s goes out of scope here, but since it doesn't own the data, nothing is dropped
    let s4 = String::from("borrow me");
    let len = calculate_length(&s4);   // & creates a reference
    println!("'{s4}' has length {len}"); // s4 still usable — it was only borrowed

    // Mutable references: only ONE at a time, and no immutable refs
    // simultaneously — this prevents data races at compile time.
    fn append_world(s: &mut String) {
        s.push_str(" world");
    }
    let mut s5 = String::from("hello");
    append_world(&mut s5);
    println!("{s5}");

    // Slices: a reference to a contiguous part of a collection, no ownership
    let s6 = String::from("hello world");
    let hello: &str = &s6[0..5];   // slice of the String
    let world: &str = &s6[6..11];
    println!("{hello} / {world}");

    let arr = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..3]; // [2, 3]
    println!("{:?}", slice);       // {:?} = "debug" formatting, prints structure
}

// ============================================================================
// 4. STRUCTS & METHODS — Rust's way of grouping data + behavior (no classes)
// ============================================================================
#[derive(Debug)]      // auto-generates a Debug implementation so we can {:?} print it
struct Rectangle {
    width: u32,
    height: u32,
}

// `impl` blocks attach functions to a type
impl Rectangle {
    // Associated function (no `self`) — acts like a "static"/constructor.
    // Called as Rectangle::square(3)
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    // Method (takes &self) — called as instance.method()
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // &mut self methods can modify the instance
    fn double(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
}

struct Point(i32, i32);      // "tuple struct" — fields accessed by .0, .1
struct Unit;                  // "unit struct" — no fields, useful as a marker type

fn structs_and_methods() {
    println!("\n--- 4. STRUCTS & METHODS ---");

    let rect1 = Rectangle { width: 30, height: 50 };
    println!("area = {}", rect1.area());
    println!("{:?}", rect1);        // requires #[derive(Debug)] above
    println!("{:#?}", rect1);       // {:#?} = "pretty" debug printing

    let mut sq = Rectangle::square(10);
    sq.double();
    println!("square doubled area = {}", sq.area());

    let p = Point(1, 2);
    println!("point = ({}, {})", p.0, p.1);
}

// ============================================================================
// 5. ENUMS & PATTERN MATCHING — enums can carry data per-variant, which
// makes `match` extremely powerful. Option<T> and Result<T, E> (below) are
// just enums from the standard library.
// ============================================================================
#[derive(Debug)]
enum Shape {
    Circle(f64),                 // variant holding one value (radius)
    Rectangle { w: f64, h: f64 }, // variant holding named fields (like a mini-struct)
    Point,                        // variant holding nothing
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle { w, h } => w * h,
            Shape::Point => 0.0,
        }
    }
}

fn enums_and_pattern_matching() {
    println!("\n--- 5. ENUMS & PATTERN MATCHING ---");

    let shapes = vec![
        Shape::Circle(2.0),
        Shape::Rectangle { w: 3.0, h: 4.0 },
        Shape::Point,
    ];
    for s in &shapes {
        println!("{:?} -> area {:.2}", s, s.area());
    }

    // Option<T>: Rust has NO null. Absence of a value is modeled explicitly
    // as Option::None; presence is Option::Some(value). Forces you to
    // handle the "missing" case at compile time — no null pointer bugs.
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;

    match some_number {
        Some(n) => println!("got {n}"),
        None => println!("got nothing"),
    }

    // `if let` is shorthand for a match with only one interesting arm
    if let Some(n) = some_number {
        println!("if let matched: {n}");
    }

    // Common Option helpers:
    println!("unwrap_or default: {}", no_number.unwrap_or(0));
    println!("is_some: {}", some_number.is_some());
}

// ============================================================================
// 6. COLLECTIONS: Vec, HashMap, String iteration
// ============================================================================
fn collections() {
    println!("\n--- 6. COLLECTIONS ---");

    // Vec<T>: growable array, heap-allocated
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let v2 = vec![10, 20, 30];        // convenience macro to build+init a Vec

    for x in &v2 {                    // borrow so v2 stays usable after
        print!("{x} ");
    }
    println!();
    println!("v2[1] = {}", v2[1]);          // indexing panics if out of bounds
    println!("v2.get(1) = {:?}", v2.get(1)); // safe version, returns Option

    // HashMap<K, V>: key-value store, no guaranteed order
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    // entry API: insert-or-update pattern (very idiomatic)
    scores.entry(String::from("Blue")).or_insert(0);          // won't overwrite (exists)
    *scores.entry(String::from("Green")).or_insert(0) += 1;   // creates then increments

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Iterating characters/words of a string
    let sentence = "the quick brown fox";
    for word in sentence.split_whitespace() {
        print!("[{word}] ");
    }
    println!();
}

// ============================================================================
// 7. ERROR HANDLING — Rust has no exceptions. Recoverable errors use
// Result<T, E>; unrecoverable ones use panic!(). The `?` operator
// propagates an Err up to the caller automatically.
// ============================================================================
#[derive(Debug)]
struct MyError(String);

fn might_fail(succeed: bool) -> Result<i32, MyError> {
    if succeed {
        Ok(42)
    } else {
        Err(MyError("something went wrong".to_string()))
    }
}

// `?` unwraps Ok(x) into x, or returns early with the Err — massively reduces
// boilerplate compared to manually matching every call site.
fn chain_calls() -> Result<i32, MyError> {
    let a = might_fail(true)?;
    let b = might_fail(true)?;
    Ok(a + b)
}

fn error_handling() {
    println!("\n--- 7. ERROR HANDLING ---");

    match might_fail(true) {
        Ok(v) => println!("succeeded: {v}"),
        Err(e) => println!("failed: {:?}", e),
    }

    match chain_calls() {
        Ok(v) => println!("chain succeeded: {v}"),
        Err(e) => println!("chain failed: {:?}", e),
    }

    // unwrap()/expect() panic (crash) on Err/None — fine for prototypes,
    // avoid in production code where the error is expected/handleable.
    let val = might_fail(true).unwrap();
    println!("unwrapped: {val}");

    // "1".parse::<i32>() is the classic example of Result in the stdlib
    let parsed: Result<i32, _> = "42".parse::<i32>();
    println!("parsed = {:?}", parsed);
}

// ============================================================================
// 8. GENERICS & TRAITS
// Generics: write code once that works over many types.
// Traits: like interfaces — define shared behavior types must implement.
// ============================================================================

// Generic function: works for any type T that implements PartialOrd (comparable)
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generic struct
struct Pair<T> {
    first: T,
    second: T,
}

// A trait: defines a method signature any implementer must provide
trait Summary {
    fn summarize(&self) -> String {
        String::from("(no summary available)") // default implementation, overridable
    }
}

struct Article { title: String, body: String }
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}...", self.title, &self.body[..10.min(self.body.len())])
    }
}

struct Tweet { user: String }
impl Summary for Tweet {} // uses the default summarize()

// `impl Trait` in argument position: accepts anything implementing Summary
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound with `where` clause (equivalent, more readable for complex bounds)
fn notify_generic<T>(item: &T) where T: Summary {
    println!("Also: {}", item.summarize());
}

fn generics_and_traits() {
    println!("\n--- 8. GENERICS & TRAITS ---");

    let numbers = vec![34, 50, 25, 100, 65];
    println!("largest number = {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("largest char = {}", largest(&chars));

    let pair = Pair { first: 1, second: 2 };
    println!("pair = ({}, {})", pair.first, pair.second);

    let article = Article { title: "Rust 2.0".into(), body: "Big changes are coming".into() };
    let tweet = Tweet { user: "rustlang".into() };
    notify(&article);
    notify_generic(&tweet);
}

// ============================================================================
// 9. CLOSURES & ITERATORS — functional-style programming in Rust
// ============================================================================
fn closures_and_iterators() {
    println!("\n--- 9. CLOSURES & ITERATORS ---");

    // Closures: anonymous functions that can CAPTURE variables from their
    // environment. Type annotations usually optional (inferred).
    let add = |a, b| a + b;
    println!("closure add(2,3) = {}", add(2, 3));

    let multiplier = 3;
    let scale = |x: i32| x * multiplier;  // captures `multiplier` from enclosing scope
    println!("scale(5) = {}", scale(5));

    // Iterators are LAZY — nothing runs until you consume them (collect, sum, for..)
    let v = vec![1, 2, 3, 4, 5];

    let doubled: Vec<i32> = v.iter()
        .map(|x| x * 2)        // transform each element
        .collect();            // consume the iterator into a Vec
    println!("doubled = {:?}", doubled);

    let evens: Vec<&i32> = v.iter()
        .filter(|&&x| x % 2 == 0)  // keep only elements matching predicate
        .collect();
    println!("evens = {:?}", evens);

    let sum: i32 = v.iter().sum();
    let total: i32 = v.iter().map(|x| x * x).sum(); // chain map + sum: sum of squares
    println!("sum = {sum}, sum of squares = {total}");

    // Chaining several iterator adapters is idiomatic and compiles to
    // tight loops — no performance penalty vs hand-written loops.
    let result: Vec<i32> = v.iter()
        .filter(|&&x| x > 1)
        .map(|x| x * 10)
        .take(2)                // only take first 2 results
        .collect();
    println!("chained result = {:?}", result);
}

// ============================================================================
// 10. SMART POINTERS — for when ownership rules alone aren't flexible enough
// ============================================================================
fn smart_pointers() {
    println!("\n--- 10. SMART POINTERS ---");

    // Box<T>: heap-allocate a value; useful for recursive types or
    // when you need a fixed-size pointer regardless of the inner type's size
    let boxed = Box::new(5);
    println!("boxed = {}", *boxed);  // * dereferences

    // Rc<T>: "Reference Counted" — allows MULTIPLE owners of the same data
    // (single-threaded only). Data is dropped when the last Rc is dropped.
    let a = Rc::new(String::from("shared"));
    let b = Rc::clone(&a);  // cheap: just bumps a counter, doesn't copy the data
    println!("rc count = {}, a = {}, b = {}", Rc::strong_count(&a), a, b);

    // RefCell<T>: lets you mutate data even through an immutable reference,
    // enforcing borrow rules at RUNTIME instead of compile time. Often
    // paired with Rc to get shared, mutable data in single-threaded code.
    let shared_mutable = Rc::new(RefCell::new(vec![1, 2, 3]));
    let clone_ref = Rc::clone(&shared_mutable);
    clone_ref.borrow_mut().push(4); // mutate through the shared reference
    println!("shared_mutable = {:?}", shared_mutable.borrow());
}

// ============================================================================
// 11. CONCURRENCY — threads + message passing, safe by the same
// ownership/borrowing rules (the compiler prevents data races)
// ============================================================================
fn concurrency() {
    println!("\n--- 11. CONCURRENCY ---");

    // std::thread::spawn runs a closure on a new OS thread.
    // `move` forces the closure to take ownership of captured variables,
    // required because the thread may outlive the current scope.
    let handle = thread::spawn(move || {
        for i in 1..3 {
            println!("thread says: {i}");
        }
        "thread result"
    });
    let result = handle.join().unwrap(); // wait for the thread to finish
    println!("joined thread returned: {result}");

    // Arc<T> = Rc<T> but thread-safe (Atomic Reference Counted).
    // Mutex<T> = mutual exclusion lock for safe shared mutable state.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // blocks until lock acquired
            *num += 1;
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("final counter = {}", *counter.lock().unwrap());

    // mpsc = "multi-producer, single-consumer" channels for message passing
    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("hello from another thread").unwrap();
    });
    println!("received: {}", rx.recv().unwrap());
}

// ============================================================================
// 12. MODULES & VISIBILITY — organizing code into namespaces
// ============================================================================
mod modules_demo {
    // Everything is PRIVATE by default; `pub` exposes it to outside callers.
    pub fn run() {
        println!("\n--- 12. MODULES ---");
        println!("2 + 3 = {}", math::add(2, 3));
        println!("greeting = {}", greetings::hello("Rustacean"));
    }

    mod math {
        pub fn add(a: i32, b: i32) -> i32 { a + b }
        fn internal_helper() -> i32 { 0 } // private, only usable inside `math`
    }

    mod greetings {
        pub fn hello(name: &str) -> String {
            format!("Hello, {name}!")
        }
    }
}

// ============================================================================
// QUICK REFERENCE
// ============================================================================
// - Cargo (the build tool/package manager):
//     cargo new my_project      -> scaffold a new binary project
//     cargo build / cargo run   -> compile / compile & run
//     cargo test                -> run #[test] functions
//     Dependencies go in Cargo.toml under [dependencies]
//
// - Lifetimes ('a): annotate how long references are valid, e.g.
//     fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }
//   The compiler infers most lifetimes; you only write them when it can't.
//
// - Custom Display trait (for user-facing formatting instead of Debug):
//     impl fmt::Display for Rectangle {
//         fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//             write!(f, "{}x{}", self.width, self.height)
//         }
//     }
//
// - Operator overloading, custom iterators, async/await, macros, and unsafe
//   are the natural "next steps" once the above feels comfortable.
// ============================================================================
