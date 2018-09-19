use data::Data;

pub trait Morse{
    fn to_morse(&self) -> String;
    //fn from_morse(&self) -> String;
}

fn substitute_morse(chr: char) -> String{
    let morse: &str = {
        match chr {
            'A' => "·−",
            'B' => "−···",
            'C' => "−·−·",
            'D' => "−··",
            'E' => "·",
            'F' => "··−·",
            'G' => "−−·",
            'H' => "····",
            'I' => "··",
            'J' => "·−−−",
            'K' => "−·−",
            'L' => "·−··",
            'M' => "−−",
            'N' => "−·",
            'O' => "−−−",
            'P' => "·−−·",
            'Q' => "−−·−",
            'R' => "·−·",
            'S' => "···",
            'T' => "−",
            'U' => "··−",
            'V' => "···−",
            'W' => "·−−",
            'X' => "−··−",
            'Y' => "−·−−",
            'Z' => "−−··",
            ' ' => " ",
            _ => panic!("{} couldn't be used in substitution.", chr),
        }
    };
    morse.to_string()
}

impl Morse for Data{
    fn to_morse(&self) -> String {
        self.to_string()
            .to_uppercase()
            .chars()
            .into_iter()
            .map(substitute_morse)
            .collect::<String>()
    }
}
