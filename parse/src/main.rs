use nom::{Parser, IResult};
use nom::branch::alt;
use nom::bytes::complete::tag;
fn hello_parse<I,O,E>(input:I) -> Result<(I,O),E>{
    if input.starts_with("hello"){
        Ok((&input["hello".len()..],"hello"))
    }else{
        Err("parse fail")
    }
}

fn hello_parser(input:&str) -> IResult<&str,&str>{
    tag("hello world")(input)
}
fn common_parser(input:&str) -> IResult<&str,&str>{
    tag(",")(input)
}
fn world_parser(input:&str) -> IResult<&str,&str>{
    tag("world")(input)
}
fn parser(input:&str) -> IResult<&str,(&str,&str,&str)> {
    match hello_parser(input){
        OK((input,output_hello)) => {
            match common_parser(input){
                Ok((input,out_common)){
                    match world_parser(input){
                        OK((input,out_world)) => {
                            Ok((input,(output_hello,out_common,out_world)))
                        }
                        Err(e) => Err(e)
                    }
                }
                Err(e) => Err(e)
            }
        }
        Err(e) => Err(e)
    }
}

fn parser_2(input:&str) -> IResult<&str,(&str,(&str,&str))>{
    pair(tag("hello"),pair(tag(","),tag("world")))(input)
}


fn main() {
    assert_eq!(parser("hello world"),Ok(("",("hello",(",","world")))));
}
