pub mod macros {
    #[macro_export]
    macro_rules! offset_u16 {
        ($i:expr) => {
            $i..$i + 2
            // $i
        };
    }

    #[macro_export]
    macro_rules! offset_u32 {
        ($i:expr ) => {
            $i..$i + 4
            // $i

        };
    }

    #[macro_export]
    macro_rules! offset_u64 {
        ($i:expr) => {
            $i..$i + 8
            // $i

        };
    }
    #[macro_export]
    macro_rules! offset_chars  {
        ($i:expr, $e:expr) => {$i..$i + $e};
    }
}