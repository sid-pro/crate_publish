// In Rust, release profiles are predefined and customizable profiles with different
//  configurations that allow a programmer to have more control over various options for compiling code.
//   Each profile is configured independently of the others.

// Cargo has two main profiles: the dev profile Cargo uses when you run cargo build and the release profile
// Cargo uses when you run cargo build --release. The dev profile is defined with good defaults for development,
// and the release profile has good defaults for release builds.

// dev build are unoptimized and take less time to compile whereas the release profile are optimized and take longer time to compile
// we can change this default setting from cargo.toml file
// we have define optimization-level for dev and release profiles

use siddharth_art::PrimaryColor;
use siddharth_art::mix;

fn main(){
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Blue;
    mix(red, yellow);
}