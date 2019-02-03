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
    /**
     * Overwrites the sprite onto the current size.
     * Returns true if the overwrite was successful.
     * Returns false if the write was successful, but, was not an overwrite.
     */
    pub fn overwrite_sprite(&self, sprite_bytes: &Vec<u8>, x: &u8, y: &u8) -> bool { 
        println!("TODO {} {}", x, y);
        return true;
    }
}
