So, now I'll go through everything I changed and did to make your app faster. Let's start at the top of `main.rs`

## `main.rs`

```rust
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(group(ArgGroup::new("uwu").required(true).args(& ["text", "infile"]),))]
struct Args {
    /// Text to uwu'ify
    #[clap(short, long, required_unless_present_all = ["infile", "outfile"], display_order = 1)]
    text: Option<String>,

    /// The file to uwu'ify
    #[clap(short, long, parse(from_os_str), conflicts_with = "text", requires = "outfile", value_name = "FILE", value_hint = clap::ValueHint::FilePath, display_order = 2)]
    infile: Option<std::path::PathBuf>,

    /// The file to output uwu'ified text
    #[clap(short, long, value_name = "FILE", value_hint = clap::ValueHint::FilePath, display_order = 3)]
    outfile: Option<String>,

    /// The modifier to determine how many words to be uwu'ified
    #[clap(short, long, value_name = "VALUE", default_value = "1", validator = is_between_zero_and_one, display_order = 4)]
    words: f32,

    /// The modifier for uwu faces e.g hello -> hewwo
    #[clap(short, long, value_name = "VALUE", default_value = "0.05", validator = is_between_zero_and_one, display_order = 5)]
    faces: f32,

    /// The modifier for actions e.g *shuffles over*
    #[clap(short, long, value_name = "VALUE", default_value = "0.125", validator = is_between_zero_and_one, display_order = 6)]
    actions: f32,

    /// The modifier for stutters e.g b-baka!
    #[clap(short, long, value_name = "VALUE", default_value = "0.225", validator = is_between_zero_and_one, display_order = 7)]
    stutters: f32,

    /// Flag to enable/disable random uwu'ifying
    #[clap(short, long, display_order = 8)]
    random: bool,
}
```

`clap` works great using it's derive macro implementation, however it's much lighter on resources if you make the `clap::App` manually

```rust
macro_rules! app {
    () => {
        clap::App::new(env!("CARGO_PKG_NAME"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .version(env!("CARGO_PKG_VERSION"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .long_about(None)
            .group(
                ArgGroup::new("uwu")
                    .required(true)
                    .args(&["text", "infile"]),
            )
            .arg(
                Arg::new("text")
                    .help("Text to uwu'ify")
                    .short('t')
                    .long("text")
                    .required_unless_present_all(["infile", "outfile"])
                    .display_order(1),
            )
            .arg(
                Arg::new("infile")
                    .help("The file to uwu'ify")
                    .short('i')
                    .long("infile")
                    .conflicts_with("text")
                    .requires("outfile")
                    .value_name("FILE")
                    .value_hint(clap::ValueHint::FilePath)
                    .display_order(2),
            )
            .arg(
                Arg::new("outfile")
                    .help("The file to output uwu'ified text")
                    .short('o')
                    .long("outfile")
                    .value_name("FILE")
                    .value_hint(clap::ValueHint::FilePath)
                    .display_order(3),
            )
            .arg(
                Arg::new("words")
                    .help("The modifier to determine how many words to be uwu'ified")
                    .short('w')
                    .long("words")
                    .value_name("VALUE")
                    .default_value("1")
                    .validator(is_between_zero_and_one)
                    .display_order(4),
            )
            .arg(
                Arg::new("faces")
                    .help("The modifier for uwu faces e.g hello -> hewwo")
                    .short('f')
                    .long("faces")
                    .value_name("VALUE")
                    .default_value("0.05")
                    .validator(is_between_zero_and_one)
                    .display_order(5),
            )
            .arg(
                Arg::new("actions")
                    .help("The modifier for actions e.g *shuffles over*")
                    .short('a')
                    .long("actions")
                    .value_name("VALUE")
                    .default_value("0.125")
                    .validator(is_between_zero_and_one)
                    .display_order(6),
            )
            .arg(
                Arg::new("stutters")
                    .help("The modifier for stutters e.g b-baka!")
                    .short('s')
                    .long("stutters")
                    .value_name("VALUE")
                    .default_value("0.225")
                    .validator(is_between_zero_and_one)
                    .display_order(7),
            )
            .arg(
                Arg::new("random")
                    .help("Flag to enable/disable random uwu'ifying")
                    .short('r')
                    .long("random")
                    .display_order(8),
            )
    };
}
```

Here, I've created a **declarative macro** describing your `clap::App`. You can think of a declarative macro like using the compiler to copy and paste something in your code. You can read more on them [here](https://doc.rust-lang.org/rust-by-example/macros.html). You'll notice that for some of the attributes of your `clap::App`, I'm using the `env!()` macro in place of literal strings. All the `env!()` macro does is it grabs an environment variable at compile time, and pastes it into wherever the macro is located in the code. You can find docs for all the environment variables Cargo sets [here](https://doc.rust-lang.org/cargo/reference/environment-variables.html). Now, we move on to `fn main()`

```rust
fn main() {
    let matches = app!().get_matches();

    match UwUify::new(
        matches.value_of("text"),
        matches.value_of("infile"),
        matches.value_of("outfile"),
        matches.value_of("words"),
        matches.value_of("faces"),
        matches.value_of("actions"),
        matches.value_of("stutters"),
        matches.is_present("random"),
    )
    .uwuify()
    {
        Ok(_) => (),
        Err(err) => {
            app!().error(ErrorKind::DisplayHelp, err).exit();
        }
    }
}
```

