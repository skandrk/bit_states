#[cfg(feature = "atomic")]
pub use bit_states_derive::AtomicBitStates;

#[cfg(feature = "non-atomic")]
pub use bit_states_derive::BitStates;

#[cfg(test)]
mod tests {

    use crate::*;
    use std::{
        cell::RefCell,
        sync::{Arc, Mutex},
    };

    #[derive(Debug, Clone, Copy, AtomicBitStates, BitStates, PartialEq, Eq)]
    #[repr(u8)]
    enum Status {
        Zero = 0,
        One = 1,
        Two = 2,
        Three = 3,
        Four = 4,
    }

    #[test]
    fn normal() {
        let events_up: RefCell<Vec<Status>> = RefCell::new(Vec::new());
        let events_down: RefCell<Vec<Status>> = RefCell::new(Vec::new());

        let mut set_status = StatusStates::new(
            |a| events_up.borrow_mut().push(a),
            |a| events_down.borrow_mut().push(a),
        );

        set_status.set(0b_0000_0001 as u8);
        assert_eq!(events_up.borrow_mut().pop(), Some(Status::Zero));
        assert_eq!(events_down.borrow_mut().pop(), None);
        set_status.set(0b_0000_0010 as u8);
        assert_eq!(events_up.borrow_mut().pop(), Some(Status::One));
        assert_eq!(events_down.borrow_mut().pop(), Some(Status::Zero));
        set_status.set(0b_0000_0100 as u8);
        assert_eq!(events_up.borrow_mut().pop(), Some(Status::Two));
        assert_eq!(events_down.borrow_mut().pop(), Some(Status::One));
        set_status.set(0b_0001_0000 as u8);
        assert_eq!(events_up.borrow_mut().pop(), Some(Status::Four));
        assert_eq!(events_down.borrow_mut().pop(), Some(Status::Two));
    }

    #[test]
    fn normal_helpers() {
        let events_up: RefCell<Vec<Status>> = RefCell::new(Vec::new());
        let events_down: RefCell<Vec<Status>> = RefCell::new(Vec::new());

        let mut set_status = StatusStates::new(
            |a| events_up.borrow_mut().push(a),
            |a| events_down.borrow_mut().push(a),
        );

        set_status.set_flag(Status::Three);
        assert_eq!(events_up.borrow_mut().pop(), Some(Status::Three));
        assert_eq!(set_status.is_set(Status::Three), true);
        set_status.clear();
        assert_eq!(set_status.bit_state, 0);
        set_status.reset_flag(Status::Three);
        assert_ne!(events_down.borrow_mut().pop(), Some(Status::Three));
    }

    #[test]
    fn atomics() {
        let events_up: Arc<Mutex<Vec<Status>>> = Arc::new(Mutex::new(Vec::new()));
        let events_down: Arc<Mutex<Vec<Status>>> = Arc::new(Mutex::new(Vec::new()));

        let set_status = std::sync::Arc::new(StatusAtomicStates::new(
            {
                let events_up = events_up.clone();
                move |a| events_up.lock().unwrap().push(a)
            },
            {
                let events_down = events_down.clone();
                move |a| events_down.lock().unwrap().push(a)
            },
        ));

        set_status.set(0b_0000_0001 as u8);

        let j = std::thread::spawn({
            let set_status = set_status.clone();
            move || {
                set_status.set(0b_0001_0000 as u8);
            }
        });

        j.join().unwrap();

        assert_eq!(events_up.lock().unwrap().len(), 2);
        assert_eq!(events_down.lock().unwrap().len(), 1);

        assert!(events_up.lock().unwrap().contains(&Status::Zero));
        assert!(events_up.lock().unwrap().contains(&Status::Four));
        assert!(events_down.lock().unwrap().contains(&Status::Zero));
    }
}
