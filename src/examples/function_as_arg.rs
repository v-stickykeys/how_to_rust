/// You can run the tests in this file by typing
/// ```
///   cargo test cat_makes_a_sound
/// ```
trait Animal {
    fn sound(self, func: fn(volume: usize) -> String) -> String;
}

struct Cat {
    volume: usize
}

impl Animal for Cat {
    fn sound(self, func: fn(volume: usize) -> String) -> String {
        func(self.volume)
    }
}

pub fn run() {
    let cat = Cat { volume: 3 };
    fn meow(volume: usize) -> String {
        "\nmeow".repeat(volume)
    }
    let sound = cat.sound(meow);
    println!("This example passed a meow function as an argument.{}", sound);
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn cat_makes_a_sound() {
        let cat = Cat { volume: 3 };
        fn meow(volume: usize) -> String {
            "meow".repeat(volume)
        }
        let sound = cat.sound(meow);
        assert_eq!(sound, "meowmeowmeow");
    }
}
