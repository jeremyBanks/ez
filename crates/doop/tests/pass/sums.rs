use doop::doop;

fn main() {
    doop! {
        static {
            const SUM: i32 = 0
        }
        for N in [1, 2, 3] { + N }
        static {
            ;
            assert_eq!(SUM, 6);
        }
    }

    let mut sum: i32 = 0;
    doop! {
        for N in [1, 2, 3] {
            sum += N;
        }
    }
    assert_eq!(sum, 6);
}
