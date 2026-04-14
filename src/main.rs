use std::fmt::Display;

use enum_display::EnumDisplay;

struct Time {
    minute_offset: Option<(MinuteWord, BeforeAfter)>,
    show_half: bool,
    hour: Hour,
}

impl Time {
    fn new(minute_offset: Option<(MinuteWord, BeforeAfter)>, show_half: bool, hour: Hour) -> Self {
        Self {
            minute_offset,
            show_half,
            hour,
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let half_word = if self.show_half { "halb " } else { "" };

        match self.minute_offset.as_ref() {
            Some((minute_word, before_after)) => {
                write!(f, "{minute_word} {before_after} {half_word}{}", self.hour)
            }
            None => write!(f, "{half_word}{}", self.hour),
        }
    }
}

impl TryFrom<(u8, u8)> for Time {
    type Error = &'static str;

    fn try_from((hour, mut minute): (u8, u8)) -> Result<Self, Self::Error> {
        minute = ((minute as f32 / 5.0).round() * 5.0) as u8;

        let hour = match hour {
            1 => Hour::One,
            2 => Hour::Two,
            3 => Hour::Three,
            4 => Hour::Four,
            5 => Hour::Five,
            6 => Hour::Six,
            7 => Hour::Seven,
            8 => Hour::Eight,
            9 => Hour::Nine,
            10 => Hour::Ten,
            11 => Hour::Eleven,
            12 => Hour::Twelve,
            _ => return Err("wrong hour"),
        };

        let next_hour = match hour {
            Hour::One => Hour::Two,
            Hour::Two => Hour::Three,
            Hour::Three => Hour::Four,
            Hour::Four => Hour::Five,
            Hour::Five => Hour::Six,
            Hour::Six => Hour::Seven,
            Hour::Seven => Hour::Eight,
            Hour::Eight => Hour::Nine,
            Hour::Nine => Hour::Ten,
            Hour::Ten => Hour::Eleven,
            Hour::Eleven => Hour::Twelve,
            Hour::Twelve => Hour::One,
        };

        match minute {
            0 => Ok(Time::new(None, false, hour)),
            5 => Ok(Time::new(
                Some((MinuteWord::Five, BeforeAfter::After)),
                false,
                hour,
            )),
            10 => Ok(Time::new(
                Some((MinuteWord::Ten, BeforeAfter::After)),
                false,
                hour,
            )),
            15 => Ok(Time::new(
                Some((MinuteWord::Quarter, BeforeAfter::After)),
                false,
                hour,
            )),
            20 => Ok(Time::new(
                Some((MinuteWord::Twenty, BeforeAfter::After)),
                false,
                hour,
            )),
            25 => Ok(Time::new(
                Some((MinuteWord::Five, BeforeAfter::Before)),
                true,
                next_hour,
            )),
            30 => Ok(Time::new(None, true, next_hour)),
            35 => Ok(Time::new(
                Some((MinuteWord::Five, BeforeAfter::After)),
                true,
                next_hour,
            )),
            40 => Ok(Time::new(
                Some((MinuteWord::Twenty, BeforeAfter::Before)),
                false,
                next_hour,
            )),
            45 => Ok(Time::new(
                Some((MinuteWord::Quarter, BeforeAfter::Before)),
                false,
                next_hour,
            )),
            50 => Ok(Time::new(
                Some((MinuteWord::Ten, BeforeAfter::Before)),
                false,
                next_hour,
            )),
            55 => Ok(Time::new(
                Some((MinuteWord::Five, BeforeAfter::Before)),
                false,
                next_hour,
            )),
            60 => Ok(Time::new(None, false, next_hour)),
            _ => Err("wrong minute"),
        }
    }
}

#[derive(EnumDisplay)]
enum Hour {
    #[display("eins")]
    One,
    #[display("zwei")]
    Two,
    #[display("drei")]
    Three,
    #[display("vier")]
    Four,
    #[display("fünf")]
    Five,
    #[display("sechs")]
    Six,
    #[display("sieben")]
    Seven,
    #[display("acht")]
    Eight,
    #[display("neun")]
    Nine,
    #[display("zehn")]
    Ten,
    #[display("elf")]
    Eleven,
    #[display("zwölf")]
    Twelve,
}

#[derive(EnumDisplay)]
enum MinuteWord {
    #[display("fünf")]
    Five,
    #[display("zehn")]
    Ten,
    #[display("viertel")]
    Quarter,
    #[display("zwanzig")]
    Twenty,
}

#[derive(EnumDisplay)]
enum BeforeAfter {
    #[display("nach")]
    After,
    #[display("vor")]
    Before,
}

// 4:00                   Vier
// 4:05 Fünf    nach      Vier
// 4:10 Zehn    nach      Vier
// 4:15 Viertel nach      Vier
// 4:20 Zwanzig nach      Vier
// 4:25 Fünf    vor  halb Fünf
// 4:30              halb Fünf
// 4:35 Fünf    nach halb Fünf
// 4:40 Zwanzig vor       Fünf
// 4:45 Viertel vor       Fünf
// 4:50 Zehn    vor       Fünf
// 4:55 Fünf    vor       Fünf

fn main() {
    for hour in 1..=12 {
        for minute in 0..60 {
            print!("{hour:0>2}:{minute:0>2} ist ");

            let time = Time::try_from((hour, minute));
            println!("{}", time.unwrap());
        }

        println!();
    }
}
