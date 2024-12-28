#[macro_export]
macro_rules! call {
    (
        $pallet_name:ident,
        $(
            $fn_name:ident ($($arg_name:ident: $arg_type:ty),*) -> $ret_type:ty
        ),*
    ) => {
        #[derive(Debug)]
        pub enum Call {
            $(
                $fn_name {
                    $($arg_name: $arg_type),*
                },
            )*
        }

        impl $pallet_name {
            pub fn dispatch(call: Call) -> Result<(), String> {
                match call {
                    $(
                        Call::$fn_name { $($arg_name),* } => Self::$fn_name($($arg_name),*),
                    )*
                }
            }
        }
    };
}
