fn main() {
    doop::doop!{
        static {
            let n = 0;
        }

        let None = [];

        let Nothing = [[], [], [],];
        let Nothings = [, , , ,];
        let NoneAndNothing = None + Nothing;

        for N in None {
            let n = n + 1000;
        }
        for N in Nothing {
            let n = n + 100;
        }
        for N in Nothings {
            let n = n + 10;
        }
        for N in NoneAndNothing {
            let n = n + 1;
        }
    }
}
