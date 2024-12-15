// ##################################################################### TRACE #####################################################################

/// Logs a message for tracing - very low priority.
///
/// # Example
///
/// ``` no_run
/// use logfather::trace;
/// 
/// trace!("This is a normal trace message");
/// trace!("This is a normal {} message", "trace");
/// trace!("key1" = "value1", "key2" = "value2"; "This is a structured trace message");
/// let map: std::collections::HashMap<&str, &str> = std::collections::HashMap::from([("key1", "value1"), ("key2", "value2")]);
/// trace!(map; "This is also a structured trace message");
/// ```
#[macro_export]
macro_rules! trace {
    ($arg1:expr; $($arg2:tt)+) => {{
        $crate::structured_log($crate::Level::Trace, module_path!(), format_args!($($arg2)+), $arg1)
    }};
    ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
        let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
        $(
            let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
            map.insert(key, $($value)*);
        )*
        $crate::structured_log($crate::Level::Trace, module_path!(), format_args!($($args)*), map)
    }};
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
    // ($arg1:expr; $($arg2:tt)+) => {{
    //     $crate::structured_log($crate::Level::Trace, module_path!(), format_args!($($arg2)+), $arg1)
    // }};
    // ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
    //     let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
    //     $(
    //         let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
    //         map.insert(key, $($value)*);
    //     )*
    //     $crate::structured_log($crate::Level::Trace, module_path!(), format_args!($($args)*), map)
    // }};
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
/// debug!("This is a normal debug message");
/// debug!("This is a normal {} message", "debug");
/// debug!("key1" = "value1", "key2" = "value2"; "This is a structured debug message");
/// let map: std::collections::HashMap<&str, &str> = std::collections::HashMap::from([("key1", "value1"), ("key2", "value2")]);
/// debug!(map; "This is also a structured debug message");
/// ```
#[macro_export]
macro_rules! debug {
    ($arg1:expr; $($arg2:tt)+) => {{
        $crate::structured_log($crate::Level::Debug, module_path!(), format_args!($($arg2)+), $arg1)
    }};
    ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
        let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
        $(
            let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
            map.insert(key, $($value)*);
        )*
        $crate::structured_log($crate::Level::Debug, module_path!(), format_args!($($args)*), map)
    }};
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
    // ($arg1:expr; $($arg2:tt)+) => {{
    //     $crate::structured_log($crate::Level::Debug, module_path!(), format_args!($($arg2)+), $arg1)
    // }};
    // ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
    //     let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
    //     $(
    //         let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
    //         map.insert(key, $($value)*);
    //     )*
    //     $crate::structured_log($crate::Level::Debug, module_path!(), format_args!($($args)*), map)
    // }};
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
/// info!("This is a normal info message");
/// info!("This is a normal {} message", "info");
/// info!("key1" = "value1", "key2" = "value2"; "This is a structured info message");
/// let map: std::collections::HashMap<&str, &str> = std::collections::HashMap::from([("key1", "value1"), ("key2", "value2")]);
/// info!(map; "This is also a structured info message");
/// ```
#[macro_export]
macro_rules! info {
    ($arg1:expr; $($arg2:tt)+) => {{
        $crate::structured_log($crate::Level::Info, module_path!(), format_args!($($arg2)+), $arg1)
    }};
    ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
        let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
        $(
            let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
            map.insert(key, $($value)*);
        )*
        $crate::structured_log($crate::Level::Info, module_path!(), format_args!($($args)*), map)
    }};
    ($($arg:tt)+) => {{
        $crate::log($crate::Level::Info, module_path!(), format_args!($($arg)+))
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
    // ($arg1:expr; $($arg2:tt)+) => {{
    //     $crate::structured_log($crate::Level::Info, module_path!(), format_args!($($arg2)+), $arg1)
    // }};
    // ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
    //     let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
    //     $(
    //         let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
    //         map.insert(key, $($value)*);
    //     )*
    //     $crate::structured_log($crate::Level::Info, module_path!(), format_args!($($args)*), map)
    // }};
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
/// warning!("This is a normal warning message");
/// warning!("This is a normal {} message", "warning");
/// warning!("key1" = "value1", "key2" = "value2"; "This is a structured warning message");
/// let map: std::collections::HashMap<&str, &str> = std::collections::HashMap::from([("key1", "value1"), ("key2", "value2")]);
/// warning!(map; "This is also a structured warning message");
/// ```
///
/// This macro simplifies the process of logging a message at the `Warning` level.
#[macro_export]
macro_rules! warning {
    ($arg1:expr; $($arg2:tt)+) => {{
        $crate::structured_log($crate::Level::Warning, module_path!(), format_args!($($arg2)+), $arg1)
    }};
    ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
        let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
        $(
            let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
            map.insert(key, $($value)*);
        )*
        $crate::structured_log($crate::Level::Warning, module_path!(), format_args!($($args)*), map)
    }};
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
/// warn!("This is a normal warning message");
/// warn!("This is a normal {} message", "warning");
/// warn!("key1" = "value1", "key2" = "value2"; "This is a structured warning message");
/// let map: std::collections::HashMap<&str, &str> = std::collections::HashMap::from([("key1", "value1"), ("key2", "value2")]);
/// warn!(map; "This is also a structured warning message");
/// ```
///
/// This macro simplifies the process of logging a message at the `Warning` level.
#[macro_export]
macro_rules! warn {
    ($arg1:expr; $($arg2:tt)+) => {{
        $crate::structured_log($crate::Level::Warning, module_path!(), format_args!($($arg2)+), $arg1)
    }};
    ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
        let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
        $(
            let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
            map.insert(key, $($value)*);
        )*
        $crate::structured_log($crate::Level::Warning, module_path!(), format_args!($($args)*), map)
    }};
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
    // ($arg1:expr; $($arg2:tt)+) => {{
    //     $crate::structured_log($crate::Level::Warning, module_path!(), format_args!($($arg2)+), $arg1)
    // }};
    // ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
    //     let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
    //     $(
    //         let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
    //         map.insert(key, $($value)*);
    //     )*
    //     $crate::structured_log($crate::Level::Warning, module_path!(), format_args!($($args)*), map)
    // }};
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
    // ($arg1:expr; $($arg2:tt)+) => {{
    //     $crate::structured_log($crate::Level::Warning, module_path!(), format_args!($($arg2)+), $arg1)
    // }};
    // ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
    //     let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
    //     $(
    //         let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
    //         map.insert(key, $($value)*);
    //     )*
    //     $crate::structured_log($crate::Level::Warning, module_path!(), format_args!($($args)*), map)
    // }};
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
/// error!("This is a normal error message");
/// error!("This is a normal {} message", "error");
/// error!("key1" = "value1", "key2" = "value2"; "This is a structured error message");
/// let map: std::collections::HashMap<&str, &str> = std::collections::HashMap::from([("key1", "value1"), ("key2", "value2")]);
/// error!(map; "This is also a structured error message");
/// ```
///
/// Use this macro for logging errors, typically when an operation fails or an unexpected condition occurs.
#[macro_export]
macro_rules! error {
    ($arg1:expr; $($arg2:tt)+) => {{
        $crate::structured_log($crate::Level::Error, module_path!(), format_args!($($arg2)+), $arg1)
    }};
    ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
        let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
        $(
            let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
            map.insert(key, $($value)*);
        )*
        $crate::structured_log($crate::Level::Error, module_path!(), format_args!($($args)*), map)
    }};
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
    // ($arg1:expr; $($arg2:tt)+) => {{
    //     $crate::structured_log($crate::Level::Error, module_path!(), format_args!($($arg2)+), $arg1)
    // }};
    // ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
    //     let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
    //     $(
    //         let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
    //         map.insert(key, $($value)*);
    //     )*
    //     $crate::structured_log($crate::Level::Error, module_path!(), format_args!($($args)*), map)
    // }};
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
/// critical!("This is a normal critical message");
/// critical!("This is a normal {} message", "critical");
/// critical!("key1" = "value1", "key2" = "value2"; "This is a structured critical message");
/// let map: std::collections::HashMap<&str, &str> = std::collections::HashMap::from([("key1", "value1"), ("key2", "value2")]);
/// critical!(map; "This is also a structured critical message");
/// ```
///
/// This macro is intended for critical errors that require immediate attention. Logging at this level typically indicates a serious failure in a component of the application.
#[macro_export]
macro_rules! critical {
    ($arg1:expr; $($arg2:tt)+) => {{
        $crate::structured_log($crate::Level::Critical, module_path!(), format_args!($($arg2)+), $arg1)
    }};
    ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
        let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
        $(
            let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
            map.insert(key, $($value)*);
        )*
        $crate::structured_log($crate::Level::Critical, module_path!(), format_args!($($args)*), map)
    }};
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
/// crit!("This is a normal critical message");
/// crit!("This is a normal {} message", "critical");
/// crit!("key1" = "value1", "key2" = "value2"; "This is a structured critical message");
/// let map: std::collections::HashMap<&str, &str> = std::collections::HashMap::from([("key1", "value1"), ("key2", "value2")]);
/// crit!(map; "This is also a structured critical message");
/// ```
///
/// This macro is intended for critical errors that require immediate attention. Logging at this level typically indicates a serious failure in a component of the application.
#[macro_export]
macro_rules! crit {
    ($arg1:expr; $($arg2:tt)+) => {{
        $crate::structured_log($crate::Level::Critical, module_path!(), format_args!($($arg2)+), $arg1)
    }};
    ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
        let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
        $(
            let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
            map.insert(key, $($value)*);
        )*
        $crate::structured_log($crate::Level::Critical, module_path!(), format_args!($($args)*), map)
    }};
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
    // ($arg1:expr; $($arg2:tt)+) => {{
    //     $crate::structured_log($crate::Level::Critical, module_path!(), format_args!($($arg2)+), $arg1)
    // }};
    // ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
    //     let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
    //     $(
    //         let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
    //         map.insert(key, $($value)*);
    //     )*
    //     $crate::structured_log($crate::Level::Critical, module_path!(), format_args!($($args)*), map)
    // }};
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
    // ($arg1:expr; $($arg2:tt)+) => {{
    //     $crate::structured_log($crate::Level::Critical, module_path!(), format_args!($($arg2)+), $arg1)
    // }};
    // ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
    //     let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
    //     $(
    //         let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
    //         map.insert(key, $($value)*);
    //     )*
    //     $crate::structured_log($crate::Level::Critical, module_path!(), format_args!($($args)*), map)
    // }};
    ($($arg:tt)*) => {{
        $crate::result_log($crate::Level::Critical, module_path!(), format_args!($($arg)*))
    }};
}

