// ##################################################################### TRACE #####################################################################

/// Logs a message for tracing - very low priority.
///
/// # Example
///
/// ``` no_run
/// use logfather::trace;
///
/// trace!("This is a traced message");
/// ```
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{
        $crate::log($crate::Level::Trace, module_path!(), format_args!($($arg)*))
    }};
}

/// Logs a message for tracing - very low priority.
///
/// # Example
///
/// ``` no_run
/// use logfather::r_trace;
///
/// let result = r_trace!("This is a traced message");
/// if result.is_err() {
///     println!("The log failed.");
/// }
/// ```
#[macro_export]
macro_rules! r_trace {
    ($($arg:tt)*) => {{
        $crate::result_log($crate::Level::Trace, module_path!(), format_args!($($arg)*))
    }};
}

// ##################################################################### DEBUG #####################################################################
/// Logs a message for debugging and will be ignored on release builds.
///
/// # Example
///
/// ``` no_run
/// use logfather::debug;
///
/// debug!("This is a debug message");
/// ```
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
        $crate::log($crate::Level::Debug, module_path!(), format_args!($($arg)*))
        }
    };
}

/// Logs a message for debugging and will be ignored on release builds.
///
/// # Example
///
/// ``` no_run
/// use logfather::r_debug;
///
/// let result = r_debug!("This is a debug message");
/// if result.is_err() {
///     println!("The log failed.");
/// }
/// ```
#[macro_export]
macro_rules! r_debug {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        {
            $crate::result_log($crate::Level::Debug, module_path!(), format_args!($($arg)*))
        }
        #[cfg(not(debug_assertions))]
        {
            Ok::<(), LogfatherError>(())
        }
    }};
}

// ##################################################################### INFO #####################################################################
/// Logs an informational message.
///
/// # Example
///
/// ``` no_run
/// use logfather::info;
///
/// info!("This is an info message");
/// ```
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::log($crate::Level::Info, module_path!(), format_args!($($arg)*));
    }};
}

/// Logs an informational message.
///
/// # Example
///
/// ``` no_run
/// use logfather::{r_info, LogfatherError};
///
/// let result = r_info!("This is an info message");
/// if result.is_err() {
///     println!("The log failed.");
/// }
/// ```
#[macro_export]
macro_rules! r_info {
    ($($arg:tt)*) => {{
        $crate::result_log($crate::Level::Info, module_path!(), format_args!($($arg)*))
    }};
}

// ##################################################################### WARNING #####################################################################
/// Logs a warning message.
///
/// # Example
///
/// ``` no_run
/// use logfather::warning;
///
/// warning!("This is a warning message");
/// ```
///
/// This macro simplifies the process of logging a message at the `Warning` level.
#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {{
        $crate::log($crate::Level::Warning, module_path!(), format_args!($($arg)*))
    }};
}

/// Logs a warning message.
///
/// # Example
///
/// ``` no_run
/// use logfather::warn;
///
/// warn!("This is a warning message");
/// ```
///
/// This macro simplifies the process of logging a message at the `Warning` level.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        $crate::log($crate::Level::Warning, module_path!(), format_args!($($arg)*))
    }};
}

/// Logs a warning message.
///
/// # Example
///
/// ``` no_run
/// use logfather::r_warning;
///
/// let result = r_warning!("This is a warning message");
/// if result.is_err() {
///     println!("The log failed.");
/// }
/// ```
///
/// This macro simplifies the process of logging a message at the `Warning` level.
#[macro_export]
macro_rules! r_warning {
    ($($arg:tt)*) => {{
        $crate::result_log($crate::Level::Warning, module_path!(), format_args!($($arg)*))
    }};
}

/// Logs a warning message.
///
/// # Example
///
/// ``` no_run
/// use logfather::r_warn;
///
/// let result = r_warn!("This is a warning message");
/// if result.is_err() {
///     println!("The log failed.");
/// }
/// ```
///
/// This macro simplifies the process of logging a message at the `Warning` level.
#[macro_export]
macro_rules! r_warn {
    ($($arg:tt)*) => {{
        $crate::result_log($crate::Level::Warning, module_path!(), format_args!($($arg)*))
    }};
}

// ##################################################################### ERROR #####################################################################
/// Logs an error message.
///
/// # Example
///
/// ``` no_run
/// use logfather::error;
///
/// error!("This is an error message");
/// ```
///
/// Use this macro for logging errors, typically when an operation fails or an unexpected condition occurs.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        $crate::log($crate::Level::Error, module_path!(), format_args!($($arg)*))
    }};
}

