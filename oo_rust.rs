//structs define the variables of our "Objects"
pub struct Dog {

    //variables used to define instances of dog
    pub breed: String,      //pub keyword makes this a public variable (default is private)
    lifespan: i32,
    toy: String,

}

//impl (implements) gives us more functionality to handle Dog
impl Dog {

    fn new(breed: String, lifespan: i32, toy: String) -> Dog {
        Dog {breed, lifespan, toy}
    }

    fn get_breed(&self) -> String {
        self.breed.clone()
    }
    fn get_lifespan(&self) -> i32 {
        self.lifespan
    }
    fn get_toy(&self) -> String {
        self.toy.clone()
    }

}

fn main() {
    let goodboy = Dog::new("Husky".to_string(), 14, "Tennis Ball".to_string()); 
    println!("Dog Breed: {}\nLifespan: {} years\nActivity Level: {}", goodboy.get_breed(), goodboy.get_lifespan(), goodboy.get_toy());
}
