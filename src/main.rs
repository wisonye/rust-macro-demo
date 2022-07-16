mod basic_macro;

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

    let one_element_arr_2 = my_vec!("Wison Ye", &str);
    println!("one_element_arr_2: {one_element_arr_2:#?}");

    let one_element_arr_3 = my_vec!("Wison Ye".to_string());
    println!("one_element_arr_3: {one_element_arr_3:#?}");
}
