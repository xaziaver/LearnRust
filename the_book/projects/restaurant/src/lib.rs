/* this project with just the below code has the following 'module tree'
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
the 'crate root' is the root module of this structure and made from lib.rs
If module A is contained in B, then A is a 'child' and B is a 'parent'

*/ 

mod front_of_house {
    
    // making the hosting mod public allows parent modules to refer to it
    pub mod hosting {
        
        // if we also want the contents of the public mod to be available, 
        // we must explicitly make those public as well
        pub fn add_to_waitlist() {}
        // these privacy rules apply to structs, enums, functions 
        // and other modules defined within the module

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// since this function & front_of_house are both defined in the same module
// (i.e. both siblings in the `crate` root module)
// front_of_house can be referred to here without needing to make it public
pub fn eat_at_restaurant() {
    // absolute path, start from root
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path, start from front_of_house
    front_of_house::hosting::add_to_waitlist();
    // defined in the same module so path still valid
}