mod generics;
mod generics_lifetime;
mod super_traits;
mod traits;

fn main() {
    generics::run_example();
    traits::run_example();
    super_traits::run_example();
    generics_lifetime::run_example();
}
