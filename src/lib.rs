//! Transformer text from standard alphanumerical characters to it’s
//! unicode equivalents of a certain variant in the [mathematical
//! alphanumeric symbols block][1] (code block U+1D400–U+1D7FF).
//!
//! ```rust
//! use math_text_transform::math_italic;
//! use math_text_transform::MathTextTransform;
//!
//! assert_eq!(math_italic('f'), Some('𝑓'));
//! assert_eq!("Bold".to_math_bold(), "𝐁𝐨𝐥𝐝");
//! ```
//!
//! Supported variants are:
//!
//! * 𝐛𝐨𝐥𝐝 (bold)
//! * 𝑖𝑡𝑎𝑙𝑖𝑐 (italic)
//! * 𝒃𝒐𝒍𝒅 𝒊𝒕𝒂𝒍𝒊𝒄 (bold italic)
//! * 𝗌𝖺𝗇𝗌-𝗌𝖾𝗋𝗂𝖿 (sans-serif)
//! * 𝘀𝗮𝗻𝘀-𝘀𝗲𝗿𝗶𝗳 𝗯𝗼𝗹𝗱 (sans-serif bold)
//! * 𝘴𝘢𝘯𝘴-𝘴𝘦𝘳𝘪𝘧 𝘪𝘵𝘢𝘭𝘪𝘤 (sans-serif italic)
//! * 𝙨𝙖𝙣𝙨-𝙨𝙚𝙧𝙞𝙛 𝙗𝙤𝙡𝙙 𝙞𝙩𝙖𝙡𝙞𝙘 (sans-serif bold italic)
//! * 𝓈𝒸𝓇𝒾𝓅𝓉 (script)
//! * 𝓫𝓸𝓵𝓭 𝓼𝓬𝓻𝓲𝓹𝓽 (bold script)
//! * 𝔣𝔯𝔞𝔨𝔱𝔲𝔯 (fraktur)
//! * 𝖇𝖔𝖑𝖉 𝖋𝖗𝖆𝖐𝖙𝖚𝖗 (bold fraktur)
//! * 𝚖𝚘𝚗𝚘𝚜𝚙𝚊𝚌𝚎 (monospace)
//! * 𝕕𝕠𝕦𝕓𝕝𝕖-𝕤𝕥𝕣𝕦𝕔𝕜 (double-struck)
//!
//! ### References
//!
//! * https://en.wikipedia.org/wiki/Mathematical_Alphanumeric_Symbols
//!
//! [1]: https://en.wikipedia.org/wiki/Mathematical_Alphanumeric_Symbols

mod variants;
pub use variants::*;

macro_rules! to_math_variant {
    ($to_math_variant:ident, $math_variant:expr) => {
        fn $to_math_variant(&self) -> String {
            self.chars()
                .map(|ch|
                     if let Some(variant) = $math_variant(ch) { variant }
                     else { ch })
                .collect()
        }
    }
}

/// Convenience trait that allows you to call the the transformation
/// straight on a string slice. If a variant doesn't exist for a given
/// character, it is left as is.
///
/// ### Examples
///
/// ```rust
/// use math_text_transform::MathTextTransform;
///
/// assert_eq!("Italıc f(π)".to_math_italic(), "𝐼𝑡𝑎𝑙𝚤𝑐 𝑓(𝜋)");
/// assert_eq!("Double-struck 123".to_math_double_struck(), "𝔻𝕠𝕦𝕓𝕝𝕖-𝕤𝕥𝕣𝕦𝕔𝕜 𝟙𝟚𝟛");
/// ```
pub trait MathTextTransform<T> {
    /// Transform to 𝐛𝐨𝐥𝐝 variant.
    fn to_math_bold(&self) -> T;

    /// Transform to 𝑖𝑡𝑎𝑙𝑖𝑐 variant.
    fn to_math_italic(&self) -> T;

    /// Transform to 𝒃𝒐𝒍𝒅 𝒊𝒕𝒂𝒍𝒊𝒄 variant.
    fn to_math_bold_italic(&self) -> T;

    /// Transform to 𝗌𝖺𝗇𝗌-𝗌𝖾𝗋𝗂𝖿 variant.
    fn to_math_sans_serif(&self) -> T;

    /// Transform to 𝘀𝗮𝗻𝘀-𝘀𝗲𝗿𝗶𝗳 𝗯𝗼𝗹𝗱 variant.
    fn to_math_sans_serif_bold(&self) -> T;

    /// Transform to 𝘴𝘢𝘯𝘴-𝘴𝘦𝘳𝘪𝘧 𝘪𝘵𝘢𝘭𝘪𝘤 variant.
    fn to_math_sans_serif_italic(&self) -> T;

    /// Transform to 𝙨𝙖𝙣𝙨-𝙨𝙚𝙧𝙞𝙛 𝙗𝙤𝙡𝙙 𝙞𝙩𝙖𝙡𝙞𝙘 variant.
    fn to_math_sans_serif_bold_italic(&self) -> T;

    /// Transform to 𝓈𝒸𝓇𝒾𝓅𝓉 variant.
    fn to_math_script(&self) -> T;

    /// Transform to 𝓫𝓸𝓵𝓭 𝓼𝓬𝓻𝓲𝓹𝓽 variant.
    fn to_math_bold_script(&self) -> T;

    /// Transform to 𝔣𝔯𝔞𝔨𝔱𝔲𝔯 variant.
    fn to_math_fraktur(&self) -> T;

    /// Transform to 𝖇𝖔𝖑𝖉 𝖋𝖗𝖆𝖐𝖙𝖚𝖗 variant.
    fn to_math_bold_fraktur(&self) -> T;

