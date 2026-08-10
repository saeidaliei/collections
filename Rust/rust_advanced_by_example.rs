// ============================================================================
// RUST BY EXAMPLE
// ----------------------------------------------------------------------------
// Companion to rust_by_example_summary.rs. Assumes you already know:
// ownership, structs, enums, basic traits/generics, Option/Result, closures.
// ============================================================================
#![allow(dead_code, unused_variables, unused_mut)]

use std::error::Error;
use std::fmt;
use std::ops::Add;

fn main() {
    macros_demo();
    advanced_modules::run();
    advanced_traits();
    advanced_error_handling().unwrap_or_else(|e| println!("top-level error: {e}"));
    lifetimes_demo();
    custom_iterator();
    advanced_pattern_matching();
    const_generics_demo();
    unsafe_basics();
    async_await_notes();
}

// ============================================================================
// 1. MACROS — code that writes code, expanded at compile time.
// Two main kinds: declarative (`macro_rules!`, shown here) and procedural
// (used for #[derive(...)], custom attributes, etc. — need a separate crate,
// so they're only described in comments below).
// ============================================================================

// A declarative macro. `$name:expr` etc. are "metavariables" with a
// "fragment specifier" (expr, ident, ty, block, tt...) telling the macro
// what kind of syntax to capture.
macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

// Macros can have multiple match arms, like `match` for syntax patterns
macro_rules! max {
    ($a:expr) => { $a };
    ($a:expr, $b:expr) => {
        if $a > $b { $a } else { $b }
    };
    // recursive case: reduces max!(a, b, c, ...) down to nested calls
    ($a:expr, $($rest:expr),+) => {
        max!($a, max!($($rest),+))
    };
}

// Variadic macro — accepts any number of comma-separated expressions,
// similar to how the built-in vec![] macro works.
macro_rules! make_vec {
    ($($item:expr),* $(,)?) => {
        {
            let mut v = Vec::new();
            $( v.push($item); )*
            v
        }
    };
}

fn macros_demo() {
    println!("--- 1. MACROS ---");

    println!("square!(5) = {}", square!(5));
    println!("max!(3, 7, 2, 9, 1) = {}", max!(3, 7, 2, 9, 1));

    let v: Vec<i32> = make_vec![10, 20, 30];
    println!("make_vec! -> {:?}", v);

    // Handy built-in macros worth knowing:
    assert!(1 + 1 == 2);                 // panics if false
    assert_eq!(2 + 2, 4);                // panics with a diff if not equal
    debug_assert!(true);                 // like assert! but stripped in release builds
    let is_some = matches!(Some(3), Some(x) if x > 2); // pattern-match as a boolean expr
    println!("matches! result = {is_some}");

    // Note on procedural macros (not runnable in a single file — they live
    // in their own crate with `proc-macro = true` in Cargo.toml):
    //   #[derive(Debug, Clone, PartialEq)]   <- derive macro: auto-implements traits
    //   #[my_custom_attribute]               <- attribute macro
    //   my_function_macro!(...)              <- function-like proc macro
    // Popular crates like `serde` (serialization) and `thiserror` (error
    // types) are built almost entirely from procedural + derive macros.
}

// ============================================================================
// 2. ADVANCED MODULES — visibility, re-exports, nested trees, `use` paths
// ============================================================================
mod advanced_modules {
    pub fn run() {
        println!("\n--- 2. ADVANCED MODULES ---");
        // `super::` refers to the parent module, `crate::` to the crate root
        println!("{}", inner::deep::greet());
        println!("{}", Reexported::hello()); // usable thanks to `pub use` below
    }

    mod inner {
        pub mod deep {
            pub fn greet() -> String {
                // reach back up to a sibling module with `super`
                format!("deep says hi, sibling value = {}", super::sibling_value())
            }
        }
        pub fn sibling_value() -> i32 { 99 }
    }

    // `pub use` re-exports an item under this module's own path — this is
    // how library crates flatten deep internal module trees into a clean
    // public API (e.g. `pub use inner::deep::Thing as Reexported;`)
    pub use inner::deep::greet as hello_fn;
    pub struct Reexported;
    impl Reexported {
        pub fn hello() -> &'static str { "re-exported item" }
    }

    // `pub(crate)` = visible anywhere in this crate, but not to external
    // users of the crate if this were a library. Useful middle ground
    // between fully private and fully `pub`.
    pub(crate) fn crate_visible_helper() -> i32 { 1 }
}

