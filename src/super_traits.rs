trait Bounded: Default {
    fn get_max() -> Self;
    fn get_min() -> Self;
}

enum Size {
    Small,
    Medium,
    Large,
}

impl Bounded for Size {
    fn get_max() -> Self {
        Self::Large
    }
    fn get_min() -> Self {
        Self::Small
    }
}
impl Default for Size {
    fn default() -> Self {
        Self::Medium
    }
}

fn get_size_num(size: &Size) -> u8 {
    match size {
        Size::Small => 0,
        Size::Medium => 1,
        Size::Large => 2,
    }
}

pub fn run_example() {
    let my_size = Size::Large;
    let min_size_num = get_size_num(&Size::get_min());
    let default_size_num = get_size_num(&Size::default());
    let my_size_num = get_size_num(&my_size);
    if my_size_num == min_size_num {
        println!("I have the shortest size!");
    }
    if my_size_num == default_size_num {
        println!("Default size suits me!")
    }
}
