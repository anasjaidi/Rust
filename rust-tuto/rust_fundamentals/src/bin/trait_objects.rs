trait Clicky {
    fn click(&self) -> i32;
}

struct Mouse {
    speed: i32,
}

impl Clicky for Mouse {
    fn click(&self) -> i32 {
        println!("clockcc");
        return self.speed;
    }
}

#[derive(Debug)]
struct Keyboard {
    num: i32,
}

impl Clicky for Keyboard {
    fn click(&self) -> i32 {
        println!("clicked");
        self.num
    }
}

fn static_triart(object: &impl Clicky) {
    (*object).click();
}

fn dyn_traits(object: &Box<&dyn Clicky>) {
    let a = (*object).click();

    println!("{a}")
}

fn dyn_vec_trait(v: &Vec<Box<dyn Clicky>>) {
    for one in v {
        println!("{}", one.click())
    }
}

fn main() {
    let key_a = Keyboard { num: 12 };
    static_triart(&key_a);
    let key_b = Box::new(Keyboard { num: 12 });
    dyn_traits(&Box::new(&*key_b));

    dyn_traits(&Box::new(&Mouse { speed: 112 }));
    dbg!(key_b);

    dyn_vec_trait(&vec![Box::new(Mouse {speed: 112}), Box::new(Keyboard {num:  1222})]);
}
