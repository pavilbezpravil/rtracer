#![feature(duration_float)]

pub mod renderer;
pub mod frame_counter;
pub mod testbed;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
