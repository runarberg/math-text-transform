//! The executable source. Provides a command line interface for the
//! crate. Run with:
//!
//! ```ignore
//! $ math-text-transform <variant> [<text> | < stdin]
//! ```
//!
//! where `<variant>` is one of `bold`, `italic`, `bold-italic`
//! or `sans-serif`. You can either provide the input as the second
//! argument or pipe it from the standard input.

extern crate math_text_transform;

use math_text_transform::MathTextTransform;

use std::env;
use std::io::{self, BufRead};

enum Variant {
    Bold,
    Italic,
    BoldItalic,
    SansSerif,
    SansSerifBold,
    SansSerifItalic,
    SansSerifBoldItalic,
    Script,
    BoldScript,
    Fraktur,
    BoldFraktur,
    Monospace,
    DoubleStruck,
}

fn main() {
    use Variant::*;

    let args: Vec<String> = env::args().collect();
    let nargs = args.len();

    if nargs < 2 || 3 < nargs {
        print!("{}", include_str!("../etc/usage.txt"));
        std::process::exit(1);
    }

    let variant = match &*args[1] {
        "-bf"       | "--bold"                   => Bold,
        "-it"       | "--italic"                 => Italic,
        "-bf-it"    | "--bold-italic"            => BoldItalic,
        "-sf"       | "--sans-serif"             => SansSerif,
        "-sf-bf"    | "--sans-serif-bold"        => SansSerifBold,
        "-sf-it"    | "--sans-serif-italic"      => SansSerifItalic,
        "-sf-bf-it" | "--sans-serif-bold-italic" => SansSerifBoldItalic,
        "-cc"       | "--script"                 => Script,
        "-bf-cc"    | "--bold-script"            => BoldScript,
        "-fr"       | "--fraktur"                => Fraktur,
        "-bf-fr"    | "--bold-fraktur"           => BoldFraktur,
        "-tt"       | "--monospace"              => Monospace,
        "-bb"       | "--double-struck"          => DoubleStruck,
        "-h" | "--help" => {
            print!("{}\n{}\n{}",
                   include_str!("../etc/description.txt"),
                   include_str!("../etc/usage.txt"),
                   include_str!("../etc/variants.txt"));
            return;
        }
        _ => {
            print!("{}", include_str!("../etc/variants.txt"));
            std::process::exit(1);
        }
    };

    let transform: Box<Fn(&str) -> String> = match variant {
        Bold                => Box::new(move |s| s.to_math_bold()),
        Italic              => Box::new(move |s| s.to_math_italic()),
        BoldItalic          => Box::new(move |s| s.to_math_bold_italic()),
        SansSerif           => Box::new(move |s| s.to_math_sans_serif()),
        SansSerifBold       => Box::new(move |s| s.to_math_sans_serif_bold()),
        SansSerifItalic     => Box::new(move |s| s.to_math_sans_serif_italic()),
        SansSerifBoldItalic => Box::new(move |s| s.to_math_sans_serif_bold_italic()),
        Script              => Box::new(move |s| s.to_math_script()),
        BoldScript          => Box::new(move |s| s.to_math_bold_script()),
        Fraktur             => Box::new(move |s| s.to_math_fraktur()),
        BoldFraktur         => Box::new(move |s| s.to_math_bold_fraktur()),
        Monospace           => Box::new(move |s| s.to_math_monospace()),
        DoubleStruck        => Box::new(move |s| s.to_math_double_struck()),
    };

    if nargs == 3 {
        let input = &args[2];
        println!("{}", transform(input));
        return;
    }

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(Ok(line)) = lines.next() {
        println!("{}", transform(&line));
    }
}
