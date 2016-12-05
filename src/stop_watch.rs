struct StopWatch {
    is_running: bool,
    elapsed_ms: u64,
}

impl StopWatch {
    fn new() -> Self {
        StopWatch {
            is_running: false,
            elapsed_ms: 0,
        }
    }

    fn start(&mut self) {}

    fn stop(&mut self) {}

    fn reset(&mut self) {}

    fn restart(&mut self) {}

    fn elapsed_ms(&self) -> u64 {
        self.elapsed_ms

    }
}
