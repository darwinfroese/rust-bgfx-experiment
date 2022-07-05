#[macro_export]
macro_rules! eng_trace {
    ($x: expr) => {{
        tracing::trace!(target:"engine", $x);
    }}
}

#[macro_export]
macro_rules! eng_debug{
    ($x: expr) => {{
        tracing::debug!(target:"engine", $x);
    }}
}

#[macro_export]
macro_rules! eng_info{
    ($x: expr) => {{
        tracing::info!(target:"engine", $x);
    }}
}

#[macro_export]
macro_rules! eng_warn{
    ($x: expr) => {{
        tracing::warn!(target:"engine", $x);
    }}
}

#[macro_export]
macro_rules! eng_error{
    ($x: expr) => {{
        tracing::error!(target:"engine", $x);
    }}
}

#[macro_export]
macro_rules! app_trace {
    ($x: expr) => {{
        tracing::trace!(target:"application", $x);
    }}
}

#[macro_export]
macro_rules! app_debug{
    ($x: expr) => {{
        tracing::debug!(target:"application", $x);
    }}
}

#[macro_export]
macro_rules! app_info{
    ($x: expr) => {{
        tracing::info!(target:"application", $x);
    }}
}

#[macro_export]
macro_rules! app_warn{
    ($x: expr) => {{
        tracing::warn!(target:"application", $x);
    }}
}

#[macro_export]
macro_rules! app_error{
    ($x: expr) => {{
        tracing::error!(target:"application", $x);
    }}
}
