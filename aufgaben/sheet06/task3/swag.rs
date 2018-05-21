use std::fmt;

fn main() {
    let x = Swagger(3);
    println!("{}", x);
    println!("{}", 3.with_swag());
}

struct Swagger<T>(T);

impl<T: fmt::Display> fmt::Display for Swagger<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "yolo {} swag", self.0)
    }
}

trait SwaggerExt: Sized {
    fn with_swag(self) -> Swagger<Self> {
        Swagger(self)
    }
}

impl<T> SwaggerExt for T {}
