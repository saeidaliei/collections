// A small tour of some fundamental Rust concepts.
//   1. Ownership
//   2. Borrowing & references
//   3. Mutable borrowing
//   4. Slices / views
//   5. Generics
//   6. Traits
//   7. Option
//   8. Result
//   9. Closures
//  10. Iterators
//  11. Putting everything together


fn main() {
    println!("=== Rust Concepts ===\n");

    ownership();
    borrowing();
    mutable_borrowing();
    slices();
    generics();
    traits();
    option_example();
    result_example();
    closures();
    iterators();
    combined_example();
}


// ============================================================
// 1. OWNERSHIP
// ============================================================
//
// Every value in Rust has an owner.
//
// When the owner goes out of scope, the value is dropped.
//
// Ownership is one of the main things that distinguishes Rust
// from languages like C++.

fn ownership() {
    println!("--- 1. Ownership ---");

    let s = String::from("hello");

    // `s` owns the String.
    println!("s = {s}");

    // Assigning a String to another variable MOVES ownership.
    //
    // `s2` becomes the owner.
    // `s` can no longer be used.
    let s2 = s;

    println!("s2 = {s2}");

    // This would NOT compile:
    //
    // println!("{s}");
    //
    // because ownership moved from `s` to `s2`.

    // Copy types behave differently.
    //
    // Integers implement Copy, so assigning them makes a copy.
    let x = 42;
    let y = x;

    println!("x = {x}, y = {y}");

    // Both are still usable because i32 is Copy.

    println!();
}


// ============================================================
// 2. BORROWING & REFERENCES
// ============================================================
//
// Instead of giving ownership to a function, we can BORROW
// the value.
//
// `&T` means "a reference to T".
//
// The reference lets us access the value without owning it.

fn borrowing() {
    println!("--- 2. Borrowing ---");

    let s = String::from("hello");

    // `&s` creates a reference to s.
    //
    // We are NOT moving s.
    let reference = &s;

    println!("original: {s}");
    println!("through reference: {reference}");

    // s is still usable because the reference only borrowed it.

    // This function borrows s rather than taking ownership.
    print_string(&s);

    // s is still valid here.
    println!("after function call: {s}");

    println!();
}


// A function can accept a reference instead of taking ownership.

fn print_string(s: &String) {
    println!("inside function: {s}");
}


// ============================================================
// 3. MUTABLE BORROWING
// ============================================================
//
// `&T`  = immutable reference
// `&mut T` = mutable reference
//
// An immutable reference allows reading.
//
// A mutable reference allows modifying.
//
// Rust's important rule:
//
// You can have:
//   - many immutable references
//
// OR:
//   - one mutable reference
//
// But not both at the same time.

fn mutable_borrowing() {
    println!("--- 3. Mutable Borrowing ---");

    let mut s = String::from("hello");

    // Immutable borrow.
    let r1 = &s;
    let r2 = &s;

    println!("r1 = {r1}");
    println!("r2 = {r2}");

    // Multiple readers are okay.

    // Once those immutable borrows are no longer being used,
    // we can create a mutable borrow.
    let r3 = &mut s;

    r3.push_str(" world");

    println!("after mutable borrow: {r3}");

    // This would NOT be allowed while r1/r2 were still being used:
    //
    // let r3 = &mut s;
    //
    // because Rust prevents simultaneous mutable and immutable access.

    println!();
}


// ============================================================
// 4. SLICES / VIEWS
// ============================================================
//
// A slice is a VIEW into existing data.
//
// It doesn't own the data.
//
// String:
//
//     String       owns the text
//     &str         borrowed view of text
//
// Vec<T>:
//
//     Vec<T>       owns the elements
//     &[T]         borrowed view of some/all elements
//
// This is one of the most useful examples of borrowing.

fn slices() {
    println!("--- 4. Slices ---");

    let s = String::from("hello world");

    // Borrow only part of the String.
    let hello = &s[0..5];

    println!("whole string: {s}");
    println!("slice: {hello}");

    //
    // Conceptually:
    //
    // s:
    //
    //   h e l l o   w o r l d
    //   ^^^^^^^^^
    //      &str
    //
    // The slice doesn't contain a separate copy of "hello".
    // It is a view into s.

    let numbers = vec![10, 20, 30, 40, 50];

    // View into part of the vector.
    let middle = &numbers[1..4];

    println!("numbers: {numbers:?}");
    println!("middle slice: {middle:?}");

    // &[i32] is a borrowed view into the Vec<i32>.

    println!();
}


// ============================================================
// 5. GENERICS
// ============================================================
//
// Generics allow us to write code that works with multiple types.
//
// Instead of:
//
//     fn first_i32(...)
//     fn first_string(...)
//     fn first_f64(...)
//
// we can write one generic function:
//
//     fn first<T>(...)
//
// T is a TYPE PARAMETER.

