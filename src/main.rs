mod basic_macro;
mod struct_macro_sample;

use struct_macro_sample::Login;

//
// About the macro, you can think like this:
//
// Macro just the source code that produce another source code!
//
// It looks a function but more flexible:)
//
fn main() {
    //
    // Use `cargo watch -c --exec "expand"` to see the macro expands result:)
    //

    let empty_string_arr = my_vec!(String);
    let empty_usize_arr = my_vec!(usize);

    let one_element_arr = my_vec!(45, usize);
    println!("one_element_arr: {one_element_arr:#?}");

    let one_element_arr_2 = my_vec!("Wison", &str);
    println!("one_element_arr_2: {one_element_arr_2:#?}");

    let one_element_arr_3 = my_vec!("Wison".to_string());
    println!("one_element_arr_3: {one_element_arr_3:#?}");

    let repeatable_arr = my_vec!("a", "b", "c", "d");
    println!("repeatable_arr: {repeatable_arr:#?}");

    let repeatable_arr_2 = my_vec!("A" => "B" => "C" => "D");
    println!("repeatable_arr_2: {repeatable_arr_2:#?}");

    struct AuthService {};

    impl_login!(AuthService);

    let service = AuthService {};
    let user_name = "wison";
    let password = "my_password";
    let login_result = service.login(&user_name, &password);

    let init_arr = my_vec!(999; 10);
    println!("init_arr: {init_arr:#?}");
}
