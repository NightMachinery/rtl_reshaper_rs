use unic_bidi::BidiInfo;
use arabic_reshaper::arabic_reshape;
use std::io::{self, Read};

fn main() {

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();
    handle
        .read_to_string(&mut input)
        .expect("Couldn't read from stdin.");

    let reshaped = arabic_reshape(&input);
    // println!("{}",reshaped);
    let bidi_info = BidiInfo::new(&reshaped, None);
    for para in &bidi_info.paragraphs {
        let line = para.range.clone();
        let display = bidi_info.reorder_line(para, line);
        print!("{}", display)
    }
}

// fn reorder_paras(text: &str) -> Vec<Cow<str>> {
//     let bidi_info = BidiInfo::new(text, None);
//     bidi_info
//         .paragraphs
//         .iter()
//         .map(|para| bidi_info.reorder_line(para, para.range.clone()))
//         .collect()
// }
