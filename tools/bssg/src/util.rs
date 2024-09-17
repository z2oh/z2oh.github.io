pub mod prelude {
    pub use log::{error, warn, info, debug, trace};
    pub use crate::benchmark;
    pub use anyhow::{anyhow, Result};
}

#[macro_export]
macro_rules! benchmark {
    ($msg:expr, $code:block) => {{
        if cfg!(debug_assertions) {
            let start = std::time::Instant::now();
            let result = $code;
            let duration = start.elapsed();

            let msg = (|| $msg)();
            info!("{} {:?}", msg, duration);

            result
        } else {
            $code
        }
    }};
}