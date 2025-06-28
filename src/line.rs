// Copyright © 2022-2025
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

use rayca_math::{Color, Point3};

#[repr(C)]
pub struct LineVertex {
    pub pos: Point3,
    pub color: Color,
}

impl LineVertex {
    pub fn new(pos: Point3, color: Color) -> Self {
        Self { pos, color }
    }
}

#[repr(C)]
pub struct Line {
    pub a: LineVertex,
    pub b: LineVertex,
}

impl Line {
    pub fn new(a: LineVertex, b: LineVertex) -> Line {
        Line { a, b }
    }
}
