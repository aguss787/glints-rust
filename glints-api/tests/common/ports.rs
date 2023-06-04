use std::sync::Mutex;

static MAX_PORT: usize = 11000;
static MIN_PORT: usize = 10000;
static NEXT_PORT: Mutex<usize> = Mutex::new(MIN_PORT);

pub fn allocate_port() -> usize {
    let mut lock = NEXT_PORT.lock().expect("unable to get port lock");

    let selected_port = *lock;
    *lock = if selected_port + 1 <= MAX_PORT {
        selected_port + 1
    } else {
        MIN_PORT
    };

    selected_port
}
