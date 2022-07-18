use raft::{raw_node::RawNode, storage::MemStorage, Config};
use slog::{o, Drain};

use std::{
    sync::mpsc::{channel, RecvTimeoutError},
    time::{Duration, Instant},
};

fn main() {
    // Select some defaults, then change what we need.
    let config = Config {
        id: 1,
        ..Default::default()
    };
    // Initialize logger.
    let logger = slog::Logger::root(slog_stdlog::StdLog.fuse(), o!());
    // ... Make any configuration changes.
    // After, make sure it's valid!
    config.validate().unwrap();
    // We'll use the built-in `MemStorage`, but you will likely want your own.
    // Finally, create our Raft node!
    let storage = MemStorage::new_with_conf_state((vec![1], vec![]));
    let mut node = RawNode::new(&config, storage, &logger).unwrap();

    let (tx, rx) = channel();
    let timeout = Duration::from_millis(100);
    let mut remaining_timeout = timeout;

    // Send the `tx` somewhere else...

    loop {
        let now = Instant::now();

        match rx.recv_timeout(remaining_timeout) {
            Ok(()) => {
                // Let's save this for later.
                unimplemented!()
            }
            Err(RecvTimeoutError::Timeout) => (),
            Err(RecvTimeoutError::Disconnected) => unimplemented!(),
        }

        let elapsed = now.elapsed();
        if elapsed >= remaining_timeout {
            remaining_timeout = timeout;
            // We drive Raft every 100ms.
            node.tick();
        } else {
            remaining_timeout -= elapsed;
        }
    }
}