// ##################################################################### FATAL #####################################################################
/// Logs a fatal message.
///
/// # Example
///
/// ``` no_run
/// use logfather::fatal;
///
/// fatal!("This is a normal fatal message");
/// fatal!("This is a normal {} message", "fatal");
/// fatal!("key1" = "value1", "key2" = "value2"; "This is a structured fatal message");
/// let map: std::collections::HashMap<&str, &str> = std::collections::HashMap::from([("key1", "value1"), ("key2", "value2")]);
/// fatal!(map; "This is also a structured fatal message");
/// ```
///
/// This macro is intended for critical errors that require immediate attention. Logging at this level typically indicates a serious failure in a component of the application.
#[macro_export]
macro_rules! fatal {
    ($arg1:expr; $($arg2:tt)+) => {{
        $crate::structured_log($crate::Level::Fatal, module_path!(), format_args!($($arg2)+), $arg1)
    }};
    ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
        let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
        $(
            let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
            map.insert(key, $($value)*);
        )*
        $crate::structured_log($crate::Level::Fatal, module_path!(), format_args!($($args)*), map)
    }};
    ($($arg:tt)*) => {{
        $crate::log($crate::Level::Fatal, module_path!(), format_args!($($arg)*))
    }};
}

/// Logs a fatal message.
///
/// # Example
///
/// ``` no_run
/// use logfather::r_fatal;
///
/// let result = r_fatal!("This is a fatal message");
/// if result.is_err() {
///     println!("The log failed.");
/// }
/// ```
///
/// This macro is intended for critical errors that require immediate attention. Logging at this level typically indicates a serious failure in a component of the application.
#[macro_export]
macro_rules! r_fatal {
    // ($arg1:expr; $($arg2:tt)+) => {{
    //     $crate::structured_log($crate::Level::Fatal, module_path!(), format_args!($($arg2)+), $arg1)
    // }};
    // ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
    //     let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
    //     $(
    //         let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
    //         map.insert(key, $($value)*);
    //     )*
    //     $crate::structured_log($crate::Level::Fatal, module_path!(), format_args!($($args)*), map)
    // }};
    ($($arg:tt)*) => {{
        $crate::result_log($crate::Level::Fatal, module_path!(), format_args!($($arg)*))
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
/// diagnostic!("This is a normal diagnostic message");
/// diagnostic!("This is a normal {} message", "diagnostic");
/// diagnostic!("key1" = "value1", "key2" = "value2"; "This is a structured diagnostic message");
/// let map: std::collections::HashMap<&str, &str> = std::collections::HashMap::from([("key1", "value1"), ("key2", "value2")]);
/// diagnostic!(map; "This is also a structured diagnostic message");
/// ```
#[macro_export]
macro_rules! diagnostic {
    ($arg1:expr; $($arg2:tt)+) => {{
        #[cfg(debug_assertions)]
        {
            $crate::structured_log($crate::Level::Diagnostic, module_path!(), format_args!($($arg2)+), $arg1)
        }
    }};
    ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
        #[cfg(debug_assertions)]
        {
            let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
            $(
                let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
                map.insert(key, $($value)*);
            )*
            $crate::structured_log($crate::Level::Diagnostic, module_path!(), format_args!($($args)*), map)
        }
    }};
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
/// diag!("This is a normal diagnostic message");
/// diag!("This is a normal {} message", "diagnostic");
/// diag!("key1" = "value1", "key2" = "value2"; "This is a structured diagnostic message");
/// let map: std::collections::HashMap<&str, &str> = std::collections::HashMap::from([("key1", "value1"), ("key2", "value2")]);
/// diag!(map; "This is also a structured diagnostic message");
/// ```
#[macro_export]
macro_rules! diag {
    ($arg1:expr; $($arg2:tt)+) => {{
        #[cfg(debug_assertions)]
        {
            $crate::structured_log($crate::Level::Diagnostic, module_path!(), format_args!($($arg2)+), $arg1)
        }
    }};
    ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
        #[cfg(debug_assertions)]
        {
            let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
            $(
                let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
                map.insert(key, $($value)*);
            )*
            $crate::structured_log($crate::Level::Diagnostic, module_path!(), format_args!($($args)*), map)
        }
    }};
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
    // ($arg1:expr; $($arg2:tt)+) => {{
    //     #[cfg(debug_assertions)]
    //     {
    //         $crate::structured_log($crate::Level::Diagnostic, module_path!(), format_args!($($arg2)+), $arg1)
    //     }
    //     #[cfg(not(debug_assertions))]
    //     {
    //         Ok::<(), LogfatherError>(())
    //     }
    // }};
    // ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
    //     #[cfg(debug_assertions)]
    //     {
    //         let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
    //         $(
    //             let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
    //             map.insert(key, $($value)*);
    //         )*
    //         $crate::structured_log($crate::Level::Diagnostic, module_path!(), format_args!($($args)*), map)
    //     }
    //     #[cfg(not(debug_assertions))]
    //     {
    //         Ok::<(), LogfatherError>(())
    //     }
    // }};
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
    // ($arg1:expr; $($arg2:tt)+) => {{
    //     #[cfg(debug_assertions)]
    //     {
    //         $crate::structured_log($crate::Level::Diagnostic, module_path!(), format_args!($($arg2)+), $arg1)
    //     }
    //     #[cfg(not(debug_assertions))]
    //     {
    //         Ok::<(), LogfatherError>(())
    //     }
    // }};
    // ($($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($args:tt)*) => {{
    //     #[cfg(debug_assertions)]
    //     {
    //         let mut map: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
    //         $(
    //             let key = concat!(stringify!($key), $(concat!(":", stringify!($capture)))?);
    //             map.insert(key, $($value)*);
    //         )*
    //         $crate::structured_log($crate::Level::Diagnostic, module_path!(), format_args!($($args)*), map)
    //     }
    //     #[cfg(not(debug_assertions))]
    //     {
    //         Ok::<(), LogfatherError>(())
    //     }
    // }};
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