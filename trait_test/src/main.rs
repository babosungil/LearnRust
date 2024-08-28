
pub trait PrintName {
    fn print(&self);
}

pub struct MyObject {
    pub components: Vec<Box<dyn PrintName>>,
}

impl MyObject {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.print();
        }
    }
}

pub struct MyObject_One {
    pub level : i32,
}

impl PrintName for MyObject_One {
    fn print(&self) {
        println!("MyObject_One");
    }
}

pub struct MyObject_Two {
    pub exp : i32,
}

impl PrintName for MyObject_Two {
    fn print(&self) {
        println!("MyObject_Two")
    }
}


fn main() {
    let objects = MyObject {
        components: vec![
            Box::new(MyObject_One {
                level: 123,
            }),
            Box::new(MyObject_Two {
                exp: 987,
            }),
        ],
    };

    objects.run();
}
