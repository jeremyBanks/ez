#[test]
fn test() {
    doop::doop! {
        let Numbers = [
            1,
            2,
            3,
            4,
            5,
            6,
        ];
        static {
            let n = 0;
        }
        for N in Numbers {
            let n = n + N;
        }
    }
    assert_eq!(n, 21);
}
