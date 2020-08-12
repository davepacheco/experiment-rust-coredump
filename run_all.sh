#!/bin/bash

#
# run_all.sh: runs the included binary using all configurations of:
#
#     (panicstrategy, use coredump crate, try to catch the panic)
#
# The panic strategy is specified by CARGO_PROFILE_DEV_PANIC=abort.  The other
# two options are determined by command-line flags to the binary.
#

PANIC_STRATEGIES="unwind abort"
USE_COREDUMP_CRATE="false true"
TRY_CATCH="false true"

echo "uname:          $(uname -a)"
echo "cargo:          $(cargo --version)"
echo "cargo +nightly: $(cargo +nightly --version) (used for integration tests)"
echo "ulimit -c:      $(ulimit -c)"

for strategy in $PANIC_STRATEGIES; do
	for use_crate in $USE_COREDUMP_CRATE; do
		for try_catch in $TRY_CATCH; do
			echo "------------------------------------"
			echo "mode:               executable binary"
			echo "panic strategy:     $strategy"
			echo "use coredump crate: $use_crate"
			echo "try to catch panic: $try_catch"
			(set -o xtrace;
			RUST_BACKTRACE=full \
			ECD_USE_COREDUMP=$use_crate \
			ECD_CATCH=$try_catch \
			CARGO_PROFILE_DEV_PANIC=$strategy \
			cargo run \
			echo "bash exit status = $?" )
			echo
		done
	done
done

# Run the same experiments for an integration test.

for strategy in $PANIC_STRATEGIES; do
	for use_crate in $USE_COREDUMP_CRATE; do
		for try_catch in $TRY_CATCH; do
			echo "------------------------------------"
			echo "mode:               integration test"
			echo "panic strategy:     $strategy"
			echo "use coredump crate: $use_crate"
			echo "try to catch panic: $try_catch"
			(set -o xtrace;
			RUST_BACKTRACE=full \
			ECD_USE_COREDUMP=$use_crate \
			ECD_CATCH=$try_catch \
			CARGO_PROFILE_DEV_PANIC=$strategy \
			CARGO_PROFILE_TEST_PANIC=$strategy \
			cargo +nightly test -Z panic-abort-tests -- --nocapture
			echo "bash exit status = $?" )
			echo
		done
	done
done


