Math Text Transform
===================

![Travis build](https://img.shields.io/travis/runarberg/math-text-transform.svg)
![crates.io](https://img.shields.io/crates/v/math-text-transform.svg)

A simple crate that provides functions to map a greek letter, latin
letter, or a decimal digit to a certain variant from the mathematical
alphanumeric symbols Unicode block (U+1D400–U+1D7FF). We also provide
convenience string methods that maps each character in a string to the
variant (or leaves it alone if there is none).

Supported variants are:

* 𝐛𝐨𝐥𝐝 (bold)
* 𝑖𝑡𝑎𝑙𝑖𝑐 (italic)
* 𝒃𝒐𝒍𝒅 𝒊𝒕𝒂𝒍𝒊𝒄 (bold italic)
* 𝗌𝖺𝗇𝗌-𝗌𝖾𝗋𝗂𝖿 (sans-serif)
* 𝘀𝗮𝗻𝘀-𝘀𝗲𝗿𝗶𝗳 𝗯𝗼𝗹𝗱 (sans-serif bold)
* 𝘴𝘢𝘯𝘴-𝘴𝘦𝘳𝘪𝘧 𝘪𝘵𝘢𝘭𝘪𝘤 (sans-serif italic)
* 𝙨𝙖𝙣𝙨-𝙨𝙚𝙧𝙞𝙛 𝙗𝙤𝙡𝙙 𝙞𝙩𝙖𝙡𝙞𝙘 (sans-serif bold italic)
* 𝓈𝒸𝓇𝒾𝓅𝓉 (script)
* 𝓫𝓸𝓵𝓭 𝓼𝓬𝓻𝓲𝓹𝓽 (bold script)
* 𝔣𝔯𝔞𝔨𝔱𝔲𝔯 (fraktur)
* 𝖇𝖔𝖑𝖉 𝖋𝖗𝖆𝖐𝖙𝖚𝖗 (bold fraktur)
* 𝚖𝚘𝚗𝚘𝚜𝚙𝚊𝚌𝚎 (monospace)
* 𝕕𝕠𝕦𝕓𝕝𝕖-𝕤𝕥𝕣𝕦𝕔𝕜 (double-struck)


Install
-------

Add this to your `Cargo.toml`:

```toml
[dependencies]
math-text-transform = "*"
```


Usage
-----

```rust
extern crate math_text_transform;
use math_text_transform::MathTextTransform;

assert_eq!("Bold".to_math_bold(), "𝐁𝐨𝐥𝐝");
assert_eq!("Σανσ-Σεριφ-Βολδ".to_math_sans_serif_bold(), "𝝨𝝰𝝼𝞂-𝝨𝝴𝞀𝝸𝞅-𝝗𝝾𝝺𝝳");
assert_eq!("Double-struck 123".to_math_double_struck(), "𝔻𝕠𝕦𝕓𝕝𝕖-𝕤𝕥𝕣𝕦𝕔𝕜 𝟙𝟚𝟛");
```


Documentation
-------------

https://runarberg.github.com/math-text-transform


CLI utility
-----------

If you are for some reason only interested in the CLI utility
you can install with:

```bash
$ git clone https://github.com/runarberg/math-text-transform.git
$ cd math-text-transform
$ cargo build --release
$ [sudo] ln -s $(pwd)/target/release/math-text-transform /usr/local/bin/math-text-transform
```

And use like so:

```bash
math-text-transform <variant> <text>
```

Text can also be supplied through the standard input.

### Examples

```bash
$ math-text-transform -bf-it Bold-Italic
𝑩𝒐𝒍𝒅-𝑰𝒕𝒂𝒍𝒊𝒄
$ echo "Double struck" | math-text-transform --double-struck
𝔻𝕠𝕦𝕓𝕝𝕖 𝕤𝕥𝕣𝕦𝕔𝕜
```
