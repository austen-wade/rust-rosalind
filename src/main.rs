mod dna;

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dna_test() {
        assert_eq!(
            "20 12 17 21".to_string(),
            dna::_count_nucleotides(
                "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC"
            )
        );
    }

    #[test]
    fn dna_to_rna_test() {
        assert_eq!(
            "GAUGGAACUUGACUACGUAAAUU",
            dna::_convert_to_rna("GATGGAACTTGACTACGTAAATT")
        );
    }

    #[test]
    fn reverse_complement_dna_test() {
        assert_eq!("ACCGGGTTTT", dna::_reverse_complement_strand("AAAACCCGGT"));
    }
}
