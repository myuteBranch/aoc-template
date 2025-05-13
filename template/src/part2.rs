use nom::{
    IResult, Parser, bytes::complete::tag, bytes::complete::take_while_m_n, combinator::map_res,
};
use tracing::info;
pub fn solve(input: &str) -> Result<i32, String> {
    let res = parse_input(input).unwrap();
    info!("{:?}", res);
    Ok(0)
}

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn parse_input(_ : &str) -> Result<Color, String> {
    let (_, color) = hex_color("#2F14DF").unwrap();
    Ok(color)
}
fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex).parse(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = (hex_primary, hex_primary, hex_primary).parse(input)?;

    Ok((input, Color { red, green, blue }))
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn it_works() {
        let input = r"input_here";
        let out = solve(input);
        assert_eq!(Ok(0), out)
    }

}

