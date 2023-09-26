use std::fmt::Debug;

trait Location {
    fn loc(&self);
}

#[derive(Debug)]
enum AirLocation {
    A,
    B,
    C,
}

enum OcceanLocation {
    W,
    V,
    Y,
}

impl Location for AirLocation {
    fn loc(&self) {
        println!("loc 1")
    }
}

impl Location for OcceanLocation {
    fn loc(&self) {
        println!("loc 2")
    }
}

struct LocatorA<T: Location + Debug, > {
    object: T
}

struct LocatorB<T> where T: Location{
    object: T,
}


impl<T: Location + Debug + Clone> LocatorA<T> {
    fn a() {
        
    }
}

impl LocatorA<AirLocation> {
    
}

fn main() {
    
}