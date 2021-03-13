use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox");
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    let x = 5;
    {
        // copies x onto heap
        let y = Box::new(x);
        assert_eq!(5, *y);
    }
    assert_eq!(5, x);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let z = MyBox::new("test");
    drop(z); // Drop early

    let s1 = "test";
    let s2 = String::from("test");
    assert_eq!(*s1, *s2);
}
