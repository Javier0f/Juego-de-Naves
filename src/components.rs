use macroquad::math::Vec2;

#[derive(Debug)]
pub struct Position(pub Vec2);
#[derive(Debug)]
pub struct Size(pub f32);
#[derive(Debug)]
pub struct Sides(pub u8);
#[derive(Debug)]
pub struct Speed(pub f32);
#[derive(Debug)]
pub struct Direction(pub Vec2);
#[derive(Debug)]
pub struct Rotation(pub f32);
#[derive(Debug)]
pub struct Life(pub u8);

pub struct Player;
pub struct Asteroid;