#[macro_export]
macro_rules! runtime {
    (
        $($pallet_name:ident),*
    ) => {
        pub struct Runtime;

        impl Runtime {
            pub fn dispatch(call: Call) -> Result<(), String> {
                match call {
                    $(
                        Call::$pallet_name(call) => $pallet_name::dispatch(call),
                    )*
                }
            }
        }

        #[derive(Debug)]
        pub enum Call {
            $(
                $pallet_name($pallet_name::Call),
            )*
        }
    };
}
