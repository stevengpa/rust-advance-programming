// << Associated Types >>
trait Printable {
    type Output;
    fn print(&self) -> Self::Output;
}
struct Circle;
impl Printable for Circle {
    type Output = String;
    fn print(&self) -> String {
        String::from("Circle")
    }
}

// << Generic Traits >>
trait Add<RHS = Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}
impl Add for i32 {
    type Output = i32;
    fn add(self, rhs: Self) -> i32 {
        self + rhs
    }
}