// In a real multi-file project, modules map to files/folders:
//   src/main.rs         -> crate root
//   src/network.rs      -> `mod network;` declared in main.rs
//   src/network/mod.rs  -> alternative layout for a module with children
// External crates are pulled in via Cargo.toml [dependencies] and then
// referenced with `use crate_name::Thing;` — no separate `mod` needed for them.

// ============================================================================
// 3. ADVANCED TRAITS — associated types, trait objects, operator
// overloading, supertraits, blanket impls
// ============================================================================

// Associated type: lets a trait say "implementers pick ONE concrete type
// for this slot" instead of the trait being generic over it. Iterator
// itself is defined this way in std (`type Item;`).
trait Container {
    type Item;
    fn get(&self, i: usize) -> Option<&Self::Item>;
}
struct Bucket { items: Vec<i32> }
impl Container for Bucket {
    type Item = i32;
    fn get(&self, i: usize) -> Option<&i32> { self.items.get(i) }
}

// Operator overloading: implement std::ops traits to make custom types
// work with +, -, ==, etc.
#[derive(Debug, Clone, Copy)]
struct Vec2 { x: f64, y: f64 }
impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}

// Supertrait: `Named` requires that anything implementing it ALSO
// implements `fmt::Display` — you can then rely on `to_string()` etc.
trait Named: fmt::Display {
    fn name_len(&self) -> usize {
        self.to_string().len() // allowed because Display is a supertrait bound
    }
}
struct Robot(String);
impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Named for Robot {}

// Blanket implementation: implement a trait for EVERY type that satisfies
// some bound, instead of one type at a time. Standard library does this a
// lot (e.g. `impl<T: Display> ToString for T`).
trait Loud {
    fn shout(&self) -> String;
}
impl<T: fmt::Display> Loud for T {
    fn shout(&self) -> String {
        format!("{}!!!", self.to_string().to_uppercase())
    }
}

// Trait objects (`dyn Trait`): when you need a collection of DIFFERENT
// concrete types that all implement the same trait, resolved at RUNTIME
// (dynamic dispatch) rather than compile time (unlike generics/impl Trait).
trait Animal { fn speak(&self) -> String; }
struct Dog; struct Cat;
impl Animal for Dog { fn speak(&self) -> String { "Woof".into() } }
impl Animal for Cat { fn speak(&self) -> String { "Meow".into() } }

fn advanced_traits() {
    println!("\n--- 3. ADVANCED TRAITS ---");

    let bucket = Bucket { items: vec![1, 2, 3] };
    println!("bucket.get(1) = {:?}", bucket.get(1));

    let a = Vec2 { x: 1.0, y: 2.0 };
    let b = Vec2 { x: 3.0, y: 4.0 };
    println!("a + b = {:?}", a + b); // uses our Add impl

    let r = Robot("R2D2".into());
    println!("robot name_len = {}", r.name_len());

    println!("shout: {}", 42.shout());           // Loud blanket impl via Display
    println!("shout: {}", "quiet".shout());

    // Vec<Box<dyn Animal>>: heterogeneous collection, dispatched at runtime.
    // Box is needed because trait objects don't have a known size at
    // compile time — Box puts them on the heap behind a fixed-size pointer.
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    for a in &animals {
        println!("animal says: {}", a.speak());
    }
}

// ============================================================================
// 4. ADVANCED ERROR HANDLING — custom Error types, `From` for automatic
// conversion (what makes `?` work across error types), boxed errors
// ============================================================================

#[derive(Debug)]
enum AppError {
    NotFound(String),
    Parse(std::num::ParseIntError),
}

// Implementing std::error::Error (plus Display) makes your type a "real"
// error that plays nicely with the wider ecosystem (and with Box<dyn Error>).
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NotFound(s) => write!(f, "not found: {s}"),
            AppError::Parse(e) => write!(f, "parse error: {e}"),
        }
    }
}
impl Error for AppError {}

// `From` lets the `?` operator auto-convert a ParseIntError into our
// AppError with zero extra code at the call site.
impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

fn find_user(id: &str) -> Result<String, AppError> {
    let numeric_id: i32 = id.parse()?; // ParseIntError auto-converts to AppError via `From`
    if numeric_id == 1 {
        Ok("Alice".to_string())
    } else {
        Err(AppError::NotFound(id.to_string()))
    }
}

