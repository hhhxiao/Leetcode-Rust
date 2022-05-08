pub fn replace_space(s: String) -> String {
    let mut res :String = String::new();
    for i in s.chars(){
        match  i {
            ' ' =>{res.push_str("%20");},
            _ => {res.push(i)}
        }
    }    
    res
}