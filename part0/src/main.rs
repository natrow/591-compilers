mod module_1; // see module_1/mod.rs

// Note: Rust modules can be placed in either (module name).rs or (module name)/mod.rs

fn main() {
    println!("part0 - Nathan Rowan and Trevin Vaughan");
    module_1::func1();
    module_1::func2();
}
