const GIFTS: [&str; 12] = [
    "a partridge in a pear tree",
    "two turtle doves",
    "three French hens",
    "four calling birds",
    "five gold rings",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
];

const ORDINALS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

fn main() {
    let mut lyrics: Vec<String> = vec![];

    for day in 0..12 {
        let verse = format!(
            "On the {} day of Christmas\nmy true love sent to me\n{}",
            day.as_ordinal(),
            get_gifts_for_day(day)
        );

        lyrics.push(verse)
    }

    let lyrics = lyrics.join("\n\n");

    println!("{}", lyrics);
}

fn get_gifts_for_day(day: usize) -> String {
    let mut gifts_for_day = String::new();

    let mut d = 0;
    for gift in GIFTS[0..=day].iter().rev() {
        gifts_for_day = match gifts_for_day.len() {
            len if len == 0 => gift.to_string(),
            _ if d < day => format!("{},\n{}", gifts_for_day, gift),
            _ => format!("{},\nand {}", gifts_for_day, gift),
        };
        d += 1;
    }

    return gifts_for_day;
}

trait Ordinal {
    fn as_ordinal(&self) -> &str;
}

impl Ordinal for usize {
    fn as_ordinal(&self) -> &str {
        return match self {
            x if *x < 12 => ORDINALS[*self],
            _ => "unknown",
        };
    }
}
