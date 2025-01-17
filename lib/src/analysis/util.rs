macro_rules! unpack {
    ($pat:pat = $val:expr) => {
        let $pat = $val else { return None; };
    };
}

pub(crate) use unpack;
