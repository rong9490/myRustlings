// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the "cons list", a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: The value of the current item and
// the next item. The last item is a value called `Nil`.

// 链表结构
// TODO: Use a `Box` in the enum definition to make the code compile.
// 栈内存一定需要确定占用大小, 递归无法确定, 所以需要用Box放到堆内存中;
#[derive(PartialEq, Debug)]
enum List { // 确认的是 栈内存的大小, 因为内存是紧密排列的!指针是一个挨着一个的;
    Cons(i32, Box<List>), // 递归, 为什么需要用Box抱起来, 因为无法确认占用内存大小! Box使用的堆内存, 显式移到堆内存上
    Nil,
}

// TODO: Create an empty cons list.
fn create_empty_list() -> List {
    List::Nil
}

// TODO: Create a non-empty cons list.
fn create_non_empty_list() -> List {
    List::Cons(42, Box::new(List::Nil)) // 栈上指针指向堆内存
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
