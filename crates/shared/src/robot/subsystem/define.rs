// Macro to define a subsystem:
//  generates: struct, new(), and Subsystem trait
#[macro_export]
macro_rules! define_subsystem {
    ($struct_name:ident, $subsystem_name:expr, { $($field_name:ident : $field_ty:ty),* $(,)? }) => {
        // Generate struct
        pub struct $struct_name {
            base: Base,
            $($field_name: $field_ty),*
        }

        // Public constructor: takes only Base, binds it, then calls init()
        impl $struct_name {
            pub fn new(base: &Base) -> Self {
                let bound_base = base.bind::<Self>();
                Self::init(bound_base)
            }

            pub fn telemetry(&self) -> &telemetry::Publisher {
                &self.base.telemetry()
            }
        }

        // Subsystem trait implementation
        impl Subsystem for $struct_name {
            const NAME: &'static str = $subsystem_name;

            fn telemetry(&self) -> &telemetry::Publisher {
                &self.base.telemetry()
            }
        }
    };
}