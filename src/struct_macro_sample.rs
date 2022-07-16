//
// Handle struct instance in more flexible way
//
#[macro_export]
macro_rules! assign_struct_field {
    ($struct: item, $field_name: ident >>> $field_type: ty) => {
        let $field_name = $field_type;
    };

    () => {

    };
}

pub trait Login {
    fn login(&self, user_name: &str, password: &str) -> bool;
}

#[macro_export]
macro_rules! impl_login {

    ($target_struct: ty) => {{
        impl crate::struct_macro_sample::Login for $target_struct {
            fn login(&self, user_name: &str, password: &str) -> bool {
                // Sample logic
                user_name.trim() == password.trim()
            }
        }
    }};
}
