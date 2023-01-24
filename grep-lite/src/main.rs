use regex::Regex; 
fn main() {
    let re = Regex::new("picture").unwrap(); //unwrap is for result
    // let search_term = "picture";
    let quote = "Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what? 
It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {

        let contains_substring = re.find(line); 
        match contains_substring {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

//same output. diff by using match and regex