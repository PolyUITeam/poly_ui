use nalgebra::Point2;
use nalgebra::Vector2;
use std::{boxed::Box, vec::Vec};

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub struct Line {
    pub start: Point2<u32>,
    pub end: Point2<u32>,
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub struct Rect {
    pub pos: Point2<u32>,
    pub size: Vector2<u32>,
}

//************************************************************************************************
//************************************************************************************************
//************************************************************************************************
pub trait CanvasTrait {
    fn sub_canvas(&self, pos: Point2<i32>, size: Vector2<u32>) -> Box<dyn CanvasTrait>;

    fn size(&self) -> Vector2<u32>;
    fn clear(&mut self);
    fn present(&mut self);

    fn draw_color(&self) -> Color;
    fn set_draw_color(&mut self, new: &Color);

    fn draw_point(&mut self, point: &Point2<i32>);
    fn draw_points(&mut self, points: &Vec<Point2<i32>>);
    fn draw_line(&mut self, line: &Line);
    fn draw_lines(&mut self, lines: &Vec<Line>);
    fn draw_rect(&mut self, rect: Rect);
    fn draw_rects(&mut self, rect: &Vec<Rect>);
    fn fill_rect(&mut self, rect: Rect);
    fn fill_rects(&mut self, rect: &Vec<Rect>);
}
