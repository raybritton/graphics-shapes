use crate::coord::Coord;
use crate::Shape;
#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};
use crate::circle::Circle;
use crate::rect::Rect;

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum LineType {
    Point,
    Horizontal,
    Vertical,
    Angled,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Line {
    start: Coord,
    end: Coord,
    len: usize,
    line_type: LineType,
    angle: isize,
}

impl Line {
    pub fn new<P1: Into<Coord>, P2: Into<Coord>>(start: P1, end: P2) -> Self {
        let start = start.into();
        let end = end.into();
        let line_type = if start == end {
            LineType::Point
        } else if start.x == end.x {
            LineType::Horizontal
        } else if start.y == end.y {
            LineType::Vertical
        } else {
            LineType::Angled
        };
        let len = start.distance(end);
        let angle = start.angle_to(end);
        Self {
            start,
            end,
            len,
            line_type,
            angle,
        }
    }
}

impl Line {
    #[allow(clippy::len_without_is_empty)] //use start()==end() to check that
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn angle(&self) -> isize {
        self.angle
    }

    #[inline]
    pub fn start(&self) -> Coord {
        self.start
    }

    #[inline]
    pub fn end(&self) -> Coord {
        self.end
    }

    #[inline]
    pub fn line_type(&self) -> LineType {
        self.line_type
    }
}

impl Shape for Line {
    fn from_points(points: Vec<Coord>) -> Self
    where
        Self: Sized,
    {
        debug_assert!(points.len() >= 2);
        Line::new(points[0], points[1])
    }

    fn contains<P: Into<Coord>>(&self, point: P) -> bool {
        let point = point.into();
        match self.line_type {
            LineType::Point => self.start == point,
            LineType::Horizontal => {
                self.start.y == point.y && self.start.x <= point.x && point.x <= self.end.x
            }
            LineType::Vertical => {
                self.start.x == point.x && self.start.y <= point.y && point.y <= self.end.y
            }
            LineType::Angled => self.start.distance(point) + self.end.distance(point) == self.len,
        }
    }

    fn points(&self) -> Vec<Coord> {
        vec![self.start, self.end]
    }

    fn center(&self) -> Coord {
        self.start.mid_point(self.end)
    }

    #[inline]
    fn left(&self) -> isize {
        self.start.x
    }

    #[inline]
    fn right(&self) -> isize {
        self.end.x
    }

    #[inline]
    fn top(&self) -> isize {
        self.start.y
    }

    #[inline]
    fn bottom(&self) -> isize {
        self.end.y
    }
}

impl Line {
    pub fn as_rect(&self) -> Rect {
        Rect::new(self.start, self.end)
    }

    pub fn as_circle(&self) -> Circle {
        Circle::new(self.start, self.start.distance(self.end))
    }
}