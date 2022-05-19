use doop::{dooped, doop};

struct S1;
struct S2;
struct S3;

trait T1{
    fn t1() {}
}
trait T2{
    fn t2() {}
}
trait T3{
    fn t3() {}
}

#[dooped(for (Struct, Trait) in [(S1, T1), (S2, T2), (S3, T3)])]
impl Trait for Struct {}

fn main() {
    doop! {
        for Struct in [S1, S2, S3]
        for method in [t1, t2, t3] {
            Struct::method();
        }
    }
}
