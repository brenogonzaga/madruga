#[macro_export]
macro_rules! madruga_retry {
    ($strategy:expr, || $op:expr) => {
        $crate::retry::retry_async($strategy, |_| $op())
    };

    ($strategy:expr, || async $body:block) => {
        $crate::retry::retry_async($strategy, |_| async $body)
    };

    ($strategy:expr, |$attempt:ident| $op:expr) => {
        $crate::retry::retry_async($strategy, |$attempt| $op)
    };

    ($strategy:expr, |$attempt:ident| async $body:block) => {
        $crate::retry::retry_async($strategy, |$attempt| async $body)
    };
}