fn generics() {
    println!("--- 5. Generics ---");

    let a = first(10, 20);
    println!("first integer: {a}");

    let b = first("hello", "world");
    println!("first string: {b}");

    let c = first(1.5, 2.5);
    println!("first float: {c}");

    // Rust figures out T from the arguments.
    //
    // first(10, 20)
    //     T = i32
    //
    // first("hello", "world")
    //     T = &str
    //
    // first(1.5, 2.5)
    //     T = f64

    println!();
}


// T means "some type".
//
// Both arguments must have the same type T.

fn first<T>(a: T, _b: T) -> T {
    a
}


// ============================================================
// 6. TRAITS
// ============================================================
//
// A trait describes BEHAVIOR that a type can implement.
//
// Roughly comparable to an interface in other languages,
// although Rust traits are more powerful and flexible.
//
// Here we define our own trait.

trait Animal {
    fn make_sound(&self);
}


// A struct is a custom type.

struct Dog;


// Dog implements the Animal trait.

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}


struct Cat;


impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}


fn traits() {
    println!("--- 6. Traits ---");

    let dog = Dog;
    let cat = Cat;

    dog.make_sound();
    cat.make_sound();

    // Both Dog and Cat implement Animal.
    //
    // This means both have the behavior required by Animal.

    // A generic function can require a trait.

    print_animal_sound(&dog);
    print_animal_sound(&cat);

    println!();
}


// T can be any type, but it MUST implement Animal.
//
// T: Animal
//
// means:
//
// "T is some type that implements the Animal trait."

fn print_animal_sound<T: Animal>(animal: &T) {
    animal.make_sound();
}


// ============================================================
// 7. OPTION<T>
// ============================================================
//
// Option represents:
//
//     "There might be a value."
//
// It has two possibilities:
//
//     Some(value)
//     None
//
// This is Rust's explicit way of representing the absence
// of a value.
//
// Unlike C/C++, Rust doesn't normally use null references.

fn option_example() {
    println!("--- 7. Option ---");

    let some_number: Option<i32> = Some(42);

    let no_number: Option<i32> = None;

    println!("some_number = {some_number:?}");
    println!("no_number = {no_number:?}");

    // We can use match to handle both possibilities.

    match some_number {
        Some(value) => println!("Got a number: {value}"),
        None => println!("There was no number"),
    }

    match no_number {
        Some(value) => println!("Got a number: {value}"),
        None => println!("There was no number"),
    }

    // A realistic example:
    //
    // A search might find something or not.

    let numbers = vec![10, 20, 30];

    let result = find_number(&numbers, 20);

    match result {
        Some(value) => println!("Found: {value}"),
        None => println!("Number wasn't found"),
    }

    println!();
}


// Notice the return type:
//
// Option<&i32>
//
// This means:
//
//     Some(reference to an i32)
// OR
//     None
//
// The returned reference BORROWS the value from numbers.
// It doesn't take ownership.

fn find_number(numbers: &[i32], target: i32) -> Option<&i32> {
    for number in numbers {
        if *number == target {
            return Some(number);
        }
    }

    None
}


// ============================================================
// 8. RESULT<T, E>
// ============================================================
//
// Result represents:
//
//     "The operation succeeded OR it failed."
//
// It has two possibilities:
//
//     Ok(value)
//     Err(error)
//
// Option:
//
//     Some / None
//
// Result:
//
//     Ok / Err
//
// Result is commonly used for things like:
//     - reading files
//     - parsing
//     - network operations
//     - operations that can fail

fn result_example() {
    println!("--- 8. Result ---");

    let successful: Result<i32, &str> = Ok(42);

    let failed: Result<i32, &str> = Err("something went wrong");

    println!("successful = {successful:?}");
    println!("failed = {failed:?}");

    match successful {
        Ok(value) => println!("Success: {value}"),
        Err(error) => println!("Error: {error}"),
    }

    match failed {
        Ok(value) => println!("Success: {value}"),
        Err(error) => println!("Error: {error}"),
    }

    // A real example:
    //
    // Parsing a string into an integer can fail.

    let parsed = "42".parse::<i32>();

    match parsed {
        Ok(number) => println!("Parsed number: {number}"),
        Err(error) => println!("Couldn't parse number: {error}"),
    }

    let invalid = "hello".parse::<i32>();

    match invalid {
        Ok(number) => println!("Parsed number: {number}"),
        Err(error) => println!("Couldn't parse number: {error}"),
    }

    println!();
}


// ============================================================
// 9. CLOSURES
// ============================================================
//
// A closure is essentially an anonymous function that can
// be stored in a variable, passed to another function, etc.
//
// Syntax:
//
//     |arguments| expression
//
// Example:
//
//     |x| x * 2

