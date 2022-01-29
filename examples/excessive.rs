use {
    indexmap::IndexMap,
    std::{
        collections::{BTreeMap, HashMap},
        fmt::Debug,
    },
};

// #[ez::main]
// fn main(args: &[&str], env: &OrderedMap<String, String>) {

// }

// #[ez::main]
// async fn main_2(args: {
//     /// Turns the thing on.
//     flag: boolean = false,
// }, env: &OrderedMap<String, String>) -> u8 {

// }

trait Main {
    type Args: MainArgs;
    type Env: MainEnv;
    type ExitStatus: MainExitStatus;

    fn call(&self, args: Self::Args, env: Self::Env) -> Self::ExitStatus;
}

impl<Args, Env, ExitStatus> Main for fn(Args, Env) -> ExitStatus
where
    Args: MainArgs,
    Env: MainEnv,
    ExitStatus: MainExitStatus,
{
    type Args = Args;
    type Env = Env;
    type ExitStatus = ExitStatus;

    fn call(&self, args: Self::Args, env: Self::Env) -> Self::ExitStatus {
        self(args, env)
    }
}

trait MainArgs: FromIterator<String> {}
impl<T> MainArgs for T where T: FromIterator<String> {}

trait MainEnv: FromIterator<(String, String)> {}
impl<T> MainEnv for T where T: FromIterator<(String, String)> {}

trait MainExitStatus: Into<i32> {}
impl<T> MainExitStatus for T where T: Into<i32> {}

pub fn main() {
    fn run_main<F: Main>(f: F) {
        // TODO: don't panic on UTF-8 errors
        let args = std::env::args().skip(1).map(|s| s.to_string());
        let args = F::Args::from_iter(args);

        // TODO: don't panic on UTF-8 errors
        let env = std::env::vars().map(|(a, b)| (a.to_string(), b.to_string()));
        let env = F::Env::from_iter(env);

        let status = f(args, env).into();
    }

    let i = vec![
        "foo".to_string(),
        "bar".to_string(),
        "baz".to_string(),
        "qux".to_string(),
    ]
    .into_iter();
    Vec::<String>::from_iter(i);

    let i = vec![
        ("foo".to_string(), "bar".to_string()),
        ("baz".to_string(), "qux".to_string()),
    ]
    .into_iter();
    HashMap::<String, String>::from_iter(i);

    fn inner_main(args: Vec<String>, env: HashMap<String, String>) -> u8 {
        return 0;
    }

    run_main(inner_main as fn(_, _) -> _);
}
