// rc1.rs
//
// 在这个练习里，我们想用 Rc<T> 类型来表达"多个所有者"的概念。这是一个太阳系的模型——
// 有一个 Sun（太阳）类型，以及多颗 Planet（行星）。行星"拥有"太阳的所有权，表示它们
// 绕着太阳转。
//
// 用恰当的 Rc 原语，让这段代码编译通过，从而表达"太阳有多个所有者"。
//
// 执行 `rustlings hint rc1` 或使用 `hint` watch 子命令来获取提示。


use std::rc::Rc;

#[derive(Debug)]
struct Sun {}

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
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
        println!("Hi from {:?}!", self)
    }
}

fn main() {
    let sun = Rc::new(Sun {});
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 个引用

    let mercury = Planet::Mercury(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 个引用
    mercury.details();

    let venus = Planet::Venus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 个引用
    venus.details();

    let earth = Planet::Earth(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 个引用
    earth.details();

    let mars = Planet::Mars(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 个引用
    mars.details();

    let jupiter = Planet::Jupiter(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 个引用
    jupiter.details();

    // TODO
    let saturn = Planet::Saturn(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 个引用
    saturn.details();

    // TODO
    let uranus = Planet::Uranus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 个引用
    uranus.details();

    // TODO
    let neptune = Planet::Neptune(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 9 个引用
    neptune.details();

    assert_eq!(Rc::strong_count(&sun), 9);

    drop(neptune);
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 个引用

    drop(uranus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 个引用

    drop(saturn);
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 个引用

    drop(jupiter);
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 个引用

    drop(mars);
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 个引用

    // TODO
	drop(earth);
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 个引用

    // TODO
	drop(venus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 个引用

    // TODO
	drop(mercury);
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 个引用

    assert_eq!(Rc::strong_count(&sun), 1);
}
