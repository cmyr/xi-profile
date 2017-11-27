# Very very basic xi profiling

This repo contains a simple harness for timing RPC performance.

It simulates a client, with no runloop; a series of simple init RPCs
are sent to core, and all received messages are logged and timestamped.

Because there is no runloop on the client side, this should give a
fairly accurate picture of core responsiveness in the most simple case.


## Use:

To setup: `make setup`

To run: `cargo run`

Results are printed to stdout.
