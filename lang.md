## Comments
```
// this is a line comment
/// this is a doc comment
```

## Variables
```
let immutable = ...;
let mut mutable = ...;
let typed: Type = ...;
let uninitialized;
```

## Primitive Types (TODO - incomplete)
```
// boolean
bool

// unsigned integers:
usize, u8, u16, u32, u64

// signed integers:
isize, i8, i16, i32, i64

// floats
f32 f64
```

## Defining Functions
```
fn func_a() {
    // some code goes here
}

fn func_b(): () {
    // explicit version of func_a's
    // implicit signature
}

fn func_c(arg1: Type, arg2: Type) {
    // functions can take arguments

    // there is no such shorthand syntax
    // as `func-c(arg1, arg2: Type)` as
    // there is in langauges like Go
}

fn func_d(): Type {
    // the return type of this function
    // is Type
}

fn func_e(arg: Type1): Type2 {
    // this function takes an argument
    // of type Type1, and returns a
    // value of type Type2
}

pub fn func_f() {
    // "public" modifier
    // this function is exported from
    // its enclosing module
}

infal fn func_g() {
    // "infallible" modifier
    // this function is guaranteed never to
    // panic or crash the program. as such, it
    // can only call other infallible functions.
}

pub infal func_h() {
    // this function is both
    // public and infallible
}
```

## Calling functions
```
// given...
fn func_a() {
    // some stuff happens here
}

fn func_b(arg1: Type1, arg2: Type2): Type2 {
    // some stuff happens here
}

// call syntax looks like...
func_a();
let returned = func_b(some_arg, another_arg);
```

## Map Operator
```
// the map operator looks like a method call,
// but with the name of the method omitted.
// it has two forms. the first is "application
// form". it works like this:
let a = func(val);
let b = val.(func);

// in the above snippet, a and b are both the
// result of the same operation - calling func
// with the argument val.

// the second form is "subtitution form". in
// this form, the parentheses contain an expression
// that has one or more occurrences of `_` within
// it (not as part of an identifier). in this form,
// the value which the map operator is being applied
// to is subtituted in for all occurrences of _. then,
// the resultant expression is evaluated. the following
// expressions are identical:
let a = val + func(val);
let a = val.(_ + func(_));

// the purpose of the map operator is to allow normal
// functions and expressions to be conveniently used
// as part of a method chain. for example:
get_input()
    .parse::<f64>()?    // string -> float
    .sqrt()
    .(_ * 2.0)
    .powi(3)
    .(print_float);

// without the map operator, the above method chain would
// be much less pleasant to read and write:
print_float(
    (get_input()
        .parse::<f64>()?
        .sqrt()
        * 2.0     // easy to mess up precedence here
    ).powi(3)
);

// or you could use temporaries:
let temp: f64 = get_input().parse()?
let temp = temp * 2.0;
let temp = temp.powi(3);
print_float(temp);

// the map operator is also useful in cases
// where there are nested function calls. this
// snippet will be familiar to anyone that's done
// asynchronous programming in rust:
let thing = Arc::new(Mutex::new(MyStruct {
    a: "a",
    b: "b",
}));

// using the map operator, we can rewrite it like this:
let thing = MyStruct {
        a: "a",
        b: "b",
    }
    .(Mutex::new)
    .(Arc::new);
```
