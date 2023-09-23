extern crate signal;

use signal::trap::Trap;

fn main() {
    // Create a new trap
    let trap = Trap::trap(&[signal::Signal::INT, signal::Signal::TERM]);

    // Install a signal handler that will print a message when the signal is received
    trap.handle(|signal| {
        println!("Received signal {}", signal);
    });

    // Wait for a signal to be received
    trap.wait();
}