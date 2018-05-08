///////////////
// 1. Basics //
///////////////

// Functions. `i32` is the type for 32-bit signed integers
fn add2(x:i32, y:i32) -> i32 {
    x + y
}


// Main function
fn main() {
    // Numbers

    // Immutable binding
    let x: i32 = 1;
    // x = 3; <-- compile-time error

    // Mutable variable
    let mut mutable = 1;
    mutable = 4;
    mutable += 2;

    // Integer/float suffixes
    let y: i32 = 13i32;
    let f: f64 = 1.3f64;

    // Type inference
    //
    // Most of the time, the Rust compiler can infer what type a variable is, so
    // you don't have to write an explicit type annotation. Throughout this
    // tutorial, types are explicitly annotated in many places for demonstrative
    // purposes. Type inference can handle this for you most of the time.
    let implicit_x = 1;
    let implicit_f = 1.3;

    // Arithmetic
    let sum = x + y + 13;

    // Strings //

    // String literals
    let x: &str = "hello world!";

    // Printing
    println!("{} {}", f, x); // 1.3 hello world
    
     // A `String` - a heap-allocated string
    let s: String = "hello world".into();
    let s2: String = "hello world".to_string();
    let s3: String = String::from("hello world");

    // A string slice: an immutable view into another string.
    //
    // This is essentially an immutable pair of pointers to a string - it
    // doesn't actually contain the contents of a string, just a pointer to the
    // begin and a pointer to the end of a string buffer, statically allocated
    // or contained in another object (in this case, `s`)
    let s_slice: &str = &s;
    let s_slice2: &str = &s[6..11];
    let s_slice3: &str = &s[6..];
    let s_slice4: &str = &s[..5];

    println!("{} {}", s, s_slice); // hello world hello world

    // Vectors/arrays //
    
    // A fixed-size arrays
    let for_ints: [i32; 4] = [1, 2, 3, 4];

    // A dynamic array (vector)
    let mut vector: Vec<i32> = vec![1, 2, 3, 4];
    vector.push(5);

    // Mutability is inherited by the bound value/ if 'vector' is not declared
    // 'mut', then the value cannnot be mutated.
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    // vector.push(5); <-- compile-time error

    // A slice - an immutable view into a vector or array
    let slice:&[i32] = &vector;
    let slice2: &[i32] = &vector[1..4];

    // Use `{:?}` to print something debug-style
    println!("{:?} | {:?}", vector, slice2); // [1, 2, 3, 4, 5] | [2, 3, 4]

    // Array, slice and vector indexing
    println!("{}", for_ints[1]); // 2
    println!("{}", vector[2]); // 3
    println!("{}", slice[3]); // 4
}

