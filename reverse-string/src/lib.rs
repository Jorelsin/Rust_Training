pub fn reverse(input: &str) -> String {
let mut reversed = String::from("");
let mut original = input.to_owned();
for _x in 0..original.len(){
    let _letter = original.get_mut(0..1).into_string();
    reversed.push(_letter);
}
reversed
}
