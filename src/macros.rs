#[macro_export]
macro_rules! config_to_driver {
    (
        $(#[$enum_meta:meta])*
         $vis:vis enum $name:ident {
            $(
                $(#[$meta:meta])*
                $variant:ident($type:ty) -> $messaging:ty
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
            pub async fn get_driver_config(&self) -> anyhow::Result<Box<dyn ErasedMessagingDriver>> {
                match self.clone() {
                    $(Self::$variant (config) => match <$messaging>::new(config.clone()).await {
                        Ok(driver_config) => Ok(Box::new(driver_config)),
                        Err(err) => Err(err.into()),
                    }),*
                }
            }
        }
    };
}