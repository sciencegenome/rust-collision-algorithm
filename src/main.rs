mod args;
use args::GraphOffsetArgs;
use clap::Parser;
use regex::Regex;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
*Author Gaurav Sablok
*Universitat Potsdam
*Date 2024-12-22

rust-collision-algorithm: this uses the hash table and a ASCII value encoded to
generate better hashes for the kmer offset and to look for the same and implements
a way to avoid the collision of the hashes.

* */

fn main() {
    let args = GraphOffsetArgs::parse();
    let suffix_output = graph_array(&args.fastqfile, args.offsetsize).unwrap();
    println!("The graph lookup table has been written: {}", suffix_output);
}

fn graph_array(path: &str, offsetsize: usize) -> Result<String, Box<dyn Error>> {
    let file_appear = File::open(path).expect("file not present");
    let fileread_appear = BufReader::new(file_appear);
    let mut filecontent: Vec<String> = Vec::new();
    let mut filecontent_seq: Vec<String> = Vec::new();
    for i in fileread_appear.lines() {
        let line = i.expect("line not present");
        if line.starts_with("@") {
            filecontent.push(line)
        } else if line.starts_with("A")
            || line.starts_with("T")
            || line.starts_with("G")
            || line.starts_with("C")
        {
            filecontent_seq.push(line)
        }
    }

    let mut offset_hold: HashSet<String> = HashSet::new();
    for i in filecontent_seq.iter() {
        let sequence_hash: Vec<String> = i
            .chars()
            .map(String::from)
            .collect::<Vec<String>>()
            .windows(offsetsize)
            .map(|x| x.join("").to_string())
            .collect::<Vec<String>>();
        for i in sequence_hash {
            offset_hold.insert(i);
        }
    }

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    struct SequenceOffSet {
        offset: usize,
        sequence: String,
        start: Vec<usize>,
        end: Vec<usize>,
    }

    // finding the offset and then i thought since there will be collision hashes,
    // so i implemented and devised this that make the searched space unique and
    // then use that to calculate the hash-indices and then if that is equal and also
    // the unique count then the hashes are the same.
    //
    // So my algorithm first find the hashes of the search string, then since there
    // might be collisions also, so to avoid the collision, i implmented a way that
    // it will make the graph offset key values unique and then multiply with the ASCII
    // code and then it does the same for the find iter and then if the hashes are the same
    // and also the unqiue value then it put them into the BTreeMap.

    let a: usize = 65usize;
    let t: usize = 84usize;
    let g: usize = 71usize;
    let c: usize = 67usize;

    // I am storing the hashes as a BTreeMap key so that if you have a hashlookup,
    // you can compare and fetch easily the corresponding BTreeMap.

    let mut graph_hash_collision_free: BTreeMap<usize, SequenceOffSet> = BTreeMap::new();

    // My hash comparison algorithm for the graph lookup table.
    for offset in offset_hold.iter() {
        let mut starthold: Vec<usize> = Vec::new();
        let mut endhold: Vec<usize> = Vec::new();
        let mut vectorspace: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for seq in filecontent_seq.iter() {
            let offset: String = offset.to_string();
            let searchspace: String = seq.to_string();
            for i in Regex::new(r"offset").unwrap().find_iter(&searchspace) {
                starthold.push(i.start());
                endhold.push(i.end());
            }
            for i in 0..starthold.len() {
                let mut seqhold: Vec<String> = Vec::new();
                seqhold.push(seq[starthold[i]..endhold[i]].to_string());
                vectorspace.insert(seq.to_string(), seqhold);
            }
        }
        let mut offsetcount: Vec<(String, usize)> = Vec::new();
        let a_hold: (String, usize) =
            ("A".to_string(), offset.to_string().matches("A").count() * a);
        let t_hold: (String, usize) =
            ("T".to_string(), offset.to_string().matches("T").count() * t);
        let g_hold: (String, usize) =
            ("G".to_string(), offset.to_string().matches("G").count() * g);
        let c_hold: (String, usize) =
            ("C".to_string(), offset.to_string().matches("C").count() * c);
        offsetcount.push(a_hold);
        offsetcount.push(t_hold);
        offsetcount.push(g_hold);
        offsetcount.push(c_hold);

        // final offset count coming from the unique values of the hashtable offset.
        let mut finaloffset_hash: usize = 0usize;
        for i in offsetcount.iter() {
            finaloffset_hash += i.1
        }
        let mut vectorsetcount: Vec<(String, usize)> = Vec::new();
        for (str, vec) in vectorspace.iter() {
            for spaceiter in vec.iter() {
                let a_hold: (String, usize) = (
                    "A".to_string(),
                    spaceiter.to_string().matches("A").count() * a,
                );
                let t_hold: (String, usize) = (
                    "T".to_string(),
                    spaceiter.to_string().matches("T").count() * t,
                );
                let g_hold: (String, usize) = (
                    "G".to_string(),
                    spaceiter.to_string().matches("G").count() * g,
                );
                let c_hold: (String, usize) = (
                    "C".to_string(),
                    spaceiter.to_string().matches("C").count() * c,
                );
                vectorsetcount.push(a_hold);
                vectorsetcount.push(t_hold);
                vectorsetcount.push(g_hold);
                vectorsetcount.push(c_hold);

            // hashcount for the searches offset also taking into account the
            // unqiue value so that it confirms both at the unique values as
            // well as at the hash table to avoid any collisions.
            //
                let mut vectoroffset_hash: usize = 0usize;
                for i in vectorsetcount.iter() {
                    vectoroffset_hash += i.1
                }

                if vectoroffset_hash == finaloffset_hash {
                    graph_hash_collision_free.insert(
                        finaloffset_hash,
                        SequenceOffSet {
                            offset: finaloffset_hash,
                            sequence: str.to_string(),
                            start: starthold.clone(),
                            end: endhold.clone(),
                        },
                    );
                }
            }
        }
    }
    Ok("The graph collision free table has been written".to_string())
}
