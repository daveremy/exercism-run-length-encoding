// pub fn encode(source: &str) -> String {
//     let source = source.chars().collect::<Vec<char>>();
//     let mut encoded = String::new();

//     let mut i = 0;
//     while i < source.len() {
//         let c = source[i];
//         let mut n = 1;
//         while i < source.len() - 1 && c == source[i + 1] {
//             n += 1;
//             i += 1;
//         }
//         if n > 1 {
//             encoded.push_str(&n.to_string());
//         }
//         encoded.push(c);
//         i += 1;
//     }
//     encoded
// }

#[derive(Debug)]
enum EncodeState {
    Initial,
    InRun(char, u32),
    Final(char, u32),
}

pub fn encode(source: &str) -> String {
    let source = source.chars().collect::<Vec<char>>();
    let mut encoded = String::new();
    let mut state = EncodeState::Initial;
    for i in 0..=source.len() + 1 {
        match state {
            EncodeState::Initial => {
                // source is empty string
                if source.len() == 0 {
                    break;
                }
                state = EncodeState::InRun(source[i], 1);
            }
            EncodeState::InRun(run_char, run_count) => {
                // last char in source
                if i == source.len() {
                    state = EncodeState::Final(run_char, run_count);
                    continue;
                }
                // continue run
                if source[i] == run_char {
                    let new_run_count = run_count + 1;
                    state = EncodeState::InRun(source[i], new_run_count);
                    continue;
                }
                // run ended, start next run
                if run_count > 1 {
                    encoded.push_str(&run_count.to_string());
                }
                encoded.push(run_char);
                state = EncodeState::InRun(source[i], 1);
            }
            EncodeState::Final(final_char, run_count) => {
                if run_count > 1 {
                    encoded.push_str(&run_count.to_string());
                }
                encoded.push(final_char);
            }
        }
    }
    encoded
}

enum DecodeState {
    Initial,
    Number,
    Char,
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut state = DecodeState::Initial;
    let mut num_accum: Vec<char> = Vec::new();
    for c in source.chars() {
        match state {
            DecodeState::Initial => {
                if c.is_numeric() {
                    num_accum.push(c);
                    state = DecodeState::Number;
                } else {
                    decoded.push(c);
                }
            }
            DecodeState::Number => {
                if c.is_numeric() {
                    num_accum.push(c);
                } else {
                    // state transition
                    let size_of_run = num_accum.iter().collect::<String>().parse::<u32>().unwrap();
                    for _ in 1..=size_of_run {
                        decoded.push(c);
                    }
                    num_accum.clear();
                    state = DecodeState::Char;
                }
            }
            DecodeState::Char => {
                if c.is_numeric() {
                    num_accum.push(c);
                    state = DecodeState::Number;
                } else {
                    decoded.push(c);
                }
            }
        }
    }
    decoded
}
