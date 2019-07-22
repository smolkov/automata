/// Time date function abstraction.
use std::time;

///Current system time in second
///
pub fn now_sec() -> u64 {
    let now = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap();
    now.as_secs()
}


//
