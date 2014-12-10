use std::string::String;
use std::vec::Vec;

pub struct PlayerStruct
{
    player_name: String,
    player_ip: String,
    bullets: Vec<BulletStruct>,
    movement: MovementStruct,
    shape: ShapeStruct
}

pub struct BulletStruct
{
    owner: String,
    movement: MovementStruct,
    shape: ShapeStruct
}

pub struct ShapeStruct
{
    width: int,
    heigth: int,
    rotation: int
}

pub struct MovementStruct
{
    location_x: int,
    location_y: int,
    slope_x: int,
    slope_y: int
}