// When you don't care about the concrete error type and just want
// "any error", `Box<dyn Error>` is the common catch-all return type —
// think of it as `dyn Trait` applied to error handling.
fn might_fail_generic(x: i32) -> Result<i32, Box<dyn Error>> {
    if x < 0 {
        return Err("x must be non-negative".into()); // &str -> Box<dyn Error> via From
    }
    let parsed: i32 = "10".parse()?; // ParseIntError -> Box<dyn Error> also via From
    Ok(x + parsed)
}

fn advanced_error_handling() -> Result<(), Box<dyn Error>> {
    println!("\n--- 4. ADVANCED ERROR HANDLING ---");

    match find_user("1") {
        Ok(name) => println!("found: {name}"),
        Err(e) => println!("error: {e}"),
    }
    match find_user("abc") {
        Ok(name) => println!("found: {name}"),
        Err(e) => println!("error: {e}"), // will be a Parse error
    }
    match find_user("99") {
        Ok(name) => println!("found: {name}"),
        Err(e) => println!("error: {e}"), // will be NotFound
    }

    let r = might_fail_generic(5)?; // propagate via ? all the way to main's handler
    println!("might_fail_generic(5) = {r}");

    // NOTE: in real projects, the `thiserror` crate removes the Display/Error
    // boilerplate above via #[derive(thiserror::Error)], and `anyhow` crate
    // gives an even more ergonomic version of Box<dyn Error> for
    // applications (as opposed to libraries, which should keep concrete
    // error enums like AppError for callers to match on).
    Ok(())
}

// ============================================================================
// 5. LIFETIMES — annotations describing how long references stay valid.
// The compiler infers most of these; you write them explicitly when a
// function/struct's output reference could be tied to more than one input.
// ============================================================================

// Without 'a here, the compiler can't tell if the returned reference should
// live as long as `x` or as long as `y` — the annotation says "the result
// lives at least as long as the SHORTER of the two input lifetimes."
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Structs holding references must declare the lifetime of what they borrow —
// this guarantees the struct can never outlive the data it points to.
struct Excerpt<'a> {
    part: &'a str,
}
impl<'a> Excerpt<'a> {
    fn announce(&self, greeting: &str) -> &str {
        println!("Attention: {greeting}");
        self.part
    }
}

fn lifetimes_demo() {
    println!("\n--- 5. LIFETIMES ---");

    let s1 = String::from("long string is long");
    let s2 = String::from("short");
    println!("longest = {}", longest(&s1, &s2));

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = Excerpt { part: first_sentence }; // excerpt can't outlive `novel`
    println!("excerpt.part = {}", excerpt.announce("listen up"));

    // 'static is a special lifetime meaning "valid for the entire program" —
    // string literals are 'static because they're baked into the binary.
    let static_str: &'static str = "I live forever";
    println!("{static_str}");
}

// ============================================================================
// 6. CUSTOM ITERATORS — implement the Iterator trait to get map/filter/
// sum/collect/... for free on your own type
// ============================================================================
struct Countdown(u32);

impl Iterator for Countdown {
    type Item = u32; // associated type again — Iterator's "yield type"

    fn next(&mut self) -> Option<u32> {
        if self.0 == 0 {
            None            // signals the iterator is exhausted
        } else {
            self.0 -= 1;
            Some(self.0 + 1)
        }
    }
}

fn custom_iterator() {
    println!("\n--- 6. CUSTOM ITERATORS ---");

    let cd = Countdown(5);
    let collected: Vec<u32> = cd.collect(); // all adapters work automatically!
    println!("countdown collected = {:?}", collected);

    let sum: u32 = Countdown(3).sum();
    println!("countdown sum = {sum}");

    let doubled: Vec<u32> = Countdown(3).map(|x| x * 2).collect();
    println!("countdown doubled = {:?}", doubled);
}

// ============================================================================
// 7. ADVANCED PATTERN MATCHING — @ bindings, nested/struct destructuring,
// matching on references, ignoring parts of a value
// ============================================================================
struct User { name: String, age: u32 }

