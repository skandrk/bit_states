use bitstates::*;
use std::cell::RefCell;

#[derive(Debug, Copy, Clone, BitStates, PartialEq, Eq)]
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

    set_status.set(0b_0000_0001);
    assert_eq!(events_up.borrow_mut().pop(), Some(Status::Zero));
    assert_eq!(events_down.borrow_mut().pop(), None);
    set_status.set(0b_0000_0010);
    assert_eq!(events_up.borrow_mut().pop(), Some(Status::One));
    assert_eq!(events_down.borrow_mut().pop(), Some(Status::Zero));
    set_status.set(0b_0000_0100);
    assert_eq!(events_up.borrow_mut().pop(), Some(Status::Two));
    assert_eq!(events_down.borrow_mut().pop(), Some(Status::One));
    set_status.set(0b_0001_0000);
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
