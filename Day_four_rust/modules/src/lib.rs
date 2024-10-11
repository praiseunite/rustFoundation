pub mod some {
    //defining the functions in the testing_module module

    // If you do not add the pub keyword before the function, the function will not be accessible outside the module.
    pub fn adding_numbers() {
        println!("examine members");
    }
    pub fn remove_numbers() {
        println!("test members");
    }

    pub mod special {  
        pub fn numbers(){  
            println!("special members");
        }
    }
}
