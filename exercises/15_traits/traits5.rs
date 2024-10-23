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

// 两个结构体同时实现两个Trait
pub struct SomeStruct;
impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

pub struct OtherStruct;
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// impl: Trait1 + Trait2 (组合)
pub fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
    item.some_function() && item.other_function()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_func() {
        assert!(some_func(SomeStruct));
        assert!(some_func(OtherStruct));
    }
}
