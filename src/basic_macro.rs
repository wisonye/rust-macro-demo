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
}
