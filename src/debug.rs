#![allow(dead_code)]
use crate::config::SCALE;
use macroquad::experimental::collections::storage;
use macroquad::prelude::*;
pub enum DebugData {
    DebugGlyph(DebugGlyph),
    DebugText { key: String, value: String },
}

pub struct DebugGlyph {
    kind: GlyphKind,
    color: Color,
    args_vec: Vec<Vec3>,
    args_f32: Vec<f32>,
}

enum GlyphKind {
    Circle,
    Line,
}

impl DebugData {
    pub fn circle(pos: Vec3, radius: f32, color: Color) -> Self {
        DebugData::DebugGlyph(DebugGlyph {
            kind: GlyphKind::Circle,
            args_vec: vec![pos],
            args_f32: vec![radius],
            color,
        })
    }

    pub fn line(start: Vec3, end: Vec3, color: Color) -> Self {
        DebugData::DebugGlyph(DebugGlyph {
            kind: GlyphKind::Line,
            args_vec: vec![start, end],
            args_f32: vec![],
            color,
        })
    }

    pub fn text(key: &str, value: String) -> Self {
        DebugData::DebugText {
            key: key.to_owned(),
            value,
        }
    }

    pub fn draw_xy(&self) {
        match self {
            DebugData::DebugGlyph(glyph) => match glyph.kind {
                GlyphKind::Circle => {
                    draw_circle(
                        glyph.args_vec[0].x * SCALE,
                        glyph.args_vec[0].y * SCALE,
                        glyph.args_f32[0] * SCALE,
                        glyph.color,
                    );
                }
                GlyphKind::Line => {
                    draw_line(
                        glyph.args_vec[0].x * SCALE,
                        glyph.args_vec[0].y * SCALE,
                        glyph.args_vec[1].x * SCALE,
                        glyph.args_vec[1].y * SCALE,
                        10.0,
                        glyph.color,
                    );
                }
            },
            _ => {}
        };
    }

    pub fn draw_zy(&self) {
        match self {
            DebugData::DebugGlyph(glyph) => match glyph.kind {
                GlyphKind::Circle => {
                    draw_circle(
                        glyph.args_vec[0].z * SCALE + 400.0,
                        glyph.args_vec[0].y * SCALE,
                        glyph.args_f32[0] * SCALE,
                        glyph.color,
                    );
                }
                GlyphKind::Line => {
                    draw_line(
                        glyph.args_vec[0].z * SCALE + 400.0,
                        glyph.args_vec[0].y * SCALE,
                        glyph.args_vec[1].z * SCALE + 400.0,
                        glyph.args_vec[1].y * SCALE,
                        10.0,
                        glyph.color,
                    );
                }
            },
            _ => {}
        };
    }
}

pub fn draw_debug() {
    let mut debug = storage::get_mut::<Vec<DebugData>>();
    let mut y = 0;
    for item in debug.iter() {
        match item {
            DebugData::DebugGlyph(_) => {
                item.draw_xy();
                item.draw_zy();
            }
            DebugData::DebugText { key, value } => {
                draw_text(
                    &format!("{}: {}", key, value),
                    0.0,
                    21.0 + y as f32 * 42.0,
                    42.0,
                    WHITE,
                );
                y += 1;
            }
        }
        item.draw_xy();
    }
    debug.clear();
}
