use std::{cmp, error, fmt, str::FromStr};

use serde::{Deserialize, Serialize};

static LOG_LEVEL_NAMES: [&str; 6] = ["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"];

static LEVEL_PARSE_ERROR: &str =
    "attempted to convert a string that doesn't match an existing log level";

/// An enum representing the available verbosity levels of the logger.
///
/// Typical usage includes: checking if a certain `Level` is enabled with
/// [`log!`](super::log), and comparing a [`Level`] directly to a
/// [`LevelFilter`](LevelFilter).
#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub enum Level {
    /// The "error" level.
    ///
    /// Designates very serious errors.
    // This way these line up with the discriminants for [LevelFilter] below
    // This works because Rust treats field-less enums the same way as C does:
    // https://doc.rust-lang.org/reference/items/enumerations.html#custom-discriminant-values-for-field-less-enumerations
    Error = 1,
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    Warn,
    /// The "info" level.
    ///
    /// Designates useful information.
    Info,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug,
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace,
}

impl PartialEq<LevelFilter> for Level {
    #[inline]
    fn eq(&self, other: &LevelFilter) -> bool {
        *self as usize == *other as usize
    }
}

impl PartialOrd for Level {
    #[inline]
    fn partial_cmp(&self, other: &Level) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }

    #[inline]
    fn lt(&self, other: &Level) -> bool {
        (*self as usize) < *other as usize
    }

    #[inline]
    fn le(&self, other: &Level) -> bool {
        *self as usize <= *other as usize
    }

    #[inline]
    fn gt(&self, other: &Level) -> bool {
        *self as usize > *other as usize
    }

    #[inline]
    fn ge(&self, other: &Level) -> bool {
        *self as usize >= *other as usize
    }
}

impl PartialOrd<LevelFilter> for Level {
    #[inline]
    fn partial_cmp(&self, other: &LevelFilter) -> Option<cmp::Ordering> {
        Some((*self as usize).cmp(&(*other as usize)))
    }

    #[inline]
    fn lt(&self, other: &LevelFilter) -> bool {
        (*self as usize) < *other as usize
    }

    #[inline]
    fn le(&self, other: &LevelFilter) -> bool {
        *self as usize <= *other as usize
    }

    #[inline]
    fn gt(&self, other: &LevelFilter) -> bool {
        *self as usize > *other as usize
    }

    #[inline]
    fn ge(&self, other: &LevelFilter) -> bool {
        *self as usize >= *other as usize
    }
}

impl Ord for Level {
    #[inline]
    fn cmp(&self, other: &Level) -> cmp::Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
}

// Reimplemented here because std::ascii is not available in libcore
fn eq_ignore_ascii_case(a: &str, b: &str) -> bool {
    fn to_ascii_uppercase(c: u8) -> u8 {
        if (b'a'..=b'z').contains(&c) {
            c - b'a' + b'A'
        } else {
            c
        }
    }

    if a.len() == b.len() {
        a.bytes()
            .zip(b.bytes())
            .all(|(a, b)| to_ascii_uppercase(a) == to_ascii_uppercase(b))
    } else {
        false
    }
}

impl FromStr for Level {
    type Err = ParseLevelError;
    fn from_str(level: &str) -> Result<Level, Self::Err> {
        LOG_LEVEL_NAMES
            .iter()
            .position(|&name| eq_ignore_ascii_case(name, level))
            .into_iter()
            .filter(|&idx| idx != 0)
            .map(|idx| Level::from_usize(idx).unwrap())
            .next()
            .ok_or(ParseLevelError(()))
    }
}

impl fmt::Display for Level {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad(self.as_str())
    }
}

impl Level {
    fn from_usize(u: usize) -> Option<Level> {
        match u {
            1 => Some(Level::Error),
            2 => Some(Level::Warn),
            3 => Some(Level::Info),
            4 => Some(Level::Debug),
            5 => Some(Level::Trace),
            _ => None,
        }
    }

    /// Returns the most verbose logging level.
    #[inline]
    pub fn max() -> Level {
        Level::Trace
    }

    /// Converts the [`Level`] to the equivalent [`LevelFilter`].
    #[inline]
    pub fn to_level_filter(&self) -> LevelFilter {
        LevelFilter::from_usize(*self as usize).unwrap()
    }

    /// Returns the string representation of the [`Level`].
    ///
    /// This returns the same string as the [`fmt::Display`] implementation.
    pub fn as_str(&self) -> &'static str {
        LOG_LEVEL_NAMES[*self as usize]
    }

    /// Iterate through all supported logging levels.
    ///
    /// The order of iteration is from more severe to less severe log messages.
    ///
    /// # Examples
    ///
    /// ```
    /// use lunatic_log::Level;
    ///
    /// let mut levels = Level::iter();
    ///
    /// assert_eq!(Some(Level::Error), levels.next());
    /// assert_eq!(Some(Level::Trace), levels.last());
    /// ```
    pub fn iter() -> impl Iterator<Item = Self> {
        (1..6).map(|i| Self::from_usize(i).unwrap())
    }
}

