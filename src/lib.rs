/*
 * This program basically just panics to see what happens under various
 * conditions when the Rust program panics.  You run it with two required
 * environment variables:
 *
 * - ECD_USE_COREDUMP ("true" or "false"): whether to register the panic handler
 *   from the "coredump" crate
 * - ECD_CATCH ("true" or "false"): whether to try to catch the panic within
 *   this program
 */

pub fn do_test() -> Result<(), Box<dyn std::error::Error>> {
    let use_coredump_crate =
        std::env::var("ECD_USE_COREDUMP")?.parse::<bool>()?;
    let try_to_catch_it = std::env::var("ECD_CATCH")?.parse::<bool>()?;

    if use_coredump_crate {
        coredump::register_panic_handler().unwrap();
        eprintln!("registering panic handler from \"coredump\" crate");
    }

    eprintln!("pid {}", std::process::id());
    if try_to_catch_it {
        eprintln!("attempting to catch panic");
        let x = std::panic::catch_unwind(|| foo("hello world"));
        eprintln!("caught! result: {:?}", x);
    } else {
        foo("hello world");
    }

    Ok(())
}

fn foo(input: &str) {
    bar(input);
}

fn bar(input: &str) {
    eprintln!("{}; about to panic", input);
    panic!("boom");
}
