//structs define the variables of our "Objects"
pub struct Dog {

    //variables used to define instances of dog
    pub breed: String,      //pub keyword makes this a public variable (default is private)
    lifespan: i32,
    activity: String,

}

//impl (implements) gives us more functionality to handle Dog
impl Dog {

    fn new(breed: String, lifespan: i32, activity: String) -> Dog {
        Dog {breed, lifespan, activity}
    }

    fn get_breed(&self) -> String {
        self.breed.clone()
    }
    fn get_lifespan(&self) -> i32 {
        self.lifespan
    }
    fn get_activity(&self) -> String {
        self.activity.clone()
    }

}

fn main() {
    let goodboy = Dog::new("Golden Retriever".to_string(), 15, "Highly Active".to_string()); 
    println!("Dog Breed: {}\nLifespan: {} years\nActivity Level: {}", goodboy.get_breed(), goodboy.get_lifespan(), goodboy.get_activity());
}
