
type MyType = i32;

struct Vector2 {
    x: MyType,
    y: i32,
}

trait MyAdd<Rhs=Self> {
    type  Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

impl MyAdd for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct MyType1(i32);
struct MyType2(i32);

impl MyAdd<MyType2> for MyType1 {
    type Output = MyType1;

    fn add(self, other: MyType2) -> MyType1 {
        MyType1(self.0 + other.0 * 100)
    }
}


fn func_test_1(a: i32, b: i32) -> i32 {
    a + b
}

fn func_get_closure() -> Box<dyn Fn(i32, i32) -> i32> {
    Box::new(|a, b| a + b)
}

fn func_test_2(f: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    f(a, b)
}


fn main() {
    let vec1 = Vector2{ x: 1, y: 2 };
    let vec2 = Vector2{ x: 3, y: 4 };
    let vec3 = vec1.add(vec2);
    println!("{}, {}", vec3.x, vec3.y);


    let my_type_1: MyType1 = MyType1(1);
    let my_type_2 = MyType2(2);
    let add_result = my_type_1.add(my_type_2);
    println!("add_result : {}", add_result.0);

    
    let my_type_3 = MyType2(2);
    let empty_type: i32 = match my_type_3 {
        MyType2(2) => 111,
        MyType2(3) => panic!("empty_type"),     // empty type
        _ => 0
    };
    println!("empty_type : {:?}", empty_type);


    let test_1 = func_test_1(2, 3);
    println!("func_Test_1 : {test_1}");
    let test_2 = func_test_2(func_test_1, 3, 5);
    println!("func_Test_2 : {test_2}");

    let func_closure = func_get_closure();    
    let test_3 = func_closure(5, 7);
    println!("func_Test_3 : {test_3}");
    
}
