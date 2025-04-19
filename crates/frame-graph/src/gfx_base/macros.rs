#[macro_export]
macro_rules! define_gfx_type {
    ($gfx_type:ident,  $gfx_type_trait: ident, $erased_gfx_type_trait:ident) => {
        use downcast_rs::impl_downcast;

        pub struct $gfx_type {
            value: Box<dyn $erased_gfx_type_trait>,
        }

        impl_downcast!($erased_gfx_type_trait);

        impl $gfx_type {
            pub fn new<T: $gfx_type_trait>(value: T) -> Self {
                $gfx_type {
                    value: Box::new(value),
                }
            }

            pub fn downcast_ref<T: $gfx_type_trait>(&self) -> Option<&T> {
                self.value.downcast_ref()
            }

            pub fn downcast<T: $gfx_type_trait>(self) -> Option<Box<T>> {
                self.value.downcast().ok()
            }
        }
    };
}

#[macro_export]
macro_rules! define_gfx_type_with_desc {
    ($gfx_type:ident, $gfx_desc:ident, $gfx_type_trait: ident, $erased_gfx_type_trait:ident) => {
        use downcast_rs::impl_downcast;

        pub struct $gfx_type {
            value: Box<dyn $erased_gfx_type_trait>,
            desc: $gfx_desc,
        }

        impl_downcast!($erased_gfx_type_trait);

        impl $gfx_type {
            pub fn new<T: $gfx_type_trait>(value: T, desc: $gfx_desc) -> Self {
                $gfx_type {
                    value: Box::new(value),
                    desc,
                }
            }

            pub fn downcast_ref<T: $gfx_type_trait>(&self) -> Option<&T> {
                self.value.downcast_ref()
            }

            pub fn downcast<T: $gfx_type_trait>(self) -> Option<Box<T>> {
                self.value.downcast().ok()
            }

            pub fn get_desc(&self) -> &$gfx_desc {
                &self.desc
            }
        }
    };
}