This is how I've implemented the main function for this app. You'll notice a few things are different about it.

1.  I'm using  my `app!()` macro in place of `Args::parse()`
2.  `UwUify::new()` is taking exclusively `Option<&str>` values and 1 bool for the `random` variable (we'll get to that later)
3.  If the app fails, and I want to print the `err` message, instead of calling `err.to_string()`, I'm simply passing `err` into the `app!().error()` function.

I'm doing this using a Rust feature called **Traits**. Traits describe the capabilities of a given type. Instead of `app!().error()` taking a `String`, `app!().error()` actually takes any type that implements `Display`. This trait suggests that the type has the capability of expressing itself as a human-readable string. In your original implementation, `UwUify.uwuify()` returned a `Result<(), Box<dyn Error>>`. You'll notice that your error type is `dyn Error`, which translates to **any type that implements the `Debug` trait and the `Display` trait, that can also produce the source of the error, a backtrace, a description of what happend, as well as the cause of the error**. The key thing to note is that according to this description, all types that implement `Error` also implement the `Display` trait, and can therefore be passed to `app!().error()` without any conversion.

## `lib.rs`

If you're making an application that is modular in the way that yours is, it's standard that your external module be called `lib.rs`, and that it be annotated in `Cargo.toml` like so

```toml
[lib]
name = "uwuify"
```

Doing this allows for 2 things

1.  People can embed your app's functionality into their own apps through `crates.io`
2.  This improves compile times significantly depending on how big your modules actually are

We'll start with your actual `UwUify` struct

```rust
#[derive(Debug)]
pub struct UwUify<'a> {
    text: &'a str,
    input: &'a str,
    output: &'a str,
    words: f64,
    faces: f64,
    actions: f64,
    stutters: f64,
    random: bool,
    linkify: LinkFinder,
}

impl<'a> Default for UwUify<'a> {
    fn default() -> Self {
        Self {
            text: "",
            input: "",
            output: "",
            words: 1.0,
            faces: 0.05,
            actions: 0.125,
            stutters: 0.225,
            random: false,
            linkify: LinkFinder::new(),
        }
    }
}
```

Here, what I've done is I've removed your `Modifiers` sub-struct (for reasons that will become clear later), I've implemented the `Default` trait (The `Default` trait just describes what your struct should look like in it's default state), and I've changed the fields that used to take `String` types into taking `&str` types. This was one of the factors slowing down your code.

### Let's talk about allocations

In Rust, there are 2 types of memory

The stack is where the majority of your variables are located. This is one way to visualize the stack

    []
    []
    []
    []

On the stack, variables can be **pushed** onto it

    []
    ↓
    []
    []
    []
    []

And removed by **popping** them off

    []
    ↑
    []
    []
    []
    []

This is the best of memory that Rust offers. Not only is the heap fast, but it's also very efficient. You can think of the heap like New York City, where everyone is packed in shoulder to shoulder. Every variable is put on the stack automatically by Rust, except for some exceptions, which are **allocated** on the **heap**

The heap is like a desert, when you put a variable on the heap, it's put in a space in memory where it isn't packed tight between other variables.

Consider the following slice of memory

    00000000000[00100100011111010010]0000000000000000000

Annotated in brackets is where are variable resides. You'll notice that it has room to grow if we need to make it bigger

    00000000000[001001000111110100100100010010110]000000

The heap exists for types that don't have a known size at compile time.

*   Because the amount of elements on a `Vec` can change, they are allocated on the heap
*   Because we know that `i32` takes up 32 bits in memory, and it can never take more memory than that, it's pushed onto the stack.

If you want your app to be FAST, your best bet is to avoid the heap as much as possible.

Now, let's get into the 2 types of strings in Rust, `String` and `&str`. A `String` is a growable array of charecters while a `&str` describes the location of an array of charecters.

Say theoretically, you were to enter a chatroom with one of these types, and you needed to know their value, this is how each type would respond

    /---------------------------------------\
    |   Yo, &str what's you value?   |
    \---------------------------------------/
                                  
                                          /---------------------------------\
                                          |    Idk, go ask the String   |
                                          |           at [location]           |
                                          \---------------------------------/

<!---->

    /-----------------------------------------\
    |   Yo, String what's you value?   |
    \-----------------------------------------/
                                  
                                          /---------------------------------\
                                          |    My value is "UwU        |
                                          |       *nuzzzles you"          |
                                          \---------------------------------/

You get the idea, the point is, the `String` type is growable, so it's allocated on the heap. And because the size of pointers (pointers point at locations in memory) are known at compile time the `&str` type is pushed onto the stack. Therefore, you should use `&str` over `String` whenever possible.

## Let's look back at your `UwUify` struct

```rust
#[derive(Debug)]
pub struct UwUify<'a> {
    text: &'a str,
    input: &'a str,
    output: &'a str,
    words: f64,
    faces: f64,
    actions: f64,
    stutters: f64,
    random: bool,
    linkify: LinkFinder,
}
```

You'll notice that not only does it take the `&str` type over `String` now, it takes a type of \`&'a str. This is because you cannot have borrowed types in structs without first annotating the **lifetime** of the reference. Lifetimes and ownership are waaaaayyy to complicated to explain in a PR, so I recommend watching [this video](https://www.youtube.com/watch?v=y7iSQ3s_yms\&list=PLJbE2Yu2zumDF6BX6\_RdPisRVHgzV02NW\&index=3) if you want to understand more about them.
