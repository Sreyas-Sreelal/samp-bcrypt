#[macro_export]
macro_rules! exec_public_with_name {
    ($amx:ident, $name:ident; $($args:tt)*) => {
        {
            $amx.find_public(&$name)
                .and_then(|index| exec!($amx, index; $($args)*))
        }
    };
}
