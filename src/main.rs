use std::collections::HashMap;
use std::io::{self, BufRead, BufReader};

fn parse_gfa<R: BufRead>(reader: R) {
    let mut segments: Vec<(String, String)> = Vec::new();
    let mut paths: Vec<(String, Vec<(String, bool)>)> = Vec::new();

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let tokens: Vec<&str> = line.split('\t').collect();
                match tokens[0] {
                    "S" => {
                        let name = tokens[1].to_string();
                        let seq = tokens[2].to_string();
                        segments.push((name, seq));
                    }
                    "P" => {
                        let name = tokens[1].to_string();
                        let path = tokens[2]
                            .split(',')
                            .map(|s| {
                                let n = s.len();
                                let (first, last) = s.split_at(n - 1);
                                (first.to_string(), (last == "-"))
                            })
                            .collect();
                        paths.push((name, path));
                    }
                    "W" => {
                        // sample#haplotype#chromosome
                        let name = tokens[1..4].join("#");
                        let indices: Vec<_> = tokens[6].match_indices(&['>', '<']).collect();
                        for i in 0..indices.len() {
                            let (pos, sep) = indices[i];
                            let is_rev = match sep {
                                ">" => false,
                                "<" => true,
                                _ => unreachable!(),
                            }
                        }
                        println!("Wline {} {:?}", name, path);
                    }
                    _ => {}
                }
            }
            Err(err) => {
                panic!("error {}", err)
            }
        }
    }
    println!("segments = {:?}", segments);
    println!("paths = {:?}", paths);
}

fn main() {
    let reader = BufReader::new(io::stdin());
    println!("Hello, world!");
    parse_gfa(reader)
}

//
// tests
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gfa() {
        let s = vec![
            "H\tVN:Z:1.2",
            "S\ts1\tATCGATCG",
            "S\ts2\tTTTTTCCCCC",
            "L\ts1\t+\ts2\t-",
            "P\tp1\ts1+,s2-\t*",
            "W\ta\t1\tchr1\t0\t10\t>s1>s2<s1>",
        ]
        .join("\n");
        parse_gfa(s.as_bytes());
    }
}
