enum TFTPCommand{
    GET,
    PUT
}

struct Parser{
    cur_token : String 
}

impl Parser{
    fn new()->Self{
        Parser{

        }
    }

    fn parse(&mut self, input : &str) -> Result<(TFTPCommand, Option<Vec<&str>>), String>{
        for i in input.chars(){
            match i{
                ' ' => {


                },
                'a' ...'z' => {
                },
                '0' ...'9' => {

                },
                _ => {}
            }
        }

        Err("".to_string())
    } 
}
