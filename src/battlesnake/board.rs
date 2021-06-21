use std::convert::TryFrom;

use super::point::Point;
use super::snake::Snake;
use super::input::GameInfo;

pub struct Board {
    pub snakes: Vec<Snake>,
    pub food: Vec<Point>,
    bound: Point,
}

impl TryFrom<&GameInfo> for Board {
    type Error = &'static str;

    fn try_from(input: &GameInfo) -> Result<Self, Self::Error> {
        
        if value <= 0 {
            Err("GreaterThanZero only accepts value superior than zero!")
        } else {
            Ok(GreaterThanZero(value))
        }
    }
}