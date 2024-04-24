//structs define the variables of our "Objects"
struct Dog {

    //variables used to define instances of dog
    pub breed: String,      //pub keyword makes this a public variable (default is private)
    pub lifespan: i32,
    pub activity: String,

}

//impl (implements) gives us more functionality to handle Dog
impl Dog {

    fn new(breed: String, lifespan: i32, activity: String) -> Dog {
        Dog {breed, lifespan, activity}
    }

    fn get_breed(&self) -> String {
        self.breed.clone()      //use clone so it can be reimplemented
    }
    fn get_lifespan(&self) -> i32 {
        self.lifespan
    }
    fn get_activity(&self) -> String {
        self.activity.clone()
    }

}