    /// Transform to 𝚖𝚘𝚗𝚘𝚜𝚙𝚊𝚌𝚎 variant.
    fn to_math_monospace(&self) -> T;

    /// Transform to 𝕕𝕠𝕦𝕓𝕝𝕖-𝕤𝕥𝕣𝕦𝕔𝕜 variant.
    fn to_math_double_struck(&self) -> T;
}

impl MathTextTransform<String> for str {
    to_math_variant!(to_math_bold, math_bold);
    to_math_variant!(to_math_italic, math_italic);
    to_math_variant!(to_math_bold_italic, math_bold_italic);
    to_math_variant!(to_math_sans_serif, math_sans_serif);
    to_math_variant!(to_math_sans_serif_bold, math_sans_serif_bold);
    to_math_variant!(to_math_sans_serif_italic, math_sans_serif_italic);
    to_math_variant!(to_math_sans_serif_bold_italic, math_sans_serif_bold_italic);
    to_math_variant!(to_math_script, math_script);
    to_math_variant!(to_math_bold_script, math_bold_script);
    to_math_variant!(to_math_fraktur, math_fraktur);
    to_math_variant!(to_math_bold_fraktur, math_bold_fraktur);
    to_math_variant!(to_math_monospace, math_monospace);
    to_math_variant!(to_math_double_struck, math_double_struck);
}


#[cfg(test)]
mod tests {
    use super::MathTextTransform;

    #[test]
    fn to_math_bold() {
        assert_eq!("Bold 123".to_math_bold(), "𝐁𝐨𝐥𝐝 𝟏𝟐𝟑");
        assert_eq!("Βολδ".to_math_bold(), "𝚩𝛐𝛌𝛅");
    }

    #[test]
    fn to_math_italic() {
        assert_eq!("Italıc 123".to_math_italic(), "𝐼𝑡𝑎𝑙𝚤𝑐 123");
        assert_eq!("Ιταλικ".to_math_italic(), "𝛪𝜏𝛼𝜆𝜄𝜅");
    }

    #[test]
    fn to_math_bold_italic() {
        assert_eq!("Bold-Italic 123".to_math_bold_italic(), "𝑩𝒐𝒍𝒅-𝑰𝒕𝒂𝒍𝒊𝒄 123");
        assert_eq!("Βολδ-Ιταλικ".to_math_bold_italic(), "𝜝𝝄𝝀𝜹-𝜤𝝉𝜶𝝀𝜾𝜿");
    }

    #[test]
    fn to_math_sans_serif() {
        assert_eq!("Sans-Serif 123".to_math_sans_serif(), "𝖲𝖺𝗇𝗌-𝖲𝖾𝗋𝗂𝖿 𝟏𝟐𝟑");
    }

    #[test]
    fn to_math_sans_serif_bold() {
        assert_eq!("Sans-Serif-Bold 123".to_math_sans_serif_bold(), "𝗦𝗮𝗻𝘀-𝗦𝗲𝗿𝗶𝗳-𝗕𝗼𝗹𝗱 𝟭𝟮𝟯");
        assert_eq!("Σανσ-Σεριφ-Βολδ".to_math_sans_serif_bold(), "𝝨𝝰𝝼𝞂-𝝨𝝴𝞀𝝸𝞅-𝝗𝝾𝝺𝝳");
    }

    #[test]
    fn to_math_sans_serif_italic() {
        assert_eq!("Sans-Serif-Italic 123".to_math_sans_serif_italic(), "𝘚𝘢𝘯𝘴-𝘚𝘦𝘳𝘪𝘧-𝘐𝘵𝘢𝘭𝘪𝘤 123");
    }

    #[test]
    fn to_math_sans_serif_bold_italic() {
        assert_eq!("Sans-Serif-Bold-Italic 123".to_math_sans_serif_bold_italic(), "𝙎𝙖𝙣𝙨-𝙎𝙚𝙧𝙞𝙛-𝘽𝙤𝙡𝙙-𝙄𝙩𝙖𝙡𝙞𝙘 123");
        assert_eq!("Σανσ-Σεριφ-Βολδ-Ιταλικ".to_math_sans_serif_bold_italic(), "𝞢𝞪𝞶𝞼-𝞢𝞮𝞺𝞲𝞿-𝞑𝞸𝞴𝞭-𝞘𝞽𝞪𝞴𝞲𝞳");
    }

    #[test]
    fn to_math_script() {
        assert_eq!("Script 123".to_math_script(), "𝒮𝒸𝓇𝒾𝓅𝓉 123");
    }

    #[test]
    fn to_math_bold_script() {
        assert_eq!("Bold-Script 123".to_math_bold_script(), "𝓑𝓸𝓵𝓭-𝓢𝓬𝓻𝓲𝓹𝓽 123");
    }

    #[test]
    fn to_math_fraktur() {
        assert_eq!("Fraktur 123".to_math_fraktur(), "𝔉𝔯𝔞𝔨𝔱𝔲𝔯 123");
    }

    #[test]
    fn to_math_bold_fraktur() {
        assert_eq!("Bold-Fraktur 123".to_math_bold_fraktur(), "𝕭𝖔𝖑𝖉-𝕱𝖗𝖆𝖐𝖙𝖚𝖗 123");
    }

    #[test]
    fn to_math_monospace() {
        assert_eq!("Monospace 123".to_math_monospace(), "𝙼𝚘𝚗𝚘𝚜𝚙𝚊𝚌𝚎 𝟷𝟸𝟹");
    }

    #[test]
    fn to_math_double_struck() {
        assert_eq!("Double-Struck 123".to_math_double_struck(), "𝔻𝕠𝕦𝕓𝕝𝕖-𝕊𝕥𝕣𝕦𝕔𝕜 𝟙𝟚𝟛");
    }
}
