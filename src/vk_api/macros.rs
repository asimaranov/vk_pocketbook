

#[macro_export]
macro_rules! vk_args {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(vk_args!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { vk_args!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = vk_args!(@count $($key),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key.to_string(), $value.get_enum_type());
            )*
            _map
        }
    };
}
