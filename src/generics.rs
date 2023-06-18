#[derive(Debug)]
struct BrowserCommand<T> {
    name: String,
    payload: T,
}

impl<T> BrowserCommand<T> {
    fn new(name: String, payload: T) -> Self {
        BrowserCommand { name, payload }
    }
    fn get_payload(&self) -> &T {
        &self.payload
    }
}

impl BrowserCommand<String> {
    fn print(&self) {
        println!("I'm a string {}: {}", self.name, self.payload);
    }
}

impl BrowserCommand<i32> {
    fn print(&self) {
        println!("I'm an integer {}: {}", self.name, self.payload);
    }
}

pub fn run_example() {
    let cmd1 = BrowserCommand::new(
        String::from("open"),
        String::from("https://www.rust-lang.org"),
    );
    let cmd2 = BrowserCommand::new(String::from("close"), 200);

    cmd1.print();
    cmd2.print();
    let payload1 = cmd1.get_payload();
    let payload2 = cmd2.get_payload();

    serialize_payload(payload1);
    serialize_payload(payload2);
}

fn serialize_payload<T>(_payload: &T) -> String {
    "payload".to_owned()
}
