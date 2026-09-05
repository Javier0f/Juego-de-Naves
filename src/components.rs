use macroquad::math::Vec2;

#[derive(Debug, Copy, Clone)]
pub struct Position(pub Vec2);

#[derive(Debug, Copy, Clone)]
pub struct Size(pub f32);

#[derive(Debug, Copy, Clone)]
pub struct Sides(pub u8);

#[derive(Debug, Copy, Clone)]
pub struct Speed(pub f32);

#[derive(Debug, Copy, Clone)]
pub struct Direction(pub Vec2);

#[derive(Debug, Copy, Clone)]
pub struct Rotation(pub f32);

#[derive(Debug, Copy, Clone)]
pub struct Life(pub u8);

pub struct Player;
pub struct Asteroid;