fn advanced_pattern_matching() {
    println!("\n--- 7. ADVANCED PATTERN MATCHING ---");

    let n = 5;
    match n {
        x @ 1..=5 => println!("got {x}, in range 1..=5"), // `@` binds AND tests a pattern
        _ => println!("out of range"),
    }

    let user = User { name: "Ana".to_string(), age: 30 };
    match user {
        User { name, age: 18..=30 } => println!("{name} is a young adult"),
        User { name, .. } => println!("{name} is some other age"), // `..` ignores remaining fields
    }

    let pair = (1, -1);
    match pair {
        (0, y) => println!("first is zero, y={y}"),
        (x, 0) => println!("second is zero, x={x}"),
        (x, y) if x == -y => println!("{x} and {y} are opposites"),
        _ => println!("no special relationship"),
    }

    // Matching through a reference: `&` in the pattern peels the reference off
    let v = vec![1, 2, 3];
    for &item in v.iter() {
        print!("{item} ");
    }
    println!();

    // Destructuring nested enums/options in one match
    let nested: Option<Result<i32, String>> = Some(Ok(10));
    match nested {
        Some(Ok(n)) => println!("nested ok: {n}"),
        Some(Err(e)) => println!("nested err: {e}"),
        None => println!("nothing at all"),
    }
}

// ============================================================================
// 8. CONST GENERICS — generic over a compile-time VALUE (not just a type),
// commonly used for fixed-size arrays
// ============================================================================
fn first_n<const N: usize>(arr: [i32; N]) -> i32 {
    arr[0]
}

struct FixedBuffer<const SIZE: usize> {
    data: [u8; SIZE],
}
impl<const SIZE: usize> FixedBuffer<SIZE> {
    fn new() -> Self {
        FixedBuffer { data: [0; SIZE] }
    }
    fn len(&self) -> usize { SIZE }
}

fn const_generics_demo() {
    println!("\n--- 8. CONST GENERICS ---");
    println!("first_n([1,2,3]) = {}", first_n([1, 2, 3]));

    let buf = FixedBuffer::<16>::new(); // SIZE is a compile-time constant, not a runtime value
    println!("buffer len = {}", buf.len());
}

// ============================================================================
// 9. UNSAFE RUST — an escape hatch for the small set of operations the
// compiler can't verify are safe. You take on the responsibility manually.
// Needed for: raw pointer deref, calling FFI (C functions), mutable
// statics, implementing certain low-level traits. Use sparingly.
// ============================================================================
fn unsafe_basics() {
    println!("\n--- 9. UNSAFE ---");

    let x = 5;
    let raw_ptr = &x as *const i32; // create a raw pointer (allowed in safe code)

    unsafe {
        // Dereferencing a raw pointer MUST happen inside an `unsafe` block —
        // the compiler trusts YOU that the pointer is valid here.
        println!("dereferenced raw pointer = {}", *raw_ptr);
    }

    // A realistic use case is calling into C libraries:
    // extern "C" { fn abs(input: i32) -> i32; }
    // let result = unsafe { abs(-3) };
    // (omitted here since it needs linking against libc)
}

// ============================================================================
// 10. ASYNC/AWAIT — concurrency model for I/O-bound work (network, disk).
// Requires an async RUNTIME (e.g. the `tokio` crate) that isn't available
// in a plain single-file rustc build, so this section is illustrative only.
// ============================================================================
fn async_await_notes() {
    println!("\n--- 10. ASYNC/AWAIT (reference only, not executed) ---");
    println!("See comments in source for a syntax example.");

    // Typical shape once you have `tokio` as a dependency in Cargo.toml:
    //
    // #[tokio::main]
    // async fn main() {
    //     let data = fetch_data().await;   // `.await` suspends until the future resolves
    //     println!("{data}");
    // }
    //
    // async fn fetch_data() -> String {
    //     // e.g. reqwest::get(url).await.unwrap().text().await.unwrap()
    //     "some data".to_string()
    // }
    //
    // Key ideas:
    // - `async fn` returns a `Future` immediately; nothing runs until it's
    //   `.await`ed (like iterators being lazy until consumed).
    // - Many futures can be run CONCURRENTLY on one thread via the runtime's
    //   scheduler — great for many simultaneous I/O waits, not for CPU-bound work.
    // - `tokio::spawn(future)` runs a future concurrently, similar in spirit
    //   to `thread::spawn` but much lighter weight ("green threads"/tasks).
    // - `join!(fut1, fut2)` / `tokio::join!` runs multiple futures concurrently
    //   and waits for all to finish.
}
