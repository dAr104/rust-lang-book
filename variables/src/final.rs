fn main() {
    // Scalar data types

    // Integers
    let a = 98_222; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte (u8 only)

    // Floating-point numbers
    let f = 2.0;
    let g: f32 = 3.0;
    // Booleans
    // Characters

    // Compound data types

    // Tuples
    let tup = ("Let's Get Rusty", 100_000) // fixed array of different types
    let (title, count) = tup;
    let sub_count = tup.1;
    // Arrays
    let error_codes = [404, 500, 200, 401, 403]; // fixed array, for dynamic use Vector
    let not_found = error_codes[0];

    let byte = [0; 5]; // [0, 0, 0, 0, 0]
}