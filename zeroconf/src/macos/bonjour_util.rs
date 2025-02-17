//! Utilities related to Bonjour

use super::constants;
use crate::NetworkInterface;
use bonjour_sys::DNSServiceErrorType;

/// Normalizes the specified domain `&str` to conform to a standard enforced by this crate.
///
/// Bonjour suffixes domains with a final `'.'` character in some contexts but is not required by
/// the standard. This function removes the final dot if present.
pub fn normalize_domain(domain: &str) -> String {
    if domain.chars().nth(domain.len() - 1).unwrap() == '.' {
        String::from(&domain[..domain.len() - 1])
    } else {
        String::from(domain)
    }
}

/// Converts the specified [`NetworkInterface`] to the Bonjour expected value.
///
/// [`NetworkInterface`]: ../../enum.NetworkInterface.html
pub fn interface_index(interface: NetworkInterface) -> u32 {
    match interface {
        NetworkInterface::Unspec => constants::BONJOUR_IF_UNSPEC,
        NetworkInterface::AtIndex(i) => i,
    }
}

/// Executes the specified closure and returns a formatted `Result`
pub fn sys_exec<F: FnOnce() -> DNSServiceErrorType>(func: F, message: &str) -> crate::Result<()> {
    let err = func();

    if err < 0 {
        crate::Result::Err(format!("{} (code: {})", message, err).into())
    } else {
        crate::Result::Ok(())
    }
}
