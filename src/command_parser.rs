use regex::Regex;
pub enum TFTPCommand{
    GET,
    PUT
}

pub struct Parser{
    get_regex : Regex,
    put_regex : Regex,
}

impl Parser{
    pub fn new()->Self{
        Parser{
            get_regex : Regex::new(r"get\s([\w\s]+)").unwrap(),
            put_regex : Regex::new(r"put\s[\w\s]+").unwrap(),

        }
    }

    pub fn parse(&mut self, input : &str) -> Result<(TFTPCommand, Option<Vec<&str>>), String>{
        if self.get_regex.is_match(input){
            let c = self.get_regex.captures(input).unwrap();
            println!("{}", c.len());
            println!("{:?}", c.at(1));
        }
        Err("".to_string())
    } 
}
