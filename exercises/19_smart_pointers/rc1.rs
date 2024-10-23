// In this exercise, we want to express the concept of multiple owners via the
// `Rc<T>` type. This is a model of our solar system - there is a `Sun` type and
// multiple `Planet`s. The planets take ownership of the sun, indicating that
// they revolve around the sun.

use std::rc::Rc;

#[derive(Debug)]
struct Sun;


#[allow(dead_code)]
#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>), // 引用计数, 传进去的
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {self:?}!");
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc1() {
        // 对于每个行星不是分别new一个sun; 而是共享一个sun的内容, 所以用到引用计数
        let sun = Rc::new(Sun); // 创建太阳, 引用计数的智能指针
        println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference ? 为什么一开始引用就是1, 而不是0

        let mercury = Planet::Mercury(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 2 references
        mercury.details();

        let venus = Planet::Venus(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 3 references
        venus.details();

        let earth = Planet::Earth(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 4 references
        earth.details();

        let mars = Planet::Mars(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 5 references
        mars.details();

        let jupiter = Planet::Jupiter(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 6 references
        jupiter.details();

        let saturn = Planet::Saturn(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 7 references
        saturn.details();

        let uranus = Planet::Uranus(Rc::clone(&sun)); // 如果重新new的话引用计数无关的, 新开的
        println!("reference count = {}", Rc::strong_count(&sun)); // 8 references
        uranus.details();

        let neptune = Planet::Neptune(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 9 references
        neptune.details();

        assert_eq!(Rc::strong_count(&sun), 9);

        /* 当引用者释放了, 计数会自动减少! */
        drop(neptune);
        println!("reference count = {}", Rc::strong_count(&sun)); // 8 references

        drop(uranus);
        println!("reference count = {}", Rc::strong_count(&sun)); // 7 references

        drop(saturn);
        println!("reference count = {}", Rc::strong_count(&sun)); // 6 references

        drop(jupiter);
        println!("reference count = {}", Rc::strong_count(&sun)); // 5 references

        drop(mars);
        println!("reference count = {}", Rc::strong_count(&sun)); // 4 references

        drop(earth);
        println!("reference count = {}", Rc::strong_count(&sun)); // 3 references

        drop(mercury);
        println!("reference count = {}", Rc::strong_count(&sun)); // 2 references

        drop(venus);
        println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

        assert_eq!(Rc::strong_count(&sun), 1);
    }
}
