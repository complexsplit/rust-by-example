// Null enumerations to define unit types
#[deriving(Show)]
enum Inch {}
#[deriving(Show)]
enum Mm {}

// Length is phantom type with hidden parameter `Unit`
#[deriving(Show)]
struct Length<Unit, T>(T,);

// `impl X for Y {}` reads "implement `X` Trait for Type `Y`"
// So, this implements the `Add` Trait for Type `Length`
// The `Add` Trait overloads the addition operator
// so elements can be added together.
// `X: Y` applies a restriction to `X` and only allows operations
// to `X` if `X` implements the Trait `Y`.
// This means that this `impl` defines `Add` only for `T` when
// two `T's` can be added together and the result is of
// Type `T`: (`T: Add<T,T>`)
impl <Unit,T: Add<T,T>> Add<Length<Unit, T>,
                            Length<Unit, T>> for Length<Unit, T> {
    fn add(&self, r: &Length<Unit, T>) -> Length<Unit, T> {
        let &Length(ref left)  = self;
        let &Length(ref right) = r;
        
        Length(*left + *right)
    }
}

fn main() {
    // Specialize one_foot to have hidden parameter `Inch`
    let one_foot:  Length<Inch, f32> = Length(12.0);
    // one_meter has hidden parameter `Mm`
    let one_meter: Length<Mm, f32>   = Length(1000.0);
    
    // Addition works
    println!("one foot + one_foot = {}", one_foot + one_foot);
    println!("one meter + one_meter = {}", one_meter + one_meter);

    // Nonsensical operations fail as they should
    // Error: type mismatch
    //let one_feter = one_foot + one_meter;
}

