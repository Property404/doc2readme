Path: /home/dagan/Development/schmargs/target/doc/schmargs/index.html
...
Wow!
#[no_std]A argument parser that can be used with 
Features

 functionality--help that allows for wrapperA 
Custom and default short and long flags
std::vec::VecMulti-arg positional arguments and options with 
Optional arguments
-friendly#![no_std]
-inspired derive macroclap-derive

Todo

Improve and write tests for help formatting
Improve documentation

Helper Attributesschmargs
This is an optional attribute that should be specified at the top level.
Arguments:

 MUST be specified.iterates_over, Schmargs::parse_env environment and plan on parsing
arguments passed to your program with std with an appropriate lifetime. If you’re in an &str. This defaults
to Schmargs::parse type passed to core::iter::Iterator
associated type of the Item - The string type that’s being iterated over. This should be the iterates_over=<type>
 - The name of the program. Defaults to the crate name.name=<str literal>

args
This is an optional attribute that should be specified on an argument.
Arguments:

 - The long flag of the argument. If no value is provided, it will
default to the the argument name.long[=<str literal>]
 - The short flag of the argument. If no value is provided, it will
default to the first letter of the argument name.short[=<char literal>]

Example
.std::env::Args, so you can iterate over String to be
iterates_over environment, you generally want to specify stdWhen using in an 

, args.content);"{:?}"(println!args = Args::parse_env();
let // This parses the arguments passed to the program
content: Vec<String>,
}

/// Obscenities to yell
    <u64>,
    Optionlength: #[arg(short, long)]
    /// Yell length, in nanoseconds
    <u64>,
    Optionvolume: #[arg(short, long)]
    /// Yell volume, in decibels
    Args {
    struct #[derive(Schmargs)]
#[schmargs(iterates_over=String)]
/// A program to yell at a cloud
schmargs::Schmargs;

use 
 Examples#![no_std]
);256(args.len, assert_eq!u8);
*const as 0x40000000 (args.start, assert_eq!));
8(Some(args.group, assert_eq!);
true(args.no_null_check, assert_eq!);
false(args.color, assert_eq!.split_whitespace()).unwrap();
"-f --group 8 0x40000000 256"args = Args::parse(let }

// required positional argument
len: usize, /// Number of bytes to read
    // required positional argument
    u8, *const start: /// Starting memory address
    // this is optional
    <u8>, Optiongroup: #[arg(short, long)]
    /// How many bytes to show per line
    no_null_check: bool,
    )]
    "force", long = 'f'#[arg(short = /// Disable sanity checks
    color: bool,
    #[arg(short, long)]
    /// Show color
    Args {
    struct )]
"hexdump"#[derive(Schmargs)]
#[schmargs(name = /// A simple memory dump program
schmargs::Schmargs;

use 
When strings are involved, you need to add a generic lifetime parameter

);"Dagan"(args.person, assert_eq!);
false(args.kick_shins, assert_eq!.split_whitespace()).unwrap();
"Dagan"args = Args::parse(let str,
}

'a &person: /// The person to greet
    kick_shins: bool,
    )]
    "kick"#[arg(short, long = /// Should we kick the person's shins after greeting them?
    > {
    'aArgs<struct )]
"greet"#[derive(Schmargs)]
#[schmargs(name = /// A very important program to greet somebody
schmargs::Schmargs;

use 

