use bio::alphabets::dna;
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader};

fn parse_p_path(s: &str) -> Vec<(usize, bool)> {
    s.split(',')
        .map(|s| {
            let n = s.len();
            let (first, last) = s.split_at(n - 1);
            (
                first.parse::<usize>().expect("node id must be integer"),
                (last == "-"),
            )
        })
        .collect()
}

fn parse_w_path(s: &str) -> Vec<(usize, bool)> {
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
        path.push((
            s[pos0 + 1..pos1]
                .parse::<usize>()
                .expect("node id must be integer"),
            is_rev,
        ));
    }
    path
}

fn parse_gfa<R: BufRead>(
    reader: R,
) -> (HashMap<usize, Vec<u8>>, Vec<(String, Vec<(usize, bool)>)>) {
    let mut segments: HashMap<usize, Vec<u8>> = HashMap::new();
    let mut paths: Vec<(String, Vec<(usize, bool)>)> = Vec::new();
    let alphabet = dna::n_alphabet();

    for (i, line_result) in reader.lines().enumerate() {
        match line_result {
            Ok(line) => {
                let tokens: Vec<&str> = line.split('\t').collect();
                match tokens[0] {
                    "S" => {
                        let name = tokens[1].parse::<usize>().expect("segment name is not int");
                        let seq = tokens[2].as_bytes().to_vec();
                        if !alphabet.is_word(&seq) {
                            panic!("non dna sequence in L{}", i + 1)
                        }
                        segments.insert(name, seq);
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
                            let start = tokens[4].parse::<usize>().unwrap();
                            let end = tokens[5].parse::<usize>().unwrap();
                            format!(
                                "{}#{}#{}:{}-{}",
                                tokens[1],
                                tokens[2],
                                tokens[3],
                                start + 1,
                                end,
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
    for (name, path) in paths.iter() {
        println!(">{}", name);
        for (node, is_rev) in path {
            let s = segments
                .get(node)
                .unwrap_or_else(|| panic!("node {} is not found", node));
            if *is_rev {
                print!("{}", std::str::from_utf8(&dna::revcomp(s)).unwrap())
            } else {
                print!("{}", std::str::from_utf8(s).unwrap())
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
        let expected = vec![(1, false), (2, false), (3, true), (4, false)];
        let path = parse_p_path("1+,2+,3-,4+");
        println!("{:?}", path);
        assert_eq!(path, expected);

        let path = parse_w_path(">1>2<3>4");
        println!("{:?}", path);
        assert_eq!(path, expected);
    }

    #[test]
    fn gfa() {
        let s = vec![
            "H\tVN:Z:1.2",
            "S\t1\tATCGATCG",
            "S\t2\tTTTTTCCCCC",
            "L\t1\t+\t2\t-",
            "P\tp1\t1+,2-\t*",
            "W\ta\t1\tchr1\t0\t10\t>1>2<1",
        ]
        .join("\n");
        let (segments, paths) = parse_gfa(s.as_bytes());
        println!("segments = {:?}", segments);
        println!("paths = {:?}", paths);
    }
}
