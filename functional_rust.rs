//functions allow us to offset functions and instructions to built in parts of the code
//in this instance we will use iterators and closures to demonstrate functinal programming

fn main() {

    //non-functional way of increasing the numbers of each value
    let vec1 = vec![10, 20, 30, 40, 50];
    let mut vec2: Vec<i32> = Vec::new();

    for &num in vec1.iter() {
        vec2.push(num + 15);
    }
    println!("non functional: {:?}", vec2);

    //functional way of increasing the numbers of each value
    let vec3 = vec![10, 20, 30, 40, 50];
    let vec4: Vec<i32> = vec3.iter()
            .map(|vec3_num| vec3_num + 15)
            .collect();

    println!("functional: {:?}", vec4);

}