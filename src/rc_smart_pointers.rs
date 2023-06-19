use std::cell::RefCell;
use std::rc::Rc;

struct Database {
    max_connections: u32,
}
struct AuthService {
    db: Rc<RefCell<Database>>,
}
struct ContentService {
    db: Rc<RefCell<Database>>,
}

pub fn run_example() {
    let db = Rc::new(RefCell::new(Database {
        max_connections: 100,
    }));
    let auth_service = AuthService { db: Rc::clone(&db) };
    let content_service = ContentService { db: Rc::clone(&db) };

    // auth_service and content_service can now mutate the database
    let mut r1 = db.borrow_mut();
    // r1 is dropped here so it can be borrowed again. Otherwise, will panic
    drop(r1);

    let mut r2 = db.borrow_mut();
    r2.max_connections = 200;
    println!("Max connections: {}", r2.max_connections);
}
