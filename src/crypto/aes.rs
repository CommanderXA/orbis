pub fn vec_to_string(arr: Vec<u8>) -> String {
    let string = arr
        .clone()
        .into_iter()
        .map(|b| {
            let mut bn = b.to_string();
            bn.push_str(",");
            bn
        })
        .collect::<String>();
    string
}

pub fn string_to_vec(string: String) -> Vec<u8> {
    let arr = string.rsplit(",").collect::<Vec<&str>>();
    let arr = arr
        .iter()
        .rev()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    arr
}
