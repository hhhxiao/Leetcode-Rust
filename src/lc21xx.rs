pub fn capitalize_title(title: String) -> String {
    let mut ss = title.split(' ');
    let res = String::new();


    let mut ve = Vec < String > ();
    for str in ss {
        let mut ascii = str.to_ascii_lowercase().into_bytes();
        if ascii.len() > 3 {
            ascii[0] += ('A' as u8 - 'a' as u8);
        }
    }

    print!("{:?}", ve);
    res
}