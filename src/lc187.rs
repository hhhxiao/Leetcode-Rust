use std::collections::HashMap;
pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    let mut res : Vec<String> = Vec::new();

    if s.len() > 10{

        let mut m = HashMap::new();
        for index in 0..(s.len() - 9){
        let sub_str = &s[index..index+10];
        let count =   m.entry(sub_str).or_insert(0);
        *count += 1;
        }

    
        for (k,v) in &m{
            match v {
                1 =>(),
                _ => res.push((*k).to_string()),
            }
        }
    
    }

    res    
}
