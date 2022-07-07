//! # My Crate
//!
//! `my_crate` is a mog's nan
pub use self::moggers::MoggersNans;


/// Add left and right
/// # Examples
/// ```
/// let answer = my_crate::add(1, 2);
/// assert_eq!(3, answer);
/// ```
/// # Mog's nan
/// is gae
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod moggers {
    pub enum MoggersNans {
        Gae,
        Moggranny,
        Mognanny,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
