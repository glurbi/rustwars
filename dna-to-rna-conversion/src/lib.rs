fn dna_to_rna(dna: &str) -> String {
    dna.chars().map(|c| if c == 'T' { 'U' } else { c }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn returns_expected() {
        assert_eq!(dna_to_rna("TTTT"), "UUUU");
        assert_eq!(dna_to_rna("GCAT"), "GCAU");
    }    
}
