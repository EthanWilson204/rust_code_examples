//imperative style is typed in a way where instructions on how to do something are explicit

fn main() {

    //mut keyword lets us change the variables of the array
    let mut example_array: [i32; 5] = [10, 20, 30, 40, 50];
    println!("Before multiplication: {:?}\n", example_array);

    //change the array, and print the change in state to the user
    for index in 0..example_array.len() {

        example_array[index] *= 2;
        println!("Iteration: {}\nArray: {:?}\n", index ,example_array);

    }

}