use regex::Regex;

#[derive(PartialEq)]
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
            get_regex : Regex::new(r"get\s([a-z0-9/.]+\s+)").unwrap(),
            put_regex : Regex::new(r"put\s[\w\s]+").unwrap(),

        }
    }

    pub fn parse<'a>(&'a mut self, input : &'a str) -> Result<(TFTPCommand, Option<Vec<&str>>), String>{
        if self.get_regex.is_match(input){
            let c = self.get_regex.captures(input).unwrap();
            println!("{}", c.len());
            println!("{:?}", c.at(1));
            return Ok((TFTPCommand::GET, Some(c.at(1).unwrap().split_whitespace().collect())))
        } else {
            //panic!("no match");
        }
        
        Err("".to_string())
    } 
}
