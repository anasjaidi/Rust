struct  Car;

struct  Animal;


trait Voice {
    fn make_sound(&self, sound: &str);
}

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

////////////////////////////////////////////////////////////
trait Shipping {
    fn ship_package(&self) -> i32{
        12
    }
}

struct BigPackage;

struct  SmallPackage;

impl Shipping for SmallPackage {}

impl Shipping for BigPackage {
    fn ship_package(&self) -> i32 {
        12345
    }
}


//////////////////////////////////////////

struct PackageA {
    wieght: i32,
}

impl PackageA {
    fn new() -> Self {
        Self {wieght: 233}
    }
}
impl Default for PackageA {
    fn default() -> Self {
        Self { wieght: 455 }
    }
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

    /////////////////////////
    let s = SmallPackage;
    let b = BigPackage;

    println!("{}", s.ship_package());
    println!("{}", b.ship_package());

    let pa = PackageA::default();

    let pa_2 = PackageA::new();

    println!("{}", pa.wieght);
    println!("{}", pa_2.wieght);

}