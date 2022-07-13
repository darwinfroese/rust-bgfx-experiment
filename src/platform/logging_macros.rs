// TODO: Empty macros in non-debug
#[macro_export]
macro_rules! eng_trace {
    ($($args: expr), *) => {{
        let mut message: String = String::from("");
        $(
            let tempstr: String = format!("{}", format_args!("{}", $args));
            message.push_str(&tempstr[..]);
        )*

        tracing::trace!(target:"engine", message);
    }}
}

#[macro_export]
macro_rules! eng_debug{
    ($($args: expr), *) => {{
        let mut message: String = String::from("");
        $(
            let tempstr: String = format!("{}", format_args!("{}", $args));
            message.push_str(&tempstr[..]);
        )*

        tracing::debug!(target:"engine", message);
    }}
}

#[macro_export]
macro_rules! eng_info{
    ($($args: expr), *) => {{
        let mut message: String = String::from("");
        $(
            let tempstr: String = format!("{}", format_args!("{}", $args));
            message.push_str(&tempstr[..]);
        )*

        tracing::info!(target:"engine", message);
    }}
}

#[macro_export]
macro_rules! eng_warn{
    ($($args: expr), *) => {{
        let mut message: String = String::from("");
        $(
            let tempstr: String = format!("{}", format_args!("{}", $args));
            message.push_str(&tempstr[..]);
        )*

        tracing::warn!(target:"engine", message);
    }}
}

#[macro_export]
macro_rules! eng_error{
    ($($args: expr), *) => {{
        let mut message: String = String::from("");
        $(
            let tempstr: String = format!("{}", format_args!("{}", $args));
            message.push_str(&tempstr[..]);
        )*

        tracing::error!(target:"engine", message);
    }}
}

#[macro_export]
macro_rules! app_trace {
    ($($args: expr), *) => {{
        let mut message: String = String::from("");
        $(
            let tempstr: String = format!("{}", format_args!("{}", $args));
            message.push_str(&tempstr[..]);
        )*

        tracing::trace!(target:"application", message);
    }}
}

#[macro_export]
macro_rules! app_debug{
    ($($args: expr), *) => {{
        let mut message: String = String::from("");
        $(
            let tempstr: String = format!("{}", format_args!("{}", $args));
            message.push_str(&tempstr[..]);
        )*

        tracing::debug!(target:"application", message);
    }}
}

#[macro_export]
macro_rules! app_info{
    ($($args: expr), *) => {{
        let mut message: String = String::from("");
        $(
            let tempstr: String = format!("{}", format_args!("{}", $args));
            message.push_str(&tempstr[..]);
        )*

        tracing::info!(target:"application", message);
    }}
}

#[macro_export]
macro_rules! app_warn{
    ($($args: expr), *) => {{
        let mut message: String = String::from("");
        $(
            let tempstr: String = format!("{}", format_args!("{}", $args));
            message.push_str(&tempstr[..]);
        )*

        tracing::warn!(target:"application", message);
    }}
}

#[macro_export]
macro_rules! app_error{
    ($($args: expr), *) => {{
        let mut message: String = String::from("");
        $(
            let tempstr: String = format!("{}", format_args!("{}", $args));
            message.push_str(&tempstr[..]);
        )*

        tracing::error!(target:"application", message);
    }}
}
