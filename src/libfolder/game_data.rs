use std::string::String;
use std::vec::Vec;
use std::bool;

pub struct PlayerStruct
{
    player_name: String,
    player_ip: String,
    bullets: Vec<BulletStruct>,
    movement: MovementStruct,
    shape: ShapeStruct,
    is_alive: bool,
}

pub struct BulletStruct
{
    owner_name: String,
    movement: MovementStruct,
    shape: ShapeStruct,
    is_alive: bool,
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

pub trait Bullet
{
    fn new(owner: String,
        movement_struct: MovementStruct,
        shape_struct: ShapeStruct) -> BulletStruct;
}

impl Bullet for BulletStruct
{
    fn new(owner: String,
        movement_struct: MovementStruct,
        shape_struct: ShapeStruct) -> BulletStruct
    {
        return BulletStruct{
            owner_name: owner,
            movement: movement_struct,
            shape: shape_struct,
            is_alive: true
        };
    }
}

pub trait Player
{
    fn new(name: String,
        ip: String,
        movement_struct: MovementStruct,
        shape_struct: ShapeStruct) -> PlayerStruct;
}

impl Player for PlayerStruct
{
    fn new(name: String,
        ip: String,
        movement_struct: MovementStruct,
        shape_struct: ShapeStruct) -> PlayerStruct
    {
        return PlayerStruct{
            player_name: name,
            player_ip: ip,
            bullets: Vec::new(),
            movement: movement_struct,
            shape: shape_struct,
            is_alive: true
        };
    }
}

pub trait Sprite
{
    fn detect_movement(&mut self);
    fn kill_self(&mut self);
    fn move_sprite(&mut self);
    fn update_shape(&mut self);
}

pub trait Movement
{
    fn new(location_x: int, location_y: int, slope_x: int, slope_x: int) -> MovementStruct;
    fn update_location(&mut self, location_x: int, location_y: int);
    fn update_slope(&mut self, slope_x: int, slope_x: int);
    fn apply_slope(&mut self);
}

impl Movement for MovementStruct
{
    fn new(l_x: int, l_y: int, s_x: int, s_y: int) -> MovementStruct
    {
        return MovementStruct{location_x: l_x, location_y: l_y, slope_x: s_x, slope_y: s_y};
    }
    fn update_location(&mut self, location_x: int, location_y: int)
    {
        self.location_x = location_x;
        self.location_y = location_y;
    }
    fn update_slope(&mut self, slope_x: int, slope_y: int)
    {
        self.slope_x = slope_x;
        self.slope_y = slope_y;
    }
    fn apply_slope(&mut self)
    {
        self.location_x += self.slope_x;
        self.location_y += self.slope_y;
    }
}

pub trait Shape
{
    fn new(width: int, heigth: int, rotation: int) -> ShapeStruct;
    fn rotate(&mut self, degrees_of_rotation: int);
    fn change_dimensions(&mut self, width: int, heigth: int);
}

impl Shape for ShapeStruct
{
    fn new(w: int, h: int, rot: int) -> ShapeStruct
    {
        return ShapeStruct{width: w, heigth: h, rotation: rot};
    }
    fn rotate(&mut self, degrees_of_rotation: int)
    {
        self.rotation += degrees_of_rotation;
    }
    fn change_dimensions(&mut self, width: int, heigth: int)
    {
        self.width = width;
        self.heigth = heigth;
    }
}
