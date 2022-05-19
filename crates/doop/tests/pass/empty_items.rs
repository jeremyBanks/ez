fn main() {
    doop::doop!{
        static {
            let n = 0;
        }
        let None = [];
        let Nothing = [[]];
        let Nothings = [, , , ,];
        for N in None {
            let n = n + 1000;
        }
        for N in Nothing {
            let n = n + 100;
        }
        for N in Nothings {
            let n = n + 1;
        }
    }
    assert_eq!(n, 105);
}
