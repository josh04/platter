use serde::{Deserialize, Serialize};
use std::{
    future::Future,
    io::Error as IOError,
    path::Path
};

mod error;
pub use self::error::SaveError;

/// Create a Future that loads a file into an owned Vec of bytes
///
/// It exists for loading files from the server with Javascript on the web, and providing a unified
/// API between desktop and the web when it comes to file loading
pub fn load_file(path: impl AsRef<Path>) -> impl Future<Output = Result<Vec<u8>, IOError>> {
    platform::load_file(path)
}

/// Where the data should be written to and read from
///
/// On desktop this determines which folder the file should be placed in (adhering to the XDG
/// desktop specification), and on web it determines which various web storage APIs it should use.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Location {
    /// Cache should be used for extremely short-lived data
    ///
    /// Cached data has no lifetime guarantee, and should be expected to be cleared between runs of
    /// the program. On web, it is guaranteed when the user leaves the application and returns that
    /// the cache data will have been cleared.
    Cache,
    /// Config should store application behavior configs, and will be long-lived
    Config,
    /// Data will store application data, and will be long-lived
    Data
}

/// Save some arbitrary data to the given location using Serde
///
/// Different platforms may have different save locations: on the Web, data is saved in local
/// storage, on the desktop, it is stored in some appropriate home-directory folder.
///
/// The appname should be some constant; this is used to name the file to place the data in on
/// desktop platforms. The profile should allow different things to save for the same app, such as
/// save for different players in a game.
///
/// The example shows how to round-trip some data. Note that for [load](fn.load.html) you must
/// explicitly specify the type of the data; this is because the struct is not passed as a
/// parameter to `load` so Rust cannot infer the type.
///
/// ```
/// use quick_fs::{Location, save, load};
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize)]
/// struct Player {
///     name: String,
///     score: u32
/// }
///
/// let player1 = Player { name: "Bob".to_string(), score: 21 };
/// save(Location::Cache, "mygame", "player1", &player1).expect("Could not save Player 1");
///
/// let player2 = Player { name: "Alice".to_string(), score: 200 };
/// save(Location::Cache, "mygame", "player2", &player2).expect("Could not save Player 2");
///
/// // Now reload.
/// let player1 = load::<Player>(Location::Cache, "mygame", "player1").expect("Could not load Player 1");
/// let player2 = load::<Player>(Location::Cache, "mygame", "player2").expect("Could not load Player 2");
/// ```
pub fn save<T: Serialize>(location: Location, appname: &str, profile: &str, data: &T)
        -> Result<(), SaveError> {
    platform::save(location, appname, profile, data)
}

/// Save some raw bytes to the given profile
///
/// Different platforms may have different save locations: on the Web, data is saved in local
/// storage, on the desktop, it is stored in some appropriate home-directory folder.
///
/// The appname should be some constant; this is used to name the file to place the data in on
/// desktop platforms. The profile should allow different things to save for the same app, such as
/// save for different players in a game.
pub fn save_raw(location: Location, appname: &str, profile: &str, data: &[u8]) -> Result<(), SaveError> {
    platform::save_raw(location, appname, profile, data)
}

/// Load some data from the given profile using Serde
///
/// Different platforms may have different save locations: on the Web, data is saved in local
/// storage, on the desktop, it is stored in some appropriate home-directory folder.
///
/// See [save](fn.save.html) for an example of saving and then loading some data.
pub fn load<T>(location: Location, appname: &str, profile: &str) -> Result<T, SaveError>
        where for<'de> T: Deserialize<'de> {
    platform::load(location, appname, profile)
}

/// Load some raw bytes from the given profile
///
/// Different platforms may have different save locations: on the Web, data is saved in local
/// storage, on the desktop, it is stored in some appropriate home-directory folder.
pub fn load_raw(location: Location, appname: &str, profile: &str) -> Result<Vec<u8>, SaveError> {
    platform::load_raw(location, appname, profile)
}


// Select which platform implementation to use based on provided features

#[cfg(not(target_arch = "wasm32"))]
#[path = "desktop.rs"]
mod platform;

#[cfg(target_arch = "wasm32")]
#[path = "web.rs"]
mod platform;
