use ez::throws;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for n in 0..10 {
        let x = fallible_dynamic(n)?;
        let y = fallible_concrete(n)?;
        println!("{x} {y}");
    }

    Ok(())
}

// throws a dynamic (eyre::Report) error
#[throws]
fn fallible_dynamic(n: i32) -> i32 {
    match n {
        // implicit return
        0 => 1,
        1 => {
            // explicit return
            return 2;
        },
        2 => {
            [3, 4, 5]
                .iter()
                .map(|x| {
                    // doesn't touch return in a closure
                    return x * 2;
                })
                .collect::<Vec<_>>()
                .last()
                .unwrap();
        },
        3 => {
            fn double(x: i32) -> i32 {
                let double = x * 2;
                // doesn't touch return in a nested function
                return double;
            }
            double(4)
        },
        4 => {
            // errors can be propagated with ?
            "what".parse()?
        },
        5 => {
            // error messages can be returned with throw!;
            throw!("this is an error");
        },
        _ => {
            // or specific error types
            let e: std::num::ParseIntError = "what".parse::<i32>().unwrap_err();
            throw!(e)
        },
    }
}

#[throws(std::num::ParseIntError)]
fn fallible_concrete(n: i32) -> i32 {
    match n {
        // implicit return
        0 => 1,
        1 => {
            // explicit return
            return 2;
        },
        2 => {
            [3, 4, 5]
                .iter()
                .map(|x| {
                    // doesn't touch return in a closure
                    return x * 2;
                })
                .collect::<Vec<_>>()
                .last()
                .unwrap();
        },
        3 => {
            fn double(x: i32) -> i32 {
                let double = x * 2;
                // doesn't touch return in a nested function
                return double;
            }
            double(4)
        },
        4 => {
            // errors can be propagated with ?
            "what".parse()?
        },
        _ => {
            // errors can be returned with throw! (but plain error messages aren't permitted
            // because they don't satisfy the concrete error type)
            let e: std::num::ParseIntError = "what".parse::<i32>().unwrap_err();
            throw!(e)
        },
    }
}
