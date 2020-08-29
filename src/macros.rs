/// A macro to automatically run a blocking version of some async function,
/// used for the blocking module.
#[macro_export]
macro_rules! run_blocking {
    ($original: expr) => {
        $crate::blocking::RT
            .handle()
            .block_on(async move { $original.await })
    };
}
