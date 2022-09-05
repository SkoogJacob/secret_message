/// This module translates a text string consisting of ASCII letters and digits to morse, and vice
/// versa.
pub mod morse {
    // Stuff is private by default in rust. So unless a type or function is explicitly declared as
    // pub it will not be callable outside of this module

    // static values are valid for the entire execution to the program and always point to a single
    // location in memory.
    // const values are inlined to every place using it. Good for small data types (like chars or
    // integral types)

    // Since most of this program uses the `String` type it cannot run on some integrated chips
    // that do not have heap allocation

    /// Enum representing the "symbols" of morse code. Designed after description
    /// [here](https://en.wikipedia.org/wiki/Morse_code) for international morse code
    #[derive(Clone, PartialEq, Eq, Debug)]
    enum Morse {
        /// A 'dot' in morse. Is the short signal
        Dit,
        /// A 'dash' in morse. Is the long signal. Length equal to 3 * Dit
        Dah,
        /// Basic space. Length is one Dit
        BaseSpace,
        /// A blank between morse letters. Length is 3 dits
        LetterSpace,
        /// A blank between words. Length is 7 dits
        WordSpace
    }

    impl Morse {
        // impl blocks are where you define functions for types (in this case for an enum)

        /// Returns a string representation of the morse symbols
        fn to_str(&self) -> &str {
            match self {
                Morse::Dit => "·",
                Morse::Dah => "―",
                Morse::LetterSpace => "   ",
                Morse::WordSpace => "       ",
                Morse::BaseSpace => " "
            }
        }

        fn from_str(symbol: &str) -> Self {
            match symbol {
                "·" => Morse::Dit,
                "―" => Morse::Dah,
                " " => Morse::BaseSpace,
                "   " => Morse::LetterSpace,
                "       " => Morse::WordSpace,
                _ => unreachable!("Morse should not contain other str sequences")
            }
        }

        /// Returns the length of the morse symbols as u8 scalars
        fn len(&self) -> u8 {
            match self {
                Morse::Dit |
                Morse::BaseSpace => 1,
                Morse::Dah |
                Morse::LetterSpace => 3,
                Morse::WordSpace => 7,
            }
        }
    }

    static CHAR_MORSE_MAP: [(char, &[Morse]); 37] = [ // needs to be &[···] since length of
                                                   // different entries varies
        (' ', &[Morse::WordSpace]),
        ('a', &[Morse::Dit, Morse::Dah]),
        ('b', &[Morse::Dah, Morse::Dit, Morse::Dit, Morse::Dit]),
        ('c', &[Morse::Dah, Morse::Dit, Morse::Dah, Morse::Dit]),
        ('d', &[Morse::Dah, Morse::Dit, Morse::Dit]),
        ('e', &[Morse::Dit]),
        ('f', &[Morse::Dit, Morse::Dit, Morse::Dah, Morse::Dit]),
        ('g', &[Morse::Dah, Morse::Dah, Morse::Dit]),
        ('h', &[Morse::Dit, Morse::Dit, Morse::Dit, Morse::Dit]),
        ('i', &[Morse::Dit, Morse::Dit]),
        ('j', &[Morse::Dit, Morse::Dah, Morse::Dah, Morse::Dah]),
        ('k', &[Morse::Dah, Morse::Dit, Morse::Dah]),
        ('l', &[Morse::Dit, Morse::Dah, Morse::Dit, Morse::Dit]),
        ('m', &[Morse::Dah, Morse::Dah]),
        ('n', &[Morse::Dah, Morse::Dit]),
        ('o', &[Morse::Dah, Morse::Dah, Morse::Dah]),
        ('p', &[Morse::Dit, Morse::Dah, Morse::Dah, Morse::Dit]),
        ('q', &[Morse::Dah, Morse::Dah, Morse::Dit, Morse::Dah]),
        ('r', &[Morse::Dit, Morse::Dah, Morse::Dit]),
        ('s', &[Morse::Dit, Morse::Dit, Morse::Dit]),
        ('t', &[Morse::Dah]),
        ('u', &[Morse::Dit, Morse::Dit, Morse::Dah]),
        ('v', &[Morse::Dit, Morse::Dit, Morse::Dit, Morse::Dah]),
        ('w', &[Morse::Dit, Morse::Dah, Morse::Dah]),
        ('x', &[Morse::Dah, Morse::Dit, Morse::Dit, Morse::Dah]),
        ('y', &[Morse::Dah, Morse::Dit, Morse::Dah, Morse::Dah]),
        ('z', &[Morse::Dah, Morse::Dah, Morse::Dit, Morse::Dit]),
        ('1', &[Morse::Dit, Morse::Dah, Morse::Dah, Morse::Dah, Morse::Dah]),
        ('2', &[Morse::Dit, Morse::Dit, Morse::Dah, Morse::Dah, Morse::Dah]),
        ('3', &[Morse::Dit, Morse::Dit, Morse::Dit, Morse::Dah, Morse::Dah]),
        ('4', &[Morse::Dit, Morse::Dit, Morse::Dit, Morse::Dit, Morse::Dah]),
        ('5', &[Morse::Dit, Morse::Dit, Morse::Dit, Morse::Dit, Morse::Dit]),
        ('6', &[Morse::Dah, Morse::Dit, Morse::Dit, Morse::Dit, Morse::Dit]),
        ('7', &[Morse::Dah, Morse::Dah, Morse::Dit, Morse::Dit, Morse::Dit]),
        ('8', &[Morse::Dah, Morse::Dah, Morse::Dah, Morse::Dit, Morse::Dit]),
        ('9', &[Morse::Dah, Morse::Dah, Morse::Dah, Morse::Dah, Morse::Dit]),
        ('0', &[Morse::Dah, Morse::Dah, Morse::Dah, Morse::Dah, Morse::Dah]),
    ];

