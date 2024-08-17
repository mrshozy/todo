#[macro_export]
macro_rules! log {
    (info, $($arg:tt)*) => {
        $crate::tracing::info!(
            target:$crate::APPLICATION_NAME,
            $($arg)*
        );
    };
    (error, $($arg:tt)*) => {
        $crate::tracing::error!(
            target:$crate::APPLICATION_NAME,
            $($arg)*
        );
    };
    (warn, $($arg:tt)*) => {
        $crate::tracing::warn!(
            target:$crate::APPLICATION_NAME,
            $($arg)*
        );
    };
    (fatal, $($arg:tt)*) => {
        {
            $crate::tracing::error!(
                target:$crate::APPLICATION_NAME,
                $($arg)*
            );
            std::process::exit(1);
        }
    };
    ($level:ident, $($arg:tt)*) => {
        $crate::tracing::$level!(
            target:$crate::APPLICATION_NAME,
            $($arg)*
        );
    };
}
