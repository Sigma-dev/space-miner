use avian2d::math::PI;
use bevy::{
    math::bounding::{Aabb2d, BoundingVolume},
    prelude::*,
};
use itertools::Itertools;

use crate::rand::{random_range, random_vec2_range};

use super::line::Line;

#[derive(Clone, Debug, Default)]
pub struct LineGroup {
    lines: Vec<Line>,
}

impl LineGroup {
    pub fn new(lines: Vec<Line>) -> LineGroup {
        LineGroup { lines }
    }

    pub fn get_lines(&self) -> &Vec<Line> {
        &self.lines
    }

    pub fn from_line(line: Line) -> LineGroup {
        LineGroup::new(vec![line])
    }

    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line);
    }

    pub fn to_points(&self) -> Vec<Vec2> {
        let mut points = Vec::new();
        for line in self.lines.iter() {
            points.push(line.a);
            points.push(line.b);
        }
        points
    }

    pub fn to_unique_points(&self) -> Vec<Vec2> {
        let mut points = Vec::new();
        for line in self.lines.iter() {
            for point in [line.a, line.b] {
                if !points.contains(&point) {
                    points.push(point);
                }
            }
        }
        points
    }

    pub fn to_unique_points_looped(&self) -> Vec<Vec2> {
        let mut points = self.to_unique_points();
        points.push(*points.first().unwrap());
        return points;
    }

    pub fn concat(&self, other: LineGroup) -> LineGroup {
        let mut new = LineGroup::new(self.lines.clone());
        new.extend(other);
        new
    }

    pub fn extend(&mut self, other: LineGroup) {
        self.lines.extend(other.lines);
    }

    pub fn generate_continuous(points: Vec<Vec2>) -> LineGroup {
        LineGroup::new(
            points
                .iter()
                .tuple_windows()
                .map(|(a, b)| Line::new(*a, *b))
                .collect(),
        )
    }

    pub fn generate_continuous_closed(points: Vec<Vec2>) -> LineGroup {
        let mut line_group = LineGroup::generate_continuous(points);
        if let Some(first) = line_group.lines.first() {
            if let Some(last) = line_group.lines.last() {
                line_group.add_line(Line::new(last.b, first.a));
            }
        }
        line_group
    }

    pub fn generate_circle(radius: f32, resolution: u32) -> LineGroup {
        let points = generate_circle_points(radius, resolution);
        LineGroup::generate_continuous_closed(points)
    }

    pub fn generate_random_circle(radius: f32, resolution: u32, range: f32) -> LineGroup {
        let points = generate_circle_points(radius, resolution);
        let offset_points: Vec<Vec2> = points
            .iter()
            .map(|p| p + random_vec2_range(-range..range))
            .collect();
        LineGroup::generate_continuous_closed(offset_points)
    }

    pub fn calculate_bounding_size(&self) -> f32 {
        let aabb = self.calculate_bounding_box();
        let br = aabb.min.abs().max_element();
        let tr = aabb.max.abs().max_element();
        br.max(tr)
    }

    pub fn calculate_bounding_box(&self) -> Aabb2d {
        Aabb2d::from_point_cloud(Isometry2d::IDENTITY, &self.to_points())
    }

    pub fn scaled(&self, factor: f32) -> LineGroup {
        let mut res = LineGroup::default();
        for line in &self.lines {
            res.add_line(Line::new(line.a * factor, line.b * factor));
        }
        res
    }

    pub fn rotated(&self, rotation_degrees: f32) -> LineGroup {
        let rot = rotation_degrees.to_radians();
        let vec = Vec2::from_angle(rot);
        let mut res = LineGroup::default();
        for line in &self.lines {
            res.add_line(Line::new(vec.rotate(line.a), vec.rotate(line.b)));
        }
        res
    }

    pub fn offset(&self, offset: Vec2) -> LineGroup {
        let mut res = LineGroup::default();
        for line in &self.lines {
            res.add_line(Line::new(line.a + offset, line.b + offset));
        }
        res
    }

    pub fn flipped_vertically(&self) -> LineGroup {
        let lines = self
            .lines
            .iter()
            .map(|l| Line::new(Vec2::new(l.a.x, -l.a.y), Vec2::new(l.b.x, -l.b.y)))
            .collect();
        LineGroup::new(lines)
    }

    pub fn centered(&self) -> LineGroup {
        self.offset(-self.calculate_bounding_box().center())
    }

    pub fn scatter(self, range: f32, number: u32, randomize_rotation: bool) -> LineGroup {
        fn generate_new_offset(
            previous_offsets: &Vec<Vec2>,
            range: f32,
            min_dist: f32,
        ) -> Option<Vec2> {
            'main: for _ in 0..64 {
                let offset = Circle::new(range).sample_interior(&mut rand::thread_rng());
                for previous in previous_offsets {
                    if offset.distance(*previous) < min_dist {
                        println!("invalid");
                        continue 'main;
                    }
                }
                return Some(offset);
            }
            None
        }

        let mut res = LineGroup::default();
        let bounding_size = self.calculate_bounding_size();
        let mut previous_offsets = Vec::new();
        for _ in 0..number {
            let Some(offset): Option<Vec2> =
                generate_new_offset(&previous_offsets, range, bounding_size)
            else {
                continue;
            };
            let rotation: f32 = random_range(0.0..360.);
            let mut instance = self.clone();
            if randomize_rotation {
                instance = instance.rotated(rotation);
            }
            res.extend(instance.offset(offset));
            previous_offsets.push(offset);
        }
        res
    }

    pub fn scatter_circle(
        &self,
        radius: f32,
        number: u32,
        offset_range: f32,
        randomize_rotation: bool,
    ) -> LineGroup {
        let mut res = LineGroup::default();
        let mut previous_offsets = Vec::new();
        for n in 0..number {
            let circle_pos: Vec2 = sample_circle(n as f32 / number as f32) * radius;
            let offset = circle_pos + random_vec2_range(-offset_range..offset_range);
            let rotation: f32 = random_range(0.0..360.);
            let mut instance = self.clone();
            if randomize_rotation {
                instance = instance.rotated(rotation);
            }
            res.extend(instance.offset(offset));
            previous_offsets.push(offset);
        }
        res
    }

    pub fn letter(letter: char) -> LineGroup {
        match letter {
            'A' => LineGroup::generate_continuous(vec![
                Vec2::new(-4., -4.),
                Vec2::new(0., 4.),
                Vec2::new(4., -4.),
            ])
            .concat(LineGroup::new(vec![Line::new(
                Vec2::new(-3., 0.),
                Vec2::new(3., 0.),
            )])),
            'E' => LineGroup::generate_continuous(vec![
                Vec2::new(3., -4.),
                Vec2::new(-4., -4.),
                Vec2::new(-4., 4.),
                Vec2::new(3., 4.),
            ])
            .concat(LineGroup::from_line(Line::new(
                Vec2::new(-4., 0.),
                Vec2::new(2., 0.),
            ))),
            'F' => LineGroup::generate_continuous(vec![
                Vec2::new(-4., -4.),
                Vec2::new(-4., 4.),
                Vec2::new(3., 4.),
            ])
            .concat(LineGroup::from_line(Line::new(
                Vec2::new(-4., 0.),
                Vec2::new(2., 0.),
            ))),
            'G' => LineGroup::generate_continuous(vec![
                Vec2::new(4., 4.),
                Vec2::new(-4., 3.),
                Vec2::new(-4., -3.),
                Vec2::new(4., -4.),
                Vec2::new(4., 0.),
                Vec2::new(0., 0.),
            ]),
            'I' => LineGroup::from_line(Line::new(Vec2::new(0., -4.), Vec2::new(0., 4.))),
            'L' => LineGroup::generate_continuous(vec![
                Vec2::new(-4., 4.),
                Vec2::new(-4., -4.),
                Vec2::new(2., -4.),
            ]),
            'M' => LineGroup::generate_continuous(vec![
                Vec2::new(-4., -4.),
                Vec2::new(-2., 4.),
                Vec2::new(0., 0.),
                Vec2::new(2., 4.),
                Vec2::new(4., -4.),
            ]),
            'N' => LineGroup::generate_continuous(vec![
                Vec2::new(-4., -4.),
                Vec2::new(-4., 4.),
                Vec2::new(4., -4.),
                Vec2::new(4., 4.),
            ]),
            'O' => LineGroup::generate_continuous_closed(vec![
                Vec2::new(-2., -4.),
                Vec2::new(2., -4.),
                Vec2::new(4., 0.),
                Vec2::new(2., 4.),
                Vec2::new(-2., 4.),
                Vec2::new(-4., 0.),
            ]),
            'P' => LineGroup::generate_continuous(vec![
                Vec2::new(-3., -4.),
                Vec2::new(-3., 4.),
                Vec2::new(2., 4.),
                Vec2::new(3., 2.),
                Vec2::new(2., 0.),
                Vec2::new(-3., 0.),
            ]),
            'R' => LineGroup::generate_continuous(vec![
                Vec2::new(-3., -4.),
                Vec2::new(-3., 4.),
                Vec2::new(2., 4.),
                Vec2::new(3., 2.),
                Vec2::new(2., 0.),
                Vec2::new(-3., 0.),
                Vec2::new(3., -4.),
            ]),
            'S' => LineGroup::generate_continuous(vec![
                Vec2::new(3., 4.),
                Vec2::new(-4., 4.),
                Vec2::new(-4., 0.),
                Vec2::new(3., 0.),
                Vec2::new(3., -4.),
                Vec2::new(-4., -4.),
            ]),
            'T' => LineGroup::new(vec![
                Line::new(Vec2::new(0., -4.), Vec2::new(0., 4.)),
                Line::new(Vec2::new(-4., 4.), Vec2::new(4., 4.)),
            ]),
            'U' => LineGroup::generate_continuous(vec![
                Vec2::new(-4., 4.),
                Vec2::new(-4., -2.),
                Vec2::new(-2., -4.),
                Vec2::new(2., -4.),
                Vec2::new(4., -2.),
                Vec2::new(4., 4.),
            ]),
            'V' => LineGroup::generate_continuous(vec![
                Vec2::new(-3., 4.),
                Vec2::new(0., -4.),
                Vec2::new(3., 4.),
            ]),
            'Y' => LineGroup::generate_continuous(vec![
                Vec2::new(-3., 4.),
                Vec2::new(0., 0.),
                Vec2::new(3., 4.),
            ])
            .concat(LineGroup::from_line(Line::new(
                Vec2::new(0., 0.),
                Vec2::new(0., -4.),
            ))),
            _ => LineGroup::default(),
        }
    }

    pub fn text(text: impl Into<String>) -> LineGroup {
        let mut res = LineGroup::default();
        let mut offset = 0.;
        for c in text.into().chars() {
            res.extend(LineGroup::letter(c).offset(Vec2::new(offset, 0.)));
            offset += 10.
        }
        res
    }
}

fn sample_circle(p: f32) -> Vec2 {
    let teta = p * 2. * PI;
    Vec2::new(teta.cos(), teta.sin())
}

fn generate_circle_points(radius: f32, resolution: u32) -> Vec<Vec2> {
    let mut points = Vec::new();
    for i in 0..resolution {
        points.push(sample_circle(i as f32 / 8.) * radius)
    }
    points
}
