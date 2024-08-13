#![no_std]

use gstd::{msg, prelude::*, ActorId};

#[gstd::async_main]
async fn main() {
    let destination: ActorId = msg::load().expect("Unable to load message");
    msg::send_for_reply(destination, "", 0, 0)
        .expect("Error in send")
        .await
        .expect("Error in reply");
}

#[cfg(test)]
mod tests {
    extern crate std;

    use gstd::{ActorId};
    use gtest::{Log, Program, System};

    #[test]
    fn test_corrupted_tree() {
        let system = System::new();
        system.init_logger();

        let program = Program::current(&system);

        let res = program.send_bytes(10, "INIT");
        assert!(!res.main_failed());

        let destination: ActorId = 100.into();

        program.send(42, destination);
        system.spend_blocks(100);
        
    }
}
