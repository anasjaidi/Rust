trait Voice {
    fn make_sound(&self, sound: &str);
}

struct  Car;

struct  Animal;


impl Voice for Car {
    fn make_sound(&self, sound: &str) {
        println!("{}", sound);
    }
}

impl Voice for Animal {
    fn make_sound(&self, sound: &str) {
        println!("{}", sound);
    }
}

fn sound(obj: &impl Voice, sound: &str) {
    obj.make_sound(sound);
}

fn main() {
    let a = Animal {};
    let c = Car;

    let a_sound = "booh".to_owned();

    let c_sound = "tiit".to_owned();

    a.make_sound(&a_sound);
    c.make_sound(&c_sound);

    sound(&a, &c_sound);
    sound(&a, &a_sound);
}