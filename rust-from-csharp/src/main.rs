use sample_mod::sample_mod_fn;

mod other_mod;
mod sub_mod;

use other_mod::OtherModType;
use sub_mod::sub_mod_func;

fn main() {
    let name: String = String::from("little fellow");

    let mut number: i32 = 1;

    let text: &str = "Hello";

    let is_true: bool = true;

    number = double(number);

    print_num(&number);

    number = number + 2;

    if (is_true) {
    println!("{}, {} {}!", text, name, number);
    }
    else {
        println!("Goodbye!");
    }

    let result: i32 = double(2);

    println!("My numbre is {}!", result);

    let nresult: Option<i32> = half(4);
    match nresult {
        Some(x) if x >= 10 => println!("Big number!"),
        Some (x) => println!("Result: {}", x),
        _ => println!("Failed to half")
    }

    let numbro: i32 = 123;
    let numbre: u128 = 8374017843123981823;
    let a: [i64; 5] = [1,2,3,4,5];

    let p: (&str, i32, bool) = ("John", 33, true);

    println!("{}, {}, {}", numbro, numbre, a[0]);

    let person = Person::create("Bob", 33);

    print_beautifully(&person);

    sample_mod_fn();

    sub_mod_func();

}

fn print_num(num: &i32)  {
    println!("{}", num);
}

mod sample_mod {
    pub fn sample_mod_fn() {}
}

    struct Person {
        name: String, 
        age: i32,
        likes_ice_coffee: bool,
    }

   impl Person {
    fn create (name: &str, age: i32) -> Self {
        Self {
            name: String::from(name),
            age,
            likes_ice_coffee: true,
        }
    }

    fn can_have_ice_coffee(&self) -> bool {
        self.age > 10
    }
   } 

   trait Beautifully {
    fn make_beautiful(&self) -> String;
   }

   impl Beautifully for Person {
    fn make_beautiful(&self) -> String {
        format!("Beautiful {}", self.name)
    }
   }

   fn print_beautifully(b: &impl Beautifully) {
    println!("Hello {}", b.make_beautiful());
   }

impl Beautifully for String {
    fn make_beautiful(&self) -> String {
        String::from(self)
    }
}

fn double(num: i32) -> i32{
    return num * 2;
}

fn half(num: i32) -> Option<i32> {
    if num == 0 {
        return None;
    }

    Some (num / 2)
}

#[test]
fn test_case() {
    assert_eq!(2 + 2, 4);
}
