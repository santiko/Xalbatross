use std::str::FromStr;
use std::fmt;
use std::env;


/// A user agent string.
///
/// Although you can use custom ones, it's recommended to use one provided by default:
///
/// ```
/// use nimiq_lib2::config::user_agent::UserAgent;
/// let user_agent = UserAgent::default();
/// ```
///
/// An example of such a user agent: `core-rs-albatross/0.1.0 (native; linux x86_64)`
///
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UserAgent(String);

impl FromStr for UserAgent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(UserAgent(s.to_string()))
    }
}

impl From<String> for UserAgent {
    fn from(s: String) -> Self {
        UserAgent(s)
    }
}

impl fmt::Display for UserAgent {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl Default for UserAgent {
    fn default() -> Self {
        format!("core-rs-albatross/{} (native; {} {})", env!("CARGO_PKG_VERSION"), env::consts::OS, env::consts::ARCH).into()
    }
}
