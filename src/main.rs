mod deref;
mod generics;
mod generics_lifetime;
mod rc_smart_pointers;
mod structs_elision;
mod super_traits;
mod traits;

fn main() {
    generics::run_example();
    traits::run_example();
    super_traits::run_example();
    generics_lifetime::run_example();
    structs_elision::run_example();
    rc_smart_pointers::run_example();
    deref::run_example();
}
