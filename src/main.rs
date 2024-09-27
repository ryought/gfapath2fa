use bio::alphabets::dna;
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader};

fn parse_p_path(s: &str) -> Vec<(String, bool)> {
    s.split(',')
        .map(|s| {
            let n = s.len();
            let (first, last) = s.split_at(n - 1);
            (first.to_string(), (last == "-"))
        })
        .collect()
}

fn parse_w_path(s: &str) -> Vec<(String, bool)> {
    let indices: Vec<_> = s.match_indices(&['>', '<']).collect();
    let mut path = vec![];
    for i in 0..indices.len() {
        let (pos0, sep) = indices[i];
        let pos1 = if i == indices.len() - 1 {
            s.len()
        } else {
            indices[i + 1].0
        };
        let is_rev = match sep {
            ">" => false,
            "<" => true,
            _ => unreachable!(),
        };
        path.push((s[pos0 + 1..pos1].to_string(), is_rev));
    }
    path
}

fn parse_gfa<R: BufRead>(reader: R) -> (Vec<(String, String)>, Vec<(String, Vec<(String, bool)>)>) {
    let mut segments: Vec<(String, String)> = Vec::new();
    let mut paths: Vec<(String, Vec<(String, bool)>)> = Vec::new();
    let alphabet = dna::n_alphabet();

    for (i, line_result) in reader.lines().enumerate() {
        match line_result {
            Ok(line) => {
                let tokens: Vec<&str> = line.split('\t').collect();
                match tokens[0] {
                    "S" => {
                        let name = tokens[1].to_string();
                        let seq = tokens[2].to_string();
                        if !alphabet.is_word(seq.as_bytes()) {
                            panic!("non dna sequence in L{}", i + 1)
                        }
                        segments.push((name, seq));
                    }
                    "L" => {
                        if !(tokens[5] == "*" || tokens[5] == "0M") {
                            panic!("overlapping link is not supported in L{}", i + 1)
                        }
                    }
                    "P" => {
                        let name = tokens[1].to_string();
                        let path = parse_p_path(tokens[2]);
                        paths.push((name, path));
                    }
                    "W" => {
                        // sample#haplotype#chromosome
                        let name = if tokens[4] != "*" && tokens[4] != "0" {
                            format!(
                                "{}#{}#{}:{}-{}",
                                tokens[1],
                                tokens[2],
                                tokens[3],
                                tokens[4].parse::<usize>().unwrap() + 1,
                                tokens[5],
                            )
                        } else {
                            tokens[1..4].join("#")
                        };
                        let path = parse_w_path(tokens[6]);
                        paths.push((name, path));
                    }
                    _ => {}
                }
            }
            Err(err) => {
                panic!("error {}", err)
            }
        }
    }
    (segments, paths)
}

fn main() {
    let reader = BufReader::new(io::stdin());
    let (segments, paths) = parse_gfa(reader);
    let segments: HashMap<String, String> = HashMap::from_iter(segments);
    for (name, path) in paths.iter() {
        println!(">{}", name);
        for (node, is_rev) in path {
            let s = segments
                .get(node)
                .unwrap_or_else(|| panic!("node {} is not found", node));
            if *is_rev {
                print!(
                    "{}",
                    std::str::from_utf8(&dna::revcomp(s.as_bytes())).unwrap()
                )
            } else {
                print!("{}", s)
            }
        }
        print!("\n")
    }
}

//
// tests
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path() {
        let expected = vec![
            ("s1".to_string(), false),
            ("s2".to_string(), false),
            ("s3".to_string(), true),
            ("s4".to_string(), false),
        ];
        let path = parse_p_path("s1+,s2+,s3-,s4+");
        println!("{:?}", path);
        assert_eq!(path, expected);

        let path = parse_w_path(">s1>s2<s3>s4");
        println!("{:?}", path);
        assert_eq!(path, expected);
    }

    #[test]
    fn gfa() {
        let s = vec![
            "H\tVN:Z:1.2",
            "S\ts1\tATCGATCG",
            "S\ts2\tTTTTTCCCCC",
            "L\ts1\t+\ts2\t-",
            "P\tp1\ts1+,s2-\t*",
            "W\ta\t1\tchr1\t0\t10\t>s1>s2<s1",
        ]
        .join("\n");
        let (segments, paths) = parse_gfa(s.as_bytes());
        println!("segments = {:?}", segments);
        println!("paths = {:?}", paths);
    }
}
