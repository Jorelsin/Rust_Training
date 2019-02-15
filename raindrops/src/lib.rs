pub fn raindrops(n: u32) -> String {

    let mut raindrop = String::from("");
    if n%3 == 0{
        raindrop.push_str("Pling");
    }

    if n%5 == 0{
        raindrop.push_str("Plang");
    }

    if n%7 == 0{        
        raindrop.push_str("Plong");

    }else{
                
    }

    if raindrop.is_empty(){
        return n.to_string();
    }else{
        return raindrop;
    }
}
