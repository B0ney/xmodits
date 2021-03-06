#[macro_export]
macro_rules! word {
    ($i:expr) => {
        $i..$i + 2
    };
}

#[macro_export]
macro_rules! dword {
    ($i:expr ) => {
        $i..$i + 4
    };
}

#[macro_export]
macro_rules! long {
    ($i:expr) => {
        $i..$i + 8
    };
}

#[macro_export]
/// chars!(offset, length)
/// 
/// Use read_chars() helper function.
macro_rules! chars  {
    ($i:expr, $e:expr) => {$i..$i + $e};
}

#[macro_export]
/// chars!(offset, length)
macro_rules! slice  {
    ($i:expr, $e:expr) => {$i..$i + $e};
}