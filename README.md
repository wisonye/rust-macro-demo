# Rust macro demo


About the macro in `Rust`, you can think like this:

Macro is just a handy way to produce source code, then you don't need to write too much boring and repeat code

It looks like a handy function but is more flexible on syntax:)


</br>

There are 2 kind of macros in `Rust`:

- Declarative Macros:

    This is the macro we’re talking about in this demo, it simply replaces the pattern input with the substitution code block you provided, and that happens before the compilation phase.

    </br>


- Procedural Macros:

    Procedural macro does the same thing as declarative macro, and it's more
    powerful but more complicated. It comes in one of three flavors:

    - Function-like macros - `custom!(...)`

    - Derive macros - `#[derive(CustomDerive)]`

    - Attribute macros - `#[CustomAttribute]`

    [FlattenTree](https://github.com/wisonye/flatten-tree) is an example for
    writing procedural marco.

    </br>


Let's take a look the macro syntax:


```rust
//
// `#[macro_export]` is equal to pub the following macro.
//
#[macro_export]
macro_rules! MACRO_NAME_HERE {
    // $rules0;
    // $rules1;
    // ...
    // $rulesN;
}
```

</br>

Each `$rule` looks like this:  `($pattern) => { source code block }`

It means that you give the `($pattern)` as the macro input, and it expands
to the source code block you provided as the macro output.

The `($pattern)` part looks like a function argument (token) list, but is more
flexible and repeatable:

`($variable name: syntax type)`

All valid `syntax type`:

- `item`: an item, like a function, struct, module, etc.
- `block`: a block (i.e. a block of statements and/or an expression, surrounded by braces)
- `stmt`: a statement
- `pat`: a pattern
- `expr`: an expression
- `ty`: a type
- `ident`: an identifier
- `path`: a path (e.g. foo, `::std::mem::replace`, `transmute::<_, int>`, …)
- `meta`: a meta item; the things that go inside `#[...]` and `#![...]` attributes
- `tt`: a single token tree


</br>

Use you can `cargo-expand` to see what source code expanded before the compilation phase. If you want watch and expand, then use `cargo watch -c --exec "expand"` instead.

</br>


### Empty vector macro

```rust
#[macro_export]
macro_rules! my_vec {
    // Only one rule, or say `matching parttern`
    ($type: ty) => {
        Vec::<$type>::new()
    };
}
```

</br>

Then you can use the macro like this:

```rust
let empty_string_arr = my_vec!(String);
let empty_usize_arr = my_vec!(usize);
```

</br>

It will expand to the following source code before the compilation:

```rust
let empty_string_arr = Vec::<String>::new();
let empty_usize_arr = Vec::<usize>::new();
```

</br>


### Single element with type macro

If you want to return the entire code block that includes declaring
variables inside, then you have to use a pair of `(), [], {}` to surround
the `{}`. For example, all the following syntax are valid:


```rust
($pattern) => ({ code block })
($pattern) => [{ code block }]
($pattern) => {{ code block }} // This is the default recommended.
```

</br>

```rust
macro_rules! my_vec {
    ($element: expr, $type: ty) => {{
        let mut temp_arr = Vec::<$type>::new();
        temp_arr.push($element);
        temp_arr
    }};
}
```

</br>

Then you can use the macro like this:

```rust
let one_element_arr = my_vec!(45, usize);
let one_element_arr_2 = my_vec!("Wison Ye", &str);
let one_element_arr_3 = my_vec!("Wison Ye".to_string());
```

</br>

It will expand to the following source code before the compilation:

```rust
let one_element_arr = {
    let mut temp_arr = Vec::<usize>::new();
    temp_arr.push(45);
    temp_arr
};

let one_element_arr_2 = {
    let mut temp_arr = Vec::<&str>::new();
    temp_arr.push("Wison");
    temp_arr
};

let one_element_arr_3 = {
    let mut temp_arr = Vec::new();
    temp_arr.push("Wison".to_string());
    temp_arr
};
```

</br>


### Init style macro

Give a init value and a count to repeat, it's very common case to init a
collection by macro.

```rust
macro_rules! my_vec {{
    ($element: expr; $count: expr) => {{
        let mut temp_arr = Vec::new();
        for _ in 0..$count {
            temp_arr.push($element);
        }

        temp_arr
    }}
}}
```
</br>

Then you can use the macro like this:

```rust
let init_arr = my_vec!(88; 10);
```

</br>

It will expand to the following source code before the compilation:

```rust
let init_arr = {
    let mut temp_arr = Vec::new();
    for _ in 0..10 {
        temp_arr.push(88);
    }
    temp_arr
};
```

</br>


### Repeatable macro


The `$(var_name: syntax type), repeating part description` syntax is used to
present the repeatable patterns. Basically, it's saying:

- The `$()` part is repeatable and separated by the `,` delimiter

- For the repeating description, is the same meaning with regex: `?/+/*`

</br>

Also, you can use `$() repeating description` syntax in the code
block to repeat like a loop:

```rust
macro_rules! my_vec {
    // `$element: expr` is repeatable, it separates by `,` delimiter.
    // And it can show up 1 or more repeat times.
    ($($element: expr), +) => {{
        let mut temp_arr = Vec::new();
        $(temp_arr.push($element);)+
        temp_arr
    }};
}
```

</br>

Then you can use the macro like this:

```rust
let repeatable_arr = my_vec!("a", "b", "c", "d");
```

</br>

It will expand to the following source code before the compilation:

```rust
let repeatable_arr = {
    let mut temp_arr = Vec::new();
    temp_arr.push("a");
    temp_arr.push("b");
    temp_arr.push("c");
    temp_arr.push("d");
    temp_arr
};
```

</br>


Pay attention that:

_**The pattern syntax actually is very loosely, you can define your own
syntax as you want if it doesn't violate the limitation.**_

For example, you can invent the following syntax to archive the same
purpose with the above matching rule:

```rust
macro_rules! my_vec {
    // `$element: expr` is repeatable, it separates by `=>` delimiter.
    // And it can show up 1 or more repeat times.
    ($($element: expr) => +) => {{
        let mut temp_arr = Vec::new();
        $(temp_arr.push($element);)+
        temp_arr
    }};
}
```

</br>

Then you can use the macro like this:

```rust
let repeatable_arr_2 = my_vec!("A" => "B" => "C" => "D");
```

</br>

It will expand to the following source code before the compilation:

```rust
let repeatable_arr_2 = {
    let mut temp_arr = Vec::new();
    temp_arr.push("A");
    temp_arr.push("B");
    temp_arr.push("C");
    temp_arr.push("D");
    temp_arr
};
```

</br>


But this doesn't work, as `>>>` is not allowed to show up after a expression!!!
```rust
macro_rules! my_vec {
    ($($element: expr) >>> +) => {{
    }};
}
```

</br>


### Implment trait for struct

You can use macro to implement a trait in a handy way, for example:

```rust
// Declare the Login trait
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
```

</br>


Then you can use the macro like this:

```rust
struct AuthService {};

impl_login!(AuthService);

// let service = AuthService {};
// let user_name = "wison";
// let password = "my_password";
// let login_result = service.login(&user_name, &password);
```

</br>

It will expand to the following source code before the compilation:

```rust
struct AuthService {};
{
    impl crate::struct_macro_sample::Login for AuthService {
        fn login(&self, user_name: &str, password: &str) -> bool {
            user_name.trim() == password.trim()
        }
    }
};

// let service = AuthService {};
// let user_name = "wison";
// let password = "my_password";
// let login_result = service.login(&user_name, &password);
```




