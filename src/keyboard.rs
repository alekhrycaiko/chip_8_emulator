pub struct Keyboard { 


}
impl Keyboard { 
    // TODO: lets see if we need this; if not ,remove
    pub fn new() -> Keyboard { 
        return Keyboard { 

        }
    }
    
    /**
     * Blocking call that waits for a keyboard input before returning.
     */
    pub fn block_for_input(&self) -> u8 { 
        // TODO
        return 1;
    }
    
    /**
    * Returns true if the provided key is pressed.
    */
    pub fn is_key_pressed(&self, key: u8) -> bool { 
        // TODO
        println!("key is {:#}", key);
        return true;
    }
    //TODO: any key is pressed function?
}
