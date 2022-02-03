use std::num::ParseIntError;

fn success() -> Result<i64, ParseIntError> {
    "2".parse()
}

fn failure() -> Result<i64, ParseIntError> {
    "threeve".parse()
}

mod throws {
    use super::*;

    mod free_functions {
        use super::*;

        #[test]
        fn test() -> Result<(), eyre::Report> {
            assert_eq!(static_ok()?, 2);
            assert_eq!(dynamic_ok()?, 2);
            assert!(static_err().is_err());
            assert!(dynamic_err().is_err());

            Ok(())
        }

        #[ez::throws]
        fn dynamic_ok() -> i64 {
            success()?
        }

        #[ez::throws]
        fn dynamic_err() -> i64 {
            failure()?
        }

        #[ez::throws(ParseIntError)]
        fn static_ok() -> i64 {
            success()?
        }

        #[ez::throws(ParseIntError)]
        fn static_err() -> i64 {
            failure()?
        }
    }

    mod traits {
        use super::*;

        #[test]
        fn test() -> Result<(), eyre::Report> {
            assert_eq!(static_ok()?, 2);
            assert_eq!(dynamic_ok()?, 2);
            assert!(static_err().is_err());
            assert!(dynamic_err().is_err());

            Ok(())
        }

        #[ez::throws]
        fn dynamic_ok() -> i64 {
            if 1 % 2 == 3 {
                success()?;
            }
            // if 1 % 2 == 3 {
            //     return success()?;
            // }
            // if 1 % 2 == 3 {
            //     ez::throw!(failure().unwrap_err());
            // }
            // if 1 % 2 == 3 {
            //     return failure()?;
            // }
            Default::default()
        }

        // fn dynamic_ok(a: i64, b: &str, c: String) -> Result<&str, eyre::Report> {
        //     {
        //         fn ez_unhygienic_dynamic_ok(a: i64, b: &str, c: String) -> impl ez::IntoResult<&str, eyre::Report> {
        //             b
        //         }
        //         ez::IntoResult::into_result(ez_unhygienic_dynamic_ok(a, b, c))
        //     }
        // }

        #[ez::throws]
        fn dynamic_err() -> i64 {
            failure()?
        }

        #[ez::throws(ParseIntError)]
        fn static_ok() -> i64 {
            success()?
        }

        #[ez::throws(ParseIntError)]
        fn static_err() -> i64 {
            failure()?
        }
    }
}
