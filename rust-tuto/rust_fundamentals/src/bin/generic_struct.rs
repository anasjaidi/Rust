///////////////////////////
/// syntax
struct Generic<T: Copy> {
    obj1: T,
}

struct Generic2<T>
where
    T: Copy + Clone,
{
    obj: T,
}

//////////////////
/// setup

trait Location {
    fn loc(&self);
}

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

///////////

struct LocatorA<T: Location> {
    object: T
}

struct LocatorB<T> where T: Location{
    object: T,
}

fn get_location(locator: LocatorA<AirLocation>) {
    locator.object.loc();
}

fn get_all<T>(locator: LocatorA<T>)where T: Location {
    locator.object.loc();
}

fn main() {}