/// Logs an error message.
///
/// # Example
///
/// ``` no_run
/// use logfather::r_error;
///
/// let result = r_error!("This is an error message");
/// if result.is_err() {
///     println!("The log failed.");
/// }
/// ```
///
/// Use this macro for logging errors, typically when an operation fails or an unexpected condition occurs.
#[macro_export]
macro_rules! r_error {
    ($($arg:tt)*) => {{
        $crate::result_log($crate::Level::Error, module_path!(), format_args!($($arg)*))
    }};
}

// ##################################################################### CRITICAL #####################################################################
/// Logs a critical message.
///
/// # Example
///
/// ``` no_run
/// use logfather::critical;
///
/// critical!("This is a critical message");
/// ```
///
/// This macro is intended for critical errors that require immediate attention. Logging at this level typically indicates a serious failure in a component of the application.
#[macro_export]
macro_rules! critical {
    ($($arg:tt)*) => {{
        $crate::log($crate::Level::Critical, module_path!(), format_args!($($arg)*))
    }};
}

/// Logs a critical message.
///
/// # Example
///
/// ``` no_run
/// use logfather::crit;
///
/// crit!("This is a critical message");
/// ```
///
/// This macro is intended for critical errors that require immediate attention. Logging at this level typically indicates a serious failure in a component of the application.
#[macro_export]
macro_rules! crit {
    ($($arg:tt)*) => {{
        $crate::log($crate::Level::Critical, module_path!(), format_args!($($arg)*))
    }};
}

/// Logs a critical message.
///
/// # Example
///
/// ``` no_run
/// use logfather::r_critical;
///
/// let result = r_critical!("This is a critical message");
/// if result.is_err() {
///     println!("The log failed.");
/// }
/// ```
///
/// This macro is intended for critical errors that require immediate attention. Logging at this level typically indicates a serious failure in a component of the application.
#[macro_export]
macro_rules! r_critical {
    ($($arg:tt)*) => {{
        $crate::result_log($crate::Level::Critical, module_path!(), format_args!($($arg)*))
    }};
}

/// Logs a critical message.
///
/// # Example
///
/// ``` no_run
/// use logfather::r_crit;
///
/// let result = r_crit!("This is a critical message");
/// if result.is_err() {
///     println!("The log failed.");
/// }
/// ```
///
/// This macro is intended for critical errors that require immediate attention. Logging at this level typically indicates a serious failure in a component of the application.
#[macro_export]
macro_rules! r_crit {
    ($($arg:tt)*) => {{
        $crate::result_log($crate::Level::Critical, module_path!(), format_args!($($arg)*))
    }};
}

// ##################################################################### DIAGNOSTIC #####################################################################
/// Logs a diagnostic message and ignores filters -- debug builds only.
///
/// # Example
///
/// ``` no_run
/// use logfather::diagnostic;
///
/// diagnostic!("This is a critical message");
/// ```
#[macro_export]
macro_rules! diagnostic {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::log($crate::Level::Diagnostic, module_path!(), format_args!($($arg)*))
        }
    };
}

/// Logs a diagnostic message and ignores filters -- debug builds only.
///
/// # Example
///
/// ``` no_run
/// use logfather::diag;
///
/// diag!("This is a critical message");
/// ```
#[macro_export]
macro_rules! diag {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            $crate::log($crate::Level::Diagnostic, module_path!(), format_args!($($arg)*))
        }
    };
}

/// Logs a diagnostic message and ignores filters -- debug builds only.
///
/// # Example
///
/// ``` no_run
/// use logfather::r_diagnostic;
///
/// let result = r_diagnostic!("This is a critical message");
/// if result.is_err() {
///     println!("The log failed.");
/// }
/// ```
#[macro_export]
macro_rules! r_diagnostic {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        {
            $crate::result_log($crate::Level::Diagnostic, module_path!(), format_args!($($arg)*))
        }
        #[cfg(not(debug_assertions))]
        {
            Ok::<(), LogfatherError>(())
        }
    }};
}

/// Logs a diagnostic message and ignores filters -- debug builds only.
///
/// # Example
///
/// ``` no_run
/// use logfather::r_diag;
///
/// let result = r_diag!("This is a critical message");
/// if result.is_err() {
///     println!("The log failed.");
/// }
/// ```
#[macro_export]
macro_rules! r_diag {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        {
            $crate::result_log($crate::Level::Diagnostic, module_path!(), format_args!($($arg)*))
        }
        #[cfg(not(debug_assertions))]
        {
            Ok::<(), LogfatherError>(())
        }
    }};
}
