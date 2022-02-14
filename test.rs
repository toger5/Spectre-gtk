use std::*;
struct A{
    val: Option<i32>
}
impl fmt::Debug for A {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
         .field(&self.val)
         .finish()
    }
}
fn main(){
    let a = Some(A{val: Some(10)});
    let b = Option::<i32>::None;
    println!("{:?}", a);
    println!("{:?}", b);
    if {a?.val?}.unwrap_or(0) == b.unwrap(){
        print!("sth")
    }
}