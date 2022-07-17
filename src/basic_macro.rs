//
// `#[macro_export]` is equal to pub the following macro.
//
#[macro_export]
macro_rules! my_vec {
    // $rules0;
    // $rules1;
    // ...
    // $rulesN;
    //
    // Each `$rule` looks like this:  ($pattern) => {$expansion}
    //
    // It's saying `($pattern)` expans to the particular expression (code block),
    // or you can see as this:
    //
    // ($pattern) => { Code block }
    //
    // ($pattern) looks like a function argument (token) list, but more flexible
    // and repeatable:
    //
    // ($variable name: syntax type)
    //
    // All valid `syntax type`:
    //
    // item: an item, like a function, struct, module, etc.
    // block: a block (i.e. a block of statements and/or an expression, surrounded by braces)
    // stmt: a statement
    // pat: a pattern
    // expr: an expression
    // ty: a type
    // ident: an identifier
    // path: a path (e.g. foo, ::std::mem::replace, transmute::<_, int>, …)
    // meta: a meta item; the things that go inside #[...] and #![...] attributes
    // tt: a single token tree
    //

    // Empty vector
    ($type: ty) => {
        Vec::<$type>::new()
    };

    //
    // If you want to create something in the returning code block that
    // can declare var and even return it, then you have to use a pair of
    // `(), [], {}` to surround the `{}`. For example, all following syntax
    // are valid:
    //
    // `($pattern) => ({ code block })`
    // `($pattern) => [{ code block }]`
    // `($pattern) => {{ code block }}` // This is the default recommended.
    //
    ($element: expr, $type: ty) => {{
        let mut temp_arr = Vec::<$type>::new();
        temp_arr.push($element);
        temp_arr
    }};

    // Only element without type
    ($element: expr) => {{
        let mut temp_arr = Vec::new();
        temp_arr.push($element);
        temp_arr
    }};

    // Repeatable
    //
    // The `$(var_name: syntax type), repeating part description` syntax is
    // used to present the repeatable patterns. Basically, it's saying:
    //
    // - The `$()` part is repeatable and separated by the `,` delimiter
    // - For the repeating part description, is the same with regex: ?/+/*
    //
    // Also, you can use `$() repeating part description` syntax in the code
    // block to repeat like for `for loop`
    //
    ($($element: expr), +) => {{
        let mut temp_arr = Vec::new();
        $(temp_arr.push($element);)+
        temp_arr
    }};

    //
    // Pay attention that:
    //
    // The pattern syntax actually is very loosely, you can define
    // your own syntax as want, if it doesn't violate the limitation.
    //
    // For example, you can invent the following syntax to archive the same
    // purpose with the above matching rule:
    //
    ($($element: expr) => +) => {{
        let mut temp_arr = Vec::new();
        $(temp_arr.push($element);)+
        temp_arr
    }};

    // But this doesn't work, as `>>>` is not allowed to show up after a expression:)
    // ($($element: expr) >>> +) => {{
    // }};



    ($element: expr; $count: expr) => {{
        // By using `with_capacity` instead of `new`, we can avoid
        // keeping allocating the Vector in a large $count case, it's
        // more efficient.
        let count = $count;
        let mut temp_arr = Vec::with_capacity(count);

        // By using `extend` instead of `push in a for loop`, we can
        // avoid the vector pointer moving and bounds checking every
        // time, it's more efficient!!!
        //
        // for _ in 0..count {
        //     temp_arr.push($element);
        // }
        temp_arr.extend(std::iter::repeat($element).take(count));

        temp_arr
    }};
}