/// An enum representing the available verbosity level filters of the logger.
///
/// A [`LevelFilter`] may be compared directly to a [`Level`].
#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub enum LevelFilter {
    /// A level lower than all log levels.
    Off,
    /// Corresponds to the `Error` log level.
    Error,
    /// Corresponds to the `Warn` log level.
    Warn,
    /// Corresponds to the `Info` log level.
    Info,
    /// Corresponds to the `Debug` log level.
    Debug,
    /// Corresponds to the `Trace` log level.
    Trace,
}

impl PartialEq<Level> for LevelFilter {
    #[inline]
    fn eq(&self, other: &Level) -> bool {
        other.eq(self)
    }
}

impl PartialOrd for LevelFilter {
    #[inline]
    fn partial_cmp(&self, other: &LevelFilter) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }

    #[inline]
    fn lt(&self, other: &LevelFilter) -> bool {
        (*self as usize) < *other as usize
    }

    #[inline]
    fn le(&self, other: &LevelFilter) -> bool {
        *self as usize <= *other as usize
    }

    #[inline]
    fn gt(&self, other: &LevelFilter) -> bool {
        *self as usize > *other as usize
    }

    #[inline]
    fn ge(&self, other: &LevelFilter) -> bool {
        *self as usize >= *other as usize
    }
}

impl PartialOrd<Level> for LevelFilter {
    #[inline]
    fn partial_cmp(&self, other: &Level) -> Option<cmp::Ordering> {
        Some((*self as usize).cmp(&(*other as usize)))
    }

    #[inline]
    fn lt(&self, other: &Level) -> bool {
        (*self as usize) < *other as usize
    }

    #[inline]
    fn le(&self, other: &Level) -> bool {
        *self as usize <= *other as usize
    }

    #[inline]
    fn gt(&self, other: &Level) -> bool {
        *self as usize > *other as usize
    }

    #[inline]
    fn ge(&self, other: &Level) -> bool {
        *self as usize >= *other as usize
    }
}

impl Ord for LevelFilter {
    #[inline]
    fn cmp(&self, other: &LevelFilter) -> cmp::Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
}

impl FromStr for LevelFilter {
    type Err = ParseLevelError;
    fn from_str(level: &str) -> Result<LevelFilter, Self::Err> {
        LOG_LEVEL_NAMES
            .iter()
            .position(|&name| eq_ignore_ascii_case(name, level))
            .map(|p| LevelFilter::from_usize(p).unwrap())
            .ok_or(ParseLevelError(()))
    }
}

impl fmt::Display for LevelFilter {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad(self.as_str())
    }
}

impl LevelFilter {
    fn from_usize(u: usize) -> Option<LevelFilter> {
        match u {
            0 => Some(LevelFilter::Off),
            1 => Some(LevelFilter::Error),
            2 => Some(LevelFilter::Warn),
            3 => Some(LevelFilter::Info),
            4 => Some(LevelFilter::Debug),
            5 => Some(LevelFilter::Trace),
            _ => None,
        }
    }

    /// Returns the most verbose logging level filter.
    #[inline]
    pub fn max() -> LevelFilter {
        LevelFilter::Trace
    }

    /// Converts `self` to the equivalent [`Level`].
    ///
    /// Returns [`None`] if `self` is [`LevelFilter::Off`].
    #[inline]
    pub fn to_level(&self) -> Option<Level> {
        Level::from_usize(*self as usize)
    }

    /// Returns the string representation of the [`LevelFilter`].
    ///
    /// This returns the same string as the [`fmt::Display`] implementation.
    pub fn as_str(&self) -> &'static str {
        LOG_LEVEL_NAMES[*self as usize]
    }

    /// Iterate through all supported filtering levels.
    ///
    /// The order of iteration is from less to more verbose filtering.
    ///
    /// # Examples
    ///
    /// ```
    /// use lunatic_log::LevelFilter;
    ///
    /// let mut levels = LevelFilter::iter();
    ///
    /// assert_eq!(Some(LevelFilter::Off), levels.next());
    /// assert_eq!(Some(LevelFilter::Trace), levels.last());
    /// ```
    pub fn iter() -> impl Iterator<Item = Self> {
        (0..6).map(|i| Self::from_usize(i).unwrap())
    }
}

/// The type returned by [`from_str`] when the string doesn't match any of the log levels.
///
/// [`from_str`]: https://doc.rust-lang.org/std/str/trait.FromStr.html#tymethod.from_str
#[allow(missing_copy_implementations)]
#[derive(Debug, PartialEq)]
pub struct ParseLevelError(());

impl fmt::Display for ParseLevelError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(LEVEL_PARSE_ERROR)
    }
}

// The Error trait is not available in libcore
impl error::Error for ParseLevelError {}
