#[macro_export]
macro_rules! time {
    ($label:expr, $block:block) => {{
        let start = Instant::now();
        let result = { $block };
        let duration = start.elapsed();
        debug!("[{}]: Time elapsed: {:?}", $label, duration);
        result
    }};
}
