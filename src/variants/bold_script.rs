/// Transform a character to it's mathematical bold script equivalent.
pub fn math_bold_script(c: char) -> Option<char> {
    match c {
        // Latin capital letters.
        'A' => Some('𝓐'),
        'B' => Some('𝓑'),
        'C' => Some('𝓒'),
        'D' => Some('𝓓'),
        'E' => Some('𝓔'),
        'F' => Some('𝓕'),
        'G' => Some('𝓖'),
        'H' => Some('𝓗'),
        'I' => Some('𝓘'),
        'J' => Some('𝓙'),
        'K' => Some('𝓚'),
        'L' => Some('𝓛'),
        'M' => Some('𝓜'),
        'N' => Some('𝓝'),
        'O' => Some('𝓞'),
        'P' => Some('𝓟'),
        'Q' => Some('𝓠'),
        'R' => Some('𝓡'),
        'S' => Some('𝓢'),
        'T' => Some('𝓣'),
        'U' => Some('𝓤'),
        'V' => Some('𝓥'),
        'W' => Some('𝓦'),
        'X' => Some('𝓧'),
        'Y' => Some('𝓨'),
        'Z' => Some('𝓩'),

        // Latin small letters.
        'a' => Some('𝓪'),
        'b' => Some('𝓫'),
        'c' => Some('𝓬'),
        'd' => Some('𝓭'),
        'e' => Some('𝓮'),
        'f' => Some('𝓯'),
        'g' => Some('𝓰'),
        'h' => Some('𝓱'),
        'i' => Some('𝓲'),
        'j' => Some('𝓳'),
        'k' => Some('𝓴'),
        'l' => Some('𝓵'),
        'm' => Some('𝓶'),
        'n' => Some('𝓷'),
        'o' => Some('𝓸'),
        'p' => Some('𝓹'),
        'q' => Some('𝓺'),
        'r' => Some('𝓻'),
        's' => Some('𝓼'),
        't' => Some('𝓽'),
        'u' => Some('𝓾'),
        'v' => Some('𝓿'),
        'w' => Some('𝔀'),
        'x' => Some('𝔁'),
        'y' => Some('𝔂'),
        'z' => Some('𝔃'),

        // No equivalence.
        _ => None,
    }
}
