// Arrays - Fixed list where elements are the same data types (Can't add on, can change values)

use std::mem;

pub fn run() {
    // Expects 5 elements if we declate it as such, e.g. cannot have just 4 elements
    let mut numbers: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", numbers);
    
    // Re-assign value
    numbers[0] = 9;

    // Get single value
    println!("Single Value: {}", numbers[0]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes in memory",mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);

}
