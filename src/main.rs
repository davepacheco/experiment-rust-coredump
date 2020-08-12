/*
 * This program basically just panics to see what happens under various
 * conditions when the Rust program panics.  You run it with two required
 * arguments:
 *
 * - USE_COREDUMP_CRATE ("true" or "false"): whether to register the panic
 *   handler from the "coredump" crate
 * - TRY_TO_CATCH_IT ("true" or "false"): whether to try to catch the panic
 *   within this program
 */

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        let arg0 = args.get(0).map(|s| s.as_str()).unwrap_or("demo");
        eprintln!("usage: {} USE_COREDUMP_CRATE TRY_TO_CATCH_IT", arg0);
        std::process::exit(2);
    }

    let use_coredump_crate = args[1].parse::<bool>()?;
    let try_to_catch_it = args[2].parse::<bool>()?;

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
