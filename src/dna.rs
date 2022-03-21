use std::collections::HashMap;

struct _Nucleobase {
    symbol: char,
}

impl _Nucleobase {
    fn _complement(&self) -> char {
        match &self.symbol {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            _ => '_',
        }
    }
}

pub fn _count_nucleotides(nt_str: &str) -> String {
    let samples: Vec<&str> = nt_str.split("").collect();
    let mut nt_map: HashMap<String, i32> = HashMap::new();

    for i in samples.iter() {
        _set_if_needed(&mut nt_map, i.to_string());
    }

    format!(
        "{} {} {} {}",
        nt_map["A"], nt_map["C"], nt_map["G"], nt_map["T"]
    )
}

fn _set_if_needed(hmap: &mut HashMap<String, i32>, k: String) {
    let val = *hmap.entry(k.clone()).or_insert(0) + 1;
    hmap.insert(k, val);
}

pub fn _convert_to_rna(dna: &str) -> String {
    dna.replace("T", "U")
}

pub fn _reverse_complement_strand(dna: &str) -> String {
    let chars: Vec<char> = dna.chars().collect();
    let mut compl_strand: Vec<char> = vec![];

    for b in chars {
        let nb: _Nucleobase = _Nucleobase { symbol: b };
        compl_strand.push(nb._complement());
    }

    compl_strand.reverse();
    compl_strand.iter().collect()
}
