#[macro_export]
macro_rules! config_to_driver {
    (
        $(#[$enum_meta:meta])*
         $vis:vis enum $name:ident {
            $(
                $(#[$meta:meta])*
                $variant:ident ($type:ty) -> $messaging:ty, $(,)?
            ),* $(,)?
        }
    ) => {
        use crate::models::driver::ErasedMessagingDriver;

        $(#[$enum_meta])*
        $vis enum $name {
            $(
                $(#[$meta])*
                $variant ($type),
            )*
        }

        impl $name {
            pub fn get_driver_config(&self) -> Box<dyn ErasedMessagingDriver> {
                match self.clone() {
                    $(Self::$variant (config) => Box::new(<$messaging>::new(config.clone()))),*
                }
            }
        }
    };
}