struct V2D {
    x: i32,
    y: i32
}

impl V2D {
    fn new(x: i32, y: i32) -> V2D {
        V2D { x, y }
    }
}

trait Trait1 {
    fn print_it();
    fn write_it(&self);
}

trait Trait2 {
    fn print_it() {
        println!("This is Trait2");
    }
    fn write_it(&self);
}

impl Trait1 for V2D {
    fn print_it() {
        println!("This is Trait1");
    }

    fn write_it(&self) {
        println!("writin' it from Trait1: x: {}, y: {}", self.x, self.y);
    }
}

impl Trait2 for V2D {
    fn write_it(&self) {
        println!("writin' it from Trait2: y: {}, x: {}", self.y, self.x);
    }

    fn print_it() {
        println!("This is Trait2, but overwritten");
    }
}

fn main() {
    let v = V2D::new(1,2);
    // Trait1::print_it(v);

    // disambiguate call on associated fn on trait with name collusion
    <V2D as Trait1>::print_it();
    <V2D as Trait2>::print_it();


    // disambiguate call on methods on trait with name collusion
    <V2D as Trait1>::write_it(&v);
    <V2D as Trait2>::write_it(&v);

    // Trait1::write_it(&v);
    // Trait2::write_it(&v);
}