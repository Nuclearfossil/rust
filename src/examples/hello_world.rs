// This is a standard, one line comment
/* This
   is a
   block comment
 */

/*
   Block comments can start and end anywhere */


/// This allows us to generate library documents for the following item
/// However, there is nothing currently below, so we don't get comments

//
// Reference Material:
//   - https://doc.rust-lang.org/stable/rust-by-example
//   = https://doc.rust-lang.org/book/second-edition/foreword.html
//

// The main entry point - we've seen this a hundred times before
fn main() 
{
    // This is a macro. This is not a function, but an actual macro (the ! at the end of the name
    // indicates that you are calling a macro).
    println!("Hello World");

    // Some additional print macros
    eprint!("Whoops, this is going out to the standard error channel (io::stderr)\n");
    eprintln!("Another bit of text going to the stderr, but without having to add a newline!");

    println!("{} is the answer to the {}.", 42, "Question");

    println!("{0} can repeat whatever {0} would {1} to {2} on {2}{3}", "You", "like", "repeat", ".");

    println!("{subject} {verb} {object}.",
        object="the lazy dog",
        verb="jumps over",
        subject="The quick brown fox");

    println!("012345678901234567890");
    println!("{number:>width$}", number=1, width=6);
    println!("{number:>0width$}", number=2, width=6);

    // in the next section, we invoke two separate methods
    println!("The square of 5 is {}", calc_square(5));
    println!("The negation of 5 is {}", calc_negation(5));

    primitives();
    literals();
    bitwise_ops();
    arrays();
}

/// Our first function. We see a data type used here (i32 for 32 bit integer).
/// We also see how a return is defined in the function signature
fn calc_square(value : i32) -> i32
{
    return value * value;
}

/// A more interesting example, utilizing the 'expression' language feature.
fn calc_negation(value : i32) -> i32
{
    if value == 0
    {
        return 0;
    }

    // this is considered an expression - without the `;` and it being the last line of
    // a function, return is assumed.
    value * -1
}

/// example without arguments or return types
fn primitives()
{
    let _is_on: bool = false;

    let _int8bit_a: i8 = 0;       // variable type declaration syntax
    let _int8bit_b = 0i8;         // variable type inferred by suffix - only works on numerical types
    let _int16bit_a: i16 = 0;
    let _int16bit_b = 0i16;
    let _int32bit_a: i32 = 0;
    let _int32bit_b = 0i32;
    let _int64bit_a: i64 = 0;
    let _int64bit_b = 0i64;

    let _uint8bit: u8 = 0;
    let _uint16bit: u16 = 0;
    let _uint32bit: u32 = 0;
    let _uint64bit: u64 = 0;

    let _float32: f32 = 0.0;
    let _float64: f64 = 0.0;

    let _letter: char = ' ';                     // character declaration - 1 char == 4 bytes
    let _anotherletter = 'a';                    // infering a character type

    // Strings are a bit of a different beast. They're actually not strings, but 'string slice'
    // Think 'String Literal'
    let _anotherstring  = "Here's a string";     // infering a string type
    let _firststring: &str = "First string";     // string declaration - note the `&`
}

fn literals()
{
    if 1000 == 1_000
    {
        println!("1000 is equal to 1_000");
    }
    else
    {
        println!("1000 is NOT EQUAL to 1_000");
    }

    if 16 == 0x10
    {
        println!("16 is equal to 0x10");
    }
    else
    {
        println!("16 is NOT EQUAL to 0x10");
    }

    if 8 == 0o10
    {
        println!("8 is equal to 0o10");
    }
    else
    {
        println!("8 is NOT EQUAL to 0o10");
    }

    if 6 == 0b00000101
    {
        println!("5 is equal to 0b00000101");
    }
    else
    {
        println!("5 is NOT EQUAL to 0b00000101");
    }
}

fn bitwise_ops()
{
    println!("0011 AND 1100 = {:08b}", 0b0011u8 & 0b1100u8);
    println!("0011 OR 1100 = {:08b}", 0b0011u8 | 0b1100u8);
    println!("1111 XOR 1100 = {:08b}", 0b1111u8 & 0b1100u8);

    println!("1 << 5 = {:08b}", 1u8 << 5);

    println!("{number:>width$}", number=1, width=6);
    println!("0x80      is 0b{:016b}", 0x80u32);
    println!("0x80 >> 2 is 0b{:016b}", 0x80u32 >> 2);
}

fn arrays()
{
    // declare a fixed-size array of floats
    let vector2 : [f32; 2] = [0.0, 0.0];
    let vector3 : [f32; 3] = [0.0, 0.0, 0.0];
    let vector4 : [f32; 4] = [0.0, 1.0, 2.0, 3.0];  // initialize all elements explicitly
    let _array4: [f32; 4] = [0.0; 4];               // initialize 0.0 to the first 4 elements

    println!("The size of vector2 is {}", vector2.len());
    println!("The size of vector3 is {}", vector3.len());
    println!("The size of vector4 is {}", vector4.len());

    // in the next case, the & represents us 'borrowing' a sub-section of an array
    // into another array
    let slice = &vector4[1 .. 3];

    println!("The length of slice is {}", slice.len());

    for element in slice
    {
        println!("The element is {}", element);
    }
}
