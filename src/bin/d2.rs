use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult,
};
use std::{cmp::max, error::Error, fs::File, io::Read};

struct Game {
    id: u32,
    draws: Vec<Draw>,
}

struct Draw {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_color(input: &str) -> IResult<&str, (Option<u32>, &str)> {
    let (input, count) = map_res(digit1, str::parse)(input)?;
    let (input, _) = space1(input)?;
    let (input, color) = alt((tag("red"), tag("green"), tag("blue")))(input)?;
    Ok((input, (Some(count), color)))
}

fn parse_draw(input: &str) -> IResult<&str, Draw> {
    let (input, colors) = separated_list1(tag(", "), parse_color)(input)?;

    let mut draw = Draw {
        red: None,
        green: None,
        blue: None,
    };

    for (count, color) in colors {
        match color {
            "red" => draw.red = count,
            "green" => draw.green = count,
            "blue" => draw.blue = count,
            _ => {}
        }
    }

    Ok((input, draw))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) = delimited(tag("Game "), parse_number, char(':'))(input)?;

    let (input, draws) = preceded(char(' '), separated_list1(tag("; "), parse_draw))(input)?;

    Ok((input, Game { id, draws }))
}

fn get_game_max_rgb(line: &str) -> (u32, (u32, u32, u32)) {
    let game = parse_game(line).unwrap().1;
    (
        game.id,
        game.draws.into_iter().fold(
            (0, 0, 0),
            |(old_r, old_g, old_b), Draw { red, green, blue }| {
                (
                    max(old_r, red.unwrap_or(0)),
                    max(old_g, green.unwrap_or(0)),
                    max(old_b, blue.unwrap_or(0)),
                )
            },
        ),
    )
}

fn parse_line_1(line: &str) -> Option<u32> {
    let (game_id, (max_r, max_g, max_b)) = get_game_max_rgb(line);
    (max_r <= 12 && max_g <= 13 && max_b <= 14).then_some(game_id)
}

fn parse_line_2(line: &str) -> u32 {
    let (_, (max_r, max_g, max_b)) = get_game_max_rgb(line);
    max_r * max_g * max_b
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut content = String::new();
    File::open("in/2r.txt")?.read_to_string(&mut content)?;
    println!(
        "Total 1: {}",
        content.lines().filter_map(parse_line_1).sum::<u32>()
    );
    println!(
        "Total 2: {}",
        content.lines().map(parse_line_2).sum::<u32>()
    );
    Ok(())
}