    /// Converts a morse letter to its string representation
    fn morse_char_to_str(c: &[Morse]) -> String {
        let morse: Vec<&str> = c.iter()
            .map(|x| x.to_str())
            .collect();
        morse.join(Morse::BaseSpace.to_str())
    }

    /// Takes a string of a morse letter and returns it as a `Morse` Vec
    fn from_morse_str_to_morse_array(msg: &str) -> Vec<Morse> {
        // Collecting into a vector since dynamically creating arrays from iterators in rust isn't
        // really a thing
        let morse: Vec<Morse> = msg.split(Morse::BaseSpace.to_str())
            .map(Morse::from_str)
            .collect();
        return morse
    }

    fn from_morse_array_to_char(morse: &[Morse]) -> char {
        let c: Vec<char> = CHAR_MORSE_MAP.iter()
            .filter(|m| m.1 == morse)
            .map(|m| m.0)
            .collect();
        return c.first().unwrap().to_owned() // Cloning a char and passing a pointer is basically the same on 64-bit systems
    }

    fn word_to_morse(word: &str) -> String {
        let mut morse_word: Vec<String> = Vec::with_capacity(128);
        for c in word.chars() {
            let char_morse: Vec<&[Morse]> = CHAR_MORSE_MAP.iter()
                .filter(|k| k.0 == c)
                .map(|m| m.1)
                .collect();
            morse_word.push(morse_char_to_str(char_morse.first().unwrap()));
        }
        morse_word.join(Morse::LetterSpace.to_str())
    }

    fn morseword_to_alphabet(morseword: &str) -> String {
        let word: Vec<String> = morseword.split(Morse::LetterSpace.to_str())
            .map(from_morse_str_to_morse_array)
            .map(|morse| {
                from_morse_array_to_char(&morse[..]).to_string()
            })
            .collect();
        word.join("")
    }

    pub fn print_morse() {
        CHAR_MORSE_MAP.iter().for_each(|c| println!("{}: {}", c.0, morse_char_to_str(c.1)))
    }

    pub fn translate_to_morse(msg: &str) -> String {
        let msg: Vec<String> = msg.split(' ')
            .map(word_to_morse)
            .collect();
        return msg.join(Morse::WordSpace.to_str())
    }

    pub fn translate_from_morse(msg: &str) -> String {
        let msg: Vec<String> = msg.split(Morse::WordSpace.to_str())
            .map(morseword_to_alphabet)
            .collect();
        msg.join(" ")
    }
}

// library files in rust can use rusts included testing harness by having a module annotated with
// `#[cfg(test)]`, every individual test is then a function annotated with `#[test]`

#[cfg(test)]
mod morse_test {
    use super::morse;

    #[test]
    fn test_encode_eq_decode() {
        let msg_str = "hello there friend";
        let msg_morse = morse::translate_to_morse(msg_str);
        let msg_back_to_str = morse::translate_from_morse(&msg_morse);
        assert_eq!(msg_str.trim(), msg_back_to_str.trim())
    }
}
