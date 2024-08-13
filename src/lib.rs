#![no_std]

use gstd::{msg, prelude::*, ActorId};

#[gstd::async_main]
async fn main() {
    let destination: ActorId = msg::load().expect("Unable to load message");
    let reply = msg::send_for_reply(destination, "", 0, 0)
        .expect("Error in send")
        .await
        .expect("Error in reply");

    if reply != b"hello" {
        panic!("Wrong received reply");
    } else {
        gstd::debug!("Succesfully received hello message");
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use gstd::{ActorId, prelude::*};
    use gtest::{Program, System, WasmProgram};

    #[derive(Debug)]
    pub struct ProgramMock;

    impl WasmProgram for ProgramMock {
        fn init(&mut self, _payload: Vec<u8>) -> Result<Option<Vec<u8>>, &'static str> {
            Ok(None)
        }

        fn handle(&mut self, _payload: Vec<u8>) -> Result<Option<Vec<u8>>, &'static str> {
            Ok(Some(b"hello".to_vec()))
        }

        fn handle_reply(&mut self, _payload: Vec<u8>) -> Result<(), &'static str> {
            unimplemented!()
        }

        fn handle_signal(&mut self, _payload: Vec<u8>) -> Result<(), &'static str> {
            unimplemented!()
        }

        fn state(&mut self) -> Result<Vec<u8>, &'static str> {
            unimplemented!()
        }
    }

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

    #[test]
    fn test_should_panic_since_mock_not_initialised() {
        let system = System::new();
        system.init_logger();
        let mock_program_id = 1000;

        let _mock_program = Program::mock_with_id(&system, mock_program_id, ProgramMock);
        let program = Program::current(&system);
    
        let res = program.send_bytes(10, "INIT");
        assert!(!res.main_failed());

        assert!(!program.send(42, ActorId::from(mock_program_id)).main_failed());
    }

    #[test]
    fn test_unreachable_code_in_second_mock_message() {
        let system = System::new();
        system.init_logger();
        let mock_program_id = 1000;

        let _mock_program = Program::mock_with_id(&system, mock_program_id, ProgramMock);
        let program = Program::current(&system);
    
        let res = program.send_bytes(10, "INIT");
        assert!(!res.main_failed());

        assert!(!program.send(42, ActorId::from(mock_program_id)).main_failed());

        // this cause: internal error: entered unreachable code
        assert!(!program.send(42, ActorId::from(mock_program_id)).main_failed());
    }

    #[test]
    fn test_mock_initialization_and_successful_execution() {
        let system = System::new();
        system.init_logger();
        let mock_program_id = 1000;

        let mock_program = Program::mock_with_id(&system, mock_program_id, ProgramMock);
        assert!(!mock_program.send_bytes(10, b"INIT").main_failed());
        let program = Program::current(&system);
    
        let res = program.send_bytes(10, "INIT");
        assert!(!res.main_failed());

        assert!(!program.send(42, ActorId::from(mock_program_id)).main_failed());

        assert!(!program.send(42, ActorId::from(mock_program_id)).main_failed());
    }
}
