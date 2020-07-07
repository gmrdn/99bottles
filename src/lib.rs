use std::fmt;

pub fn verse(n: u32) -> String {
    let bottle_number = BottleNumber::initialize(n);
    let next_bottle_number = BottleNumber::initialize(bottle_number.successor());
    format!(
        "{} of beer on the wall, \
        {} of beer.\n\
        {} \
        {} of beer on the wall.\n",
        capitalize(&bottle_number.to_string()),
        bottle_number,
        bottle_number.action(),
        next_bottle_number,
    )
}

fn capitalize(words: &String) -> String {
    let mut v: Vec<char> = words.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    v.into_iter().collect()
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song: String = String::from("");
    for i in (end..=start).rev() {
        song.push_str(verse(i).as_str());
        if i > end {
            song.push_str("\n");
        }
    }
    song
}

struct BottleNumber {
    number: u32,
}

impl fmt::Display for BottleNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.quantity(), self.container())
    }
}

impl BottleNumber {
    fn initialize(number: u32) -> BottleNumber {
        BottleNumber { number }
    }

    fn container(&self) -> String {
        match self.number {
            1 => String::from("bottle"),
            _ => String::from("bottles"),
        }
    }

    fn pronoun(&self) -> String {
        match self.number {
            1 => String::from("it"),
            _ => String::from("one"),
        }
    }

    fn quantity(&self) -> String {
        match self.number {
            0 => String::from("no more"),
            _ => format!("{}", self.number),
        }
    }

    fn action(&self) -> String {
        match self.number {
            0 => String::from("Go to the store and buy some more,"),
            _ => format!("Take {} down and pass it around,", self.pronoun()),
        }
    }

    fn successor(&self) -> u32 {
        match self.number {
            0 => 99,
            _ => self.number - 1,
        }
    }
}
