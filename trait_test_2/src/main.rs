
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

    loop {
        println!("looooooop");
    }


}
