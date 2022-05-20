use doop::{doop, dooped};

struct S1;
struct S2;
struct S3;

trait T1 {
    fn t1() {}
}
trait T2 {
    fn t2() {}
}
trait T3 {
    fn t3() {}
}

#[dooped(for Struct in [S1, S2, S3] for Trait in [T1, T2, T3])]
impl Trait for Struct {}

trait T4 {
    fn t4() {}
}
trait T5 {
    fn t5() {}
}
trait T6 {
    fn t6() {}
}

#[dooped(for (Struct, Trait) in [(S1, T4), (S2, T5), (S3, T6)])]
impl Trait for Struct {}

#[test]
fn test() {
    doop! {
        for Struct in [S1, S2, S3]
        for method in [t1, t2, t3] {
            Struct::method();
        }

        for (Struct, method) in [(S1, t4), (S2, t5), (S3, t6)] {
            Struct::method();
        }
    }
}
