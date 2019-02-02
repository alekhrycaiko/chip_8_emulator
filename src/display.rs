pub struct Display { 


}
impl Display { 
    
    pub fn new() -> Display { 
        println!("Made a new display");
        return Display { 

        }
    }
    pub fn clear(&self) { 
        println!("Hit clear");
    }
}
