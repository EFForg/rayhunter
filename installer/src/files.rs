#[cfg(debug_assertions)]
macro_rules! get_file {
    ($var:literal) => {{
        let path = env!($var);
        match ::std::fs::read(path) {
            Ok(bytes) => bytes.leak() as &'static [u8],
            Err(e) => panic!("Failed to read file for {}: {}", $var, e),
        }
    }};
}

#[cfg(not(debug_assertions))]
macro_rules! get_file {
    ($var:literal) => {{
        const _: () = assert!(
            !env!($var).is_empty(),
            concat!($var, " was not bundled at build time"),
        );
        include_bytes!(env!($var)) as &'static [u8]
    }};
}

pub(crate) use get_file;
