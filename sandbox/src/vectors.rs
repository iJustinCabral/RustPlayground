pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4];
    
    // Reassign a value
    numbers[2]= 20;
    
    // Add on to vector
    numbers.push(25);
    numbers.push(86);

    // Pop off last value
    numbers.pop();

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number:{}", x);
    }

    // Loop and mutuate
    for x in numbers.iter_mut() {
        *x *= 2;
    }

   println!("{:?}", numbers); 
   println!("Numbers Vec: {:?}", &numbers);

}
