// Your task is to replace the '??' sections so the code compiles.
// Don't change any line other than the marked one.

// I AM NOT DONE

pub trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

pub trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct {
    name: String,
}

impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn some_func(item: ??) -> bool {
    item.some_function() && item.other_function()
}

fn main() {}
