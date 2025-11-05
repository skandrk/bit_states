use bitstates::*;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, AtomicBitStates, PartialEq, Eq)]
#[repr(u8)]
enum Status {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
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

#[test]
fn atomic_helpers() {
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

    let j = std::thread::spawn({
        let set_status = set_status.clone();
        move || {
            set_status.set(0b_0001_0000 as u8);
            set_status.set_flag(Status::Three);
        }
    });

    j.join().unwrap();
    assert_eq!(set_status.get(), 0b_0001_1000 as u8);
    assert_eq!(events_up.lock().unwrap().pop(), Some(Status::Three));
    assert_eq!(set_status.is_set(Status::Three), true);
    set_status.clear();
    assert_eq!(set_status.get(), 0);
    set_status.reset_flag(Status::Three);
    assert_eq!(events_down.lock().unwrap().len(), 0);
}