fn closures() {
    println!("--- 9. Closures ---");

    let add_one = |x| x + 1;

    println!("add_one(5) = {}", add_one(5));

    let square = |x| x * x;

    println!("square(5) = {}", square(5));

    // Closures can capture variables from their surrounding scope.

    let amount = 10;

    let add_amount = |x| x + amount;

    println!("add_amount(5) = {}", add_amount(5));

    // The closure remembers `amount`.

    //
    // Conceptually:
    //
    // amount = 10
    //
    //       ↓
    //   ┌──────────────┐
    //   │ closure      │
    //   │              │
    //   │ x + amount   │
    //   │       ↑      │
    //   │      10      │
    //   └──────────────┘
    //

    println!();
}


// ============================================================
// 10. ITERATORS
// ============================================================
//
// Iterators let us process sequences of values.
//
// Rust's iterator system is heavily connected to closures.
//
// Common methods:
//
//     map
//     filter
//     find
//     collect
//
// These methods often take closures.

fn iterators() {
    println!("--- 10. Iterators ---");

    let numbers = vec![1, 2, 3, 4, 5];

    // map transforms every element.

    let doubled: Vec<i32> = numbers
        .iter()
        .map(|x| x * 2)
        .collect();

    println!("numbers: {numbers:?}");
    println!("doubled: {doubled:?}");

    // filter keeps only elements satisfying a condition.

    let even: Vec<&i32> = numbers
        .iter()
        .filter(|x| **x % 2 == 0)
        .collect();

    println!("even: {even:?}");

    // find returns an Option.
    //
    // So iterators and Option naturally work together.

    let found = numbers
        .iter()
        .find(|x| **x == 3);

    match found {
        Some(value) => println!("Found: {value}"),
        None => println!("Not found"),
    }

    println!();
}


// ============================================================
// 11. PUTTING EVERYTHING TOGETHER
// ============================================================
//
// This example combines:
//
//     Generics
//     Traits
//     Closures
//     Borrowing
//     Slices
//     Option
//     Iterators
//
// It looks complicated at first:
//
//     fn find<T: PartialEq>(
//         items: &[T],
//         predicate: impl Fn(&T) -> bool,
//     ) -> Option<&T>
//
// But each piece has a specific job.

fn combined_example() {
    println!("--- 11. Combined Example ---");

    let numbers = vec![1, 4, 7, 10, 13];

    // `&numbers` borrows the vector.
    //
    // The function receives a slice, which is a borrowed view.
    //
    // The closure decides what we're looking for.
    //
    // Option represents the possibility that nothing matches.

    let result = find_with_predicate(
        &numbers,
        |x| *x > 8,
    );

    match result {
        Some(number) => println!("Found number > 8: {number}"),
        None => println!("No number > 8"),
    }

    println!();
}


// Generic function.
//
// T = some type.
//
// T: PartialEq
//     means T must support equality comparisons.
//
// &[T]
//     means borrowed slice of T.
//
// impl Fn(&T) -> bool
//     means we accept something callable like a closure.
//
// Option<&T>
//     means we either return a borrowed T or nothing.

fn find_with_predicate<T: PartialEq>(
    items: &[T],
    predicate: impl Fn(&T) -> bool,
) -> Option<&T> {

    for item in items {
        if predicate(item) {
            return Some(item);
        }
    }

    None
}


// ============================================================
// END
// ============================================================
//
// OWNERSHIP
//     Values have owners.
//     Ownership can move.
//
// BORROWING
//     &T lets you temporarily access something without owning it.
//
// MUTABLE BORROWING
//     &mut T lets you modify borrowed data.
//
// SLICES
//     &[T] and &str are borrowed views into existing data.
//
// GENERICS
//     T lets code work with different types.
//
// TRAITS
//     Describe behavior that types can implement.
//
// OPTION
//     Some(T) or None.
//
// RESULT
//     Ok(T) or Err(E).
//
// CLOSURES
//     Anonymous functions that can capture their environment.
//
// ITERATORS
//     Process sequences using methods such as map/filter/find.
//
//
// The big picture:
//
//                 OWNERSHIP
//                     │
//                     ↓
//                 BORROWING
//                     │
//              ┌──────┴──────┐
//              ↓             ↓
//           &T / &mut T    SLICES
//                            │
//                            ↓
//                        ITERATORS
//                            │
//                            ↓
//                        CLOSURES
//
//              ┌─────────────────────┐
//              │                     │
//           GENERICS              TRAITS
//              │                     │
//              └──────────┬──────────┘
//                         ↓
//                    ABSTRACTION
//
//              ┌─────────────────────┐
//              │                     │
//           OPTION                RESULT
//              │                     │
//          maybe value         success/failure
//
