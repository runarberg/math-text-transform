/// Transform a character to it's mathematical bold equivalent.
pub fn math_bold(c: char) -> Option<char> {
    match c {
        // Digits
        '0' => Some('𝟎'),
        '1' => Some('𝟏'),
        '2' => Some('𝟐'),
        '3' => Some('𝟑'),
        '4' => Some('𝟒'),
        '5' => Some('𝟓'),
        '6' => Some('𝟔'),
        '7' => Some('𝟕'),
        '8' => Some('𝟖'),
        '9' => Some('𝟗'),

        // Latin capital letters.
        'A' => Some('𝐀'),
        'B' => Some('𝐁'),
        'C' => Some('𝐂'),
        'D' => Some('𝐃'),
        'E' => Some('𝐄'),
        'F' => Some('𝐅'),
        'G' => Some('𝐆'),
        'H' => Some('𝐇'),
        'I' => Some('𝐈'),
        'J' => Some('𝐉'),
        'K' => Some('𝐊'),
        'L' => Some('𝐋'),
        'M' => Some('𝐌'),
        'N' => Some('𝐍'),
        'O' => Some('𝐎'),
        'P' => Some('𝐏'),
        'Q' => Some('𝐐'),
        'R' => Some('𝐑'),
        'S' => Some('𝐒'),
        'T' => Some('𝐓'),
        'U' => Some('𝐔'),
        'V' => Some('𝐕'),
        'W' => Some('𝐖'),
        'X' => Some('𝐗'),
        'Y' => Some('𝐘'),
        'Z' => Some('𝐙'),

        // Latin small letters.
        'a' => Some('𝐚'),
        'b' => Some('𝐛'),
        'c' => Some('𝐜'),
        'd' => Some('𝐝'),
        'e' => Some('𝐞'),
        'f' => Some('𝐟'),
        'g' => Some('𝐠'),
        'h' => Some('𝐡'),
        'i' => Some('𝐢'),
        'j' => Some('𝐣'),
        'k' => Some('𝐤'),
        'l' => Some('𝐥'),
        'm' => Some('𝐦'),
        'n' => Some('𝐧'),
        'o' => Some('𝐨'),
        'p' => Some('𝐩'),
        'q' => Some('𝐪'),
        'r' => Some('𝐫'),
        's' => Some('𝐬'),
        't' => Some('𝐭'),
        'u' => Some('𝐮'),
        'v' => Some('𝐯'),
        'w' => Some('𝐰'),
        'x' => Some('𝐱'),
        'y' => Some('𝐲'),
        'z' => Some('𝐳'),

        // Greek capital letters.
        'Α' => Some('𝚨'),
        'Β' => Some('𝚩'),
        'Γ' => Some('𝚪'),
        'Δ' => Some('𝚫'),
        'Ε' => Some('𝚬'),
        'Ζ' => Some('𝚭'),
        'Η' => Some('𝚮'),
        'Θ' => Some('𝚯'),
        'Ι' => Some('𝚰'),
        'Κ' => Some('𝚱'),
        'Λ' => Some('𝚲'),
        'Μ' => Some('𝚳'),
        'Ν' => Some('𝚴'),
        'Ξ' => Some('𝚵'),
        'Ο' => Some('𝚶'),
        'Π' => Some('𝚷'),
        'Ρ' => Some('𝚸'),
        'ϴ' => Some('𝚹'),
        'Σ' => Some('𝚺'),
        'Τ' => Some('𝚻'),
        'Υ' => Some('𝚼'),
        'Φ' => Some('𝚽'),
        'Χ' => Some('𝚾'),
        'Ψ' => Some('𝚿'),
        'Ω' => Some('𝛀'),
        '∇' => Some('𝛁'),

        // Greek small letters
        'α' => Some('𝛂'),
        'β' => Some('𝛃'),
        'γ' => Some('𝛄'),
        'δ' => Some('𝛅'),
        'ε' => Some('𝛆'),
        'ζ' => Some('𝛇'),
        'η' => Some('𝛈'),
        'θ' => Some('𝛉'),
        'ι' => Some('𝛊'),
        'κ' => Some('𝛋'),
        'λ' => Some('𝛌'),
        'μ' => Some('𝛍'),
        'ν' => Some('𝛎'),
        'ξ' => Some('𝛏'),
        'ο' => Some('𝛐'),
        'π' => Some('𝛑'),
        'ρ' => Some('𝛒'),
        'ς' => Some('𝛓'),
        'σ' => Some('𝛔'),
        'τ' => Some('𝛕'),
        'υ' => Some('𝛖'),
        'φ' => Some('𝛗'),
        'χ' => Some('𝛘'),
        'ψ' => Some('𝛙'),
        'ω' => Some('𝛚'),
        '∂' => Some('𝛛'),
        'ϵ' => Some('𝛜'),
        'ϑ' => Some('𝛝'),
        'ϰ' => Some('𝛞'),
        'ϕ' => Some('𝛟'),
        'ϱ' => Some('𝛠'),
        'ϖ' => Some('𝛡'),
        'Ϝ' => Some('𝟊'),
        'ϝ' => Some('𝟋'),

        // No equivalence.
        _ => None,
    }
}
