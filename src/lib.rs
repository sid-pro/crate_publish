
//Now we see to upload our code to crates.io

// first write the documentation comments so other people can understand how to use the code 
// the documentation comments are starts with /// and use markdown for formatting and //! for describing crates and modules

// Documentation comments within items are useful for describing crates and modules especially.
// Use them to explain the overall purpose of the container to help your users understand the crate’s organization.

// currently commenting this till line 34 we can uncomment and see the output cargo doc --open
/* 
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Add one to the number given
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = crate_publish::add_one(arg);
/// 
/// assert_eq!(answer,6);
/// ```

// we can build the HTML documentation of our crate by using command cargo doc --open
// pub fn add_one(x:i32)->i32{
//     x+1
// }

// if we run cargo test than the example written in documentation will run
*/

// we have a library called art conatins two module kinds and utils
// An art library with items organized into kinds and utils modules

//! Art
//! 
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;   // now in doc the enums come at top level
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;


pub mod kinds{
     /// The primary colors according to the RYB color model.
    pub enum PrimaryColor{
        Red,
        Blue,
        Green,
    }
    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils{
    use crate::kinds::*;
     /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1:PrimaryColor,c2:PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}