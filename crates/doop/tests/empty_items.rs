
        #[test]
        fn test() {
    doop::doop!{
        static {
            let n = 0;
        }

        // this is an empty set
        let None = [];

        // this is the set containing only the empty list
        let Nothing = [[], [], []];
        let NoneAndNothing = None + Nothing;

        for N in None {
            let n = n + 1000;
        }
        for N in Nothing {
            let n = n + 100;
        }
        for N in NoneAndNothing {
            let n = n + 1;
        }
    }
    assert_eq!(n, 101);
}
