#[macro_export]
macro_rules! slc {
    ($buf:ident, $offset:expr, $len:expr) => {
        $buf[$offset..($offset + $len)]
    };
    ($buf:ident, $offset:expr, $len:expr, $t:ty) => {
        <$t>::from_be_bytes(slc!($buf, $offset, $len).try_into()?)
    };
}
