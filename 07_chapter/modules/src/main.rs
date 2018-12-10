//Remember : A package can contain zero or one library crates and
//as many binary crates as you’d like.
//There must be at least one crate (either a library or a binary) in a package.

//Remember : If a package contains both src/main.rs and src/lib.rs,
//then it has two crates: a library and a binary, both with the same name.

//Remember : A package can have multiple binary crates
//by placing files in the src/bin directory: each file will be a separate binary crate.

//Remember : All the items in the module are private by default.
//Private childern of the current module can not be accessed.
//You are allowed to use any code defined in ancestor modules or the current module.
//In other words,
//items without the pub keyword are private as you look “down” the module tree
//from the current module, but items without the pub keyword are public
//as you look “up” the tree from the current module.

//Eamples
mod sound {
    //Note : pub makes instrument module public.
    pub mod instrument {
        //Note : pub makes fn clarinet() public.
        pub fn clarinet() {
            println!("Hello, Module!");
        }
    }
}

mod hello;

//Note : as sound is in the same module as of main,
//absolute path should start with crete keyword.
use self::hello::wonderful::world;
use crate::sound::instrument;
fn main() {
    //Remember : Because clarinet is defined within the same crate as main,
    //we use the crate keyword to start an absolute path.
    println!("Use of absolute path to access a function defined in a module.");
    crate::sound::instrument::clarinet();

    println!("Use of relative path to access a function defined in a module.");
    sound::instrument::clarinet();

    println!("Use of use keyword to bring the scope of a module.");
    instrument::clarinet();

    //calling fn of module defined outside the current file.
    //with absolute path
    println!("\nCalling fn of module defined outside the current file.");
    crate::hello::wonderful::world::message();
    //with relative path
    hello::wonderful::world::message();
    //with bringing the scope
    world::message();
}
