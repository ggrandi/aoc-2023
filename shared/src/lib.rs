pub fn char_to_usize(c: char) -> usize {
    ((c as u8) - b'0') as usize
}

#[macro_export]
macro_rules! dprintln {
    ($($arg:tt)*) => {
    #[cfg(debug_assertions)]
    { println!($($arg)*);}};
}
