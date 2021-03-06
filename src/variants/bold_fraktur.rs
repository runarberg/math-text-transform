/// Transform a character to it's mathematical bold fraktur
/// equivalent.
pub fn math_bold_fraktur(c: char) -> Option<char> {
    match c {
        // Latin capital letters.
        'A' => Some('𝕬'),
        'B' => Some('𝕭'),
        'C' => Some('𝕮'),
        'D' => Some('𝕯'),
        'E' => Some('𝕰'),
        'F' => Some('𝕱'),
        'G' => Some('𝕲'),
        'H' => Some('𝕳'),
        'I' => Some('𝕴'),
        'J' => Some('𝕵'),
        'K' => Some('𝕶'),
        'L' => Some('𝕷'),
        'M' => Some('𝕸'),
        'N' => Some('𝕹'),
        'O' => Some('𝕺'),
        'P' => Some('𝕻'),
        'Q' => Some('𝕼'),
        'R' => Some('𝕽'),
        'S' => Some('𝕾'),
        'T' => Some('𝕿'),
        'U' => Some('𝖀'),
        'V' => Some('𝖁'),
        'W' => Some('𝖂'),
        'X' => Some('𝖃'),
        'Y' => Some('𝖄'),
        'Z' => Some('𝖅'),

        // Latin small letters.
        'a' => Some('𝖆'),
        'b' => Some('𝖇'),
        'c' => Some('𝖈'),
        'd' => Some('𝖉'),
        'e' => Some('𝖊'),
        'f' => Some('𝖋'),
        'g' => Some('𝖌'),
        'h' => Some('𝖍'),
        'i' => Some('𝖎'),
        'j' => Some('𝖏'),
        'k' => Some('𝖐'),
        'l' => Some('𝖑'),
        'm' => Some('𝖒'),
        'n' => Some('𝖓'),
        'o' => Some('𝖔'),
        'p' => Some('𝖕'),
        'q' => Some('𝖖'),
        'r' => Some('𝖗'),
        's' => Some('𝖘'),
        't' => Some('𝖙'),
        'u' => Some('𝖚'),
        'v' => Some('𝖛'),
        'w' => Some('𝖜'),
        'x' => Some('𝖝'),
        'y' => Some('𝖞'),
        'z' => Some('𝖟'),

        // No equivalence.
        _ => None,
    }
}
