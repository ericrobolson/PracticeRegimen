use std::{fmt::Display, vec};

use rand::Rng;

fn main() {
    // Guitar regimen
    music_theory_reading();
    tune_instrument();
    learn_fretboard();
    play_scales_and_chords();
    learn_intervals();
    circle_of_fifths();

    play_songs();
}

fn circle_of_fifths() {
    todo!("Somehow learn circle of fifths. Pg. 19.");
}
fn play_songs() {
    todo!()
}
fn play_scales_and_chords() {
    println!("Pick a random key.");

    // Scale + mode training
    println!("Do this for each mode:");
    println!("\tGo up and down it in a vertical fashion, moving up the neck.");
    println!("\tGo up and down it in a horizontal fashion, moving across the neck. Repeat for each string by starting on the lowest string, find the tonic note, go up two octaves if possible.");
    // Chord training
    println!("For each chord in the key: ");
    println!("\nFind the positions of the chord on the neck.");
    println!("\nStrum that chord.");
    println!("\nPlay the arpeggio for each chord in a vertical fashion.");
    println!("\nPlay the arpeggio for each chord in a horizontal fashion.");
}

fn learn_intervals() {
    println!("Pick a random fret on the neck.");
    println!("Use the shapes on pg 26 to play the intervals.");
    println!("This needs to do fig 5 and fig 6 on pg 26.");
    println!("Play all simple intervals in Music Theory pg 25");
    println!("Play all complex intervals in Music Theory pg 25");
}

fn music_theory_reading() {
    println!("Spend a bit of time each day on a music theory book.");
    println!("This will be Music Theory - Guitar Method by Hal Leonard.");
    println!("Read about 4 pages.");
    println!("Note book name");
    println!("Note page started and page ended");
    println!("What was the topic?");
    println!("Summarize it. Give about 4 bullet points.");
    println!("What was one thing that stood out to you?");
    println!("Think about how to apply this to the regimen.")
}

fn tune_instrument() {
    println!("Tune your instrument using a tuner.");
    println!("Tune your guitar to itself:");

    println!("\tPlay the 5th fret of the low F# string and tune the B string to it.");
    println!("\tPlay the 5th fret of the low B string and tune the E string to it.");
    println!("\tPlay the 5th fret of the low E string and tune the A string to it.");
    println!("\tPlay the 5th fret of the A string and tune the D string to it.");
    println!("\tPlay the 5th fret of the D string and tune the G string to it.");
    println!("\tPlay the 4th fret of the G string and tune the B string to it.");
    println!("\tPlay the 5th fret of the B string and tune the high E string to it.");
}

fn learn_fretboard() {
    let note = get_random_note();
    let timing_combinations = vec![
        vec![
            TimingDivisor::Sixteenth,
            TimingDivisor::Sixteenth,
            TimingDivisor::Sixteenth,
            TimingDivisor::Sixteenth,
        ],
        vec![TimingDivisor::Triplet],
        vec![
            TimingDivisor::Sixteenth,
            TimingDivisor::Eighth,
            TimingDivisor::Eighth,
        ],
        vec![
            TimingDivisor::Eighth,
            TimingDivisor::Eighth,
            TimingDivisor::Sixteenth,
        ],
    ];

    println!("");
    println!("");
    println!("For the note {}, do the following from frets 0-11:", note);
    println!("\tGo up and down the strings and play it in the following timing:");
    for (i, timing) in timing_combinations.iter().enumerate() {
        println!("\t\t{} {}", i + 1, print_timings(timing));
    }
    println!("Repeat this for frets 12+.");
}

fn get_random_note() -> Note {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..12);
    Note::from(r)
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum TimingDivisor {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    Triplet,
}
fn print_timings(timings: &Vec<TimingDivisor>) -> String {
    let mut output = String::new();
    output.push_str("[ ");
    output.push_str(
        &timings
            .iter()
            .map(|t| format!("{}", t))
            .collect::<Vec<String>>()
            .join(" "),
    );

    output.push_str(" ]");
    output
}
impl Display for TimingDivisor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TimingDivisor::Whole => write!(f, "1"),
            TimingDivisor::Half => write!(f, "½"),
            TimingDivisor::Quarter => write!(f, "¼"),
            TimingDivisor::Eighth => write!(f, "⅛"),
            TimingDivisor::Sixteenth => write!(f, "¹⁄₁₆"),
            TimingDivisor::Triplet => write!(f, "triplet"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Note {
    A,
    BFlat,
    B,
    C,
    CSharp,
    D,
    EFlat,
    E,
    F,
    FSharp,
    G,
    GSharp,
}
impl Note {
    pub fn from(num: usize) -> Self {
        match num % 12 {
            0 => Note::A,
            1 => Note::BFlat,
            2 => Note::B,
            3 => Note::C,
            4 => Note::CSharp,
            5 => Note::D,
            6 => Note::EFlat,
            7 => Note::E,
            8 => Note::F,
            9 => Note::FSharp,
            10 => Note::G,
            11 => Note::GSharp,
            _ => Note::A,
        }
    }
}
impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Note::A => write!(f, "A"),
            Note::BFlat => write!(f, "A#/B♭"),
            Note::B => write!(f, "B"),
            Note::C => write!(f, "C"),
            Note::CSharp => write!(f, "C#/B♭"),
            Note::D => write!(f, "D"),
            Note::EFlat => write!(f, "D#/E♭"),
            Note::E => write!(f, "E"),
            Note::F => write!(f, "F"),
            Note::FSharp => write!(f, "F#/G♭"),
            Note::G => write!(f, "G"),
            Note::GSharp => write!(f, "G#/A♭"),
        }
    }
}
