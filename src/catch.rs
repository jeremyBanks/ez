#[test]
fn test() {
    catch! {
        try {
            let i: 32 = "4.2".parse().unwrap();
        }
        catch (err: SomeErrors::Foo) {

        }
        catch (err: SomeErrors::Foo) {

        }
        finally {

        }
    }
}

#[derive(thiserror::Error, Debug)]
enum SomeErrors {
    Foo,
    Bar,
}

macro_rules! catch_unhandled {
    { $($tt:tt)* } => {
        PANIC_HOOK_SUPPRESSIONS.with(|suppressions_cell| {
            suppressions_cell.set(suppressions_cell.get() + 1);
            let result = std::panic::catch_unwind(|| {
                    $($tt)*
            }).map_err(|err| {
                // if we can downcast to eyre, we can wrap it with
                // more context
                eprintln!("ERRRRR = {:#?}", err);
                if let Ok(err) = Box::new(err).downcast::<eyre::Report>() {
                    err.wrap_err("unhandled error panicked")
                } else {
                    eyre!("unhandled non-dynamic error panicked")
                }
            });
            suppressions_cell.set(suppressions_cell.get() - 1);
            result
        })
    };
}

thread_local! {
    static PANIC_HOOK_SUPPRESSIONS: Cell<i64> = Cell::new(0);
}

fn install_hook_suppressor() {
    static INSTALLED: std::sync::Once = std::sync::Once::new();
    INSTALLED.call_once(|| {
        let wrapped_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            PANIC_HOOK_SUPPRESSIONS.with(|suppressions_cell| {
                let suppressions = suppressions_cell.get();
                if suppressions == 0 {
                    wrapped_hook(info);
                }
            });
        }));
    });
}
