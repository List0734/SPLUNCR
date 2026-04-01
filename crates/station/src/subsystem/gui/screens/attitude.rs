use std::f32::consts::TAU;
use egui::{Align2, Color32, FontId, Painter, Pos2, Rect, Sense, Stroke, Ui, Vec2};

const SKY: Color32 = Color32::from_rgb(0, 102, 204);
const GROUND: Color32 = Color32::from_rgb(139, 90, 43);
const TAPE_HEIGHT: f32 = 28.0;
const TAPE_WIDTH: f32 = 40.0;

const PAD: f32 = 2.0;

pub fn attitude_indicator(ui: &mut Ui, size: f32, roll_deg: f64, pitch_deg: f64, depth: f64, heading_deg: f64) {
    let total = Vec2::new(TAPE_WIDTH + size + PAD, size + TAPE_HEIGHT + PAD);
    let (response, mut painter) = ui.allocate_painter(total, Sense::hover());
    let rect = response.rect;

    let heading_rect = Rect::from_min_size(
        Pos2::new(rect.left() + TAPE_WIDTH, rect.top()),
        Vec2::new(size, TAPE_HEIGHT),
    );
    let depth_rect = Rect::from_min_size(
        Pos2::new(rect.left(), rect.top() + TAPE_HEIGHT),
        Vec2::new(TAPE_WIDTH, size),
    );
    let ball_rect = Rect::from_min_size(
        Pos2::new(rect.left() + TAPE_WIDTH, rect.top() + TAPE_HEIGHT),
        Vec2::splat(size),
    );

    draw_ball(&painter, &ball_rect, roll_deg, pitch_deg);
    draw_heading_tape(&mut painter, &heading_rect, heading_deg);
    draw_depth_tape(&mut painter, &depth_rect, depth);
}

fn draw_ball(painter: &Painter, rect: &Rect, roll_deg: f64, pitch_deg: f64) {
    let center = rect.center();
    let radius = rect.width() / 2.0;
    let px_per_deg = radius / 30.0;

    let pitch_offset = pitch_deg as f32 * px_per_deg;
    let roll_rad = -(roll_deg as f32).to_radians();

    let rotate = |ox: f32, oy: f32| -> Pos2 {
        let (sin_r, cos_r) = roll_rad.sin_cos();
        Pos2::new(
            center.x + ox * cos_r - oy * sin_r,
            center.y + ox * sin_r + oy * cos_r,
        )
    };

    draw_sky_and_ground(painter, center, radius, pitch_offset, &rotate);
    draw_pitch_ladder(painter, pitch_offset, radius, px_per_deg, &rotate);
    draw_aircraft_symbol(painter, center, roll_deg);
    draw_roll_pointer(painter, center, radius);
    painter.circle_stroke(center, radius, Stroke::new(1.5, Color32::WHITE));
}

fn draw_sky_and_ground(painter: &Painter, center: Pos2, radius: f32, pitch_offset: f32, rotate: &dyn Fn(f32, f32) -> Pos2) {
    painter.circle_filled(center, radius, SKY);

    let ground = ground_polygon(radius, pitch_offset, rotate);
    if !ground.is_empty() {
        painter.add(egui::Shape::convex_polygon(ground, GROUND, Stroke::NONE));
    }
}

fn ground_polygon(radius: f32, pitch_offset: f32, rotate: &dyn Fn(f32, f32) -> Pos2) -> Vec<Pos2> {
    if pitch_offset >= radius {
        return vec![];
    }
    if pitch_offset <= -radius {
        return (0..32).map(|i| {
            let angle = i as f32 * TAU / 32.0;
            rotate(radius * angle.cos(), radius * angle.sin())
        }).collect();
    }

    let half_chord = (radius * radius - pitch_offset * pitch_offset).sqrt();

    let right_angle = pitch_offset.atan2(half_chord);
    let mut left_angle = pitch_offset.atan2(-half_chord);
    if left_angle < right_angle {
        left_angle += TAU;
    }

    let arc_steps = 24;
    let mut points = vec![rotate(half_chord, pitch_offset)];
    for i in 1..=arc_steps {
        let t = i as f32 / arc_steps as f32;
        let angle = right_angle + t * (left_angle - right_angle);
        points.push(rotate(radius * angle.cos(), radius * angle.sin()));
    }
    points
}

fn draw_pitch_ladder(painter: &Painter, pitch_offset: f32, radius: f32, px_per_deg: f32, rotate: &dyn Fn(f32, f32) -> Pos2) {
    let font = FontId::monospace(9.0);

    let horizon_half = circle_clamp(radius, pitch_offset);
    if horizon_half > 0.0 {
        painter.line_segment(
            [rotate(-horizon_half, pitch_offset), rotate(horizon_half, pitch_offset)],
            Stroke::new(2.0, Color32::WHITE),
        );
    }

    for deg in (-90..=90).step_by(10) {
        if deg == 0 {
            continue;
        }
        let oy = pitch_offset - deg as f32 * px_per_deg;
        let max_half = circle_clamp(radius, oy);
        if max_half <= 0.0 {
            continue;
        }
        let tick_half = if deg % 20 == 0 { 25.0_f32 } else { 15.0_f32 };
        let tick_half = tick_half.min(max_half);

        painter.line_segment(
            [rotate(-tick_half, oy), rotate(tick_half, oy)],
            Stroke::new(1.0, Color32::WHITE),
        );

        if deg % 20 == 0 && max_half - tick_half > 18.0 {
            painter.text(
                rotate(tick_half + 12.0, oy),
                Align2::CENTER_CENTER,
                format!("{}", deg),
                font.clone(),
                Color32::WHITE,
            );
        }
    }
}

fn circle_clamp(radius: f32, oy: f32) -> f32 {
    (radius * radius - oy * oy).max(0.0).sqrt()
}

fn draw_aircraft_symbol(painter: &Painter, center: Pos2, roll_deg: f64) {
    let wing = 30.0_f32;
    let gap = 8.0_f32;
    let stroke = Stroke::new(2.0, Color32::YELLOW);

    painter.line_segment(
        [Pos2::new(center.x - wing, center.y), Pos2::new(center.x - gap, center.y)],
        stroke,
    );
    painter.line_segment(
        [Pos2::new(center.x + gap, center.y), Pos2::new(center.x + wing, center.y)],
        stroke,
    );
    painter.circle_stroke(center, 3.0, stroke);

    painter.text(
        Pos2::new(center.x, center.y + 14.0),
        Align2::CENTER_TOP,
        format!("{:.0}\u{00B0}", roll_deg),
        FontId::monospace(10.0),
        Color32::YELLOW,
    );
}

fn draw_roll_pointer(painter: &Painter, center: Pos2, radius: f32) {
    let y = center.y - radius;
    painter.line_segment(
        [Pos2::new(center.x - 6.0, y - 8.0), Pos2::new(center.x, y)],
        Stroke::new(2.0, Color32::YELLOW),
    );
    painter.line_segment(
        [Pos2::new(center.x + 6.0, y - 8.0), Pos2::new(center.x, y)],
        Stroke::new(2.0, Color32::YELLOW),
    );
}

fn draw_heading_tape(painter: &mut Painter, rect: &Rect, heading_deg: f64) {
    painter.set_clip_rect(*rect);

    let center_x = rect.center().x;
    let px_per_deg = rect.width() / 2.0 / 45.0;
    let heading = heading_deg as f32;

    draw_heading_ticks(painter, rect, center_x, heading, px_per_deg);
    draw_heading_pointer(painter, center_x, rect.bottom());
}

fn draw_heading_ticks(painter: &Painter, rect: &Rect, center_x: f32, heading: f32, px_per_deg: f32) {
    let font = FontId::monospace(9.0);
    let tick_stroke = Stroke::new(1.0, Color32::WHITE);

    let start = (heading - 50.0).floor() as i32;
    let end = (heading + 50.0).ceil() as i32;

    for deg in start..=end {
        let mut display = deg % 360;
        if display > 180 { display -= 360; }
        if display <= -180 { display += 360; }

        let x = center_x + (deg as f32 - heading) * px_per_deg;

        if deg % 30 == 0 {
            painter.line_segment(
                [Pos2::new(x, rect.bottom() - 10.0), Pos2::new(x, rect.bottom())],
                tick_stroke,
            );
            painter.text(
                Pos2::new(x, rect.top() + 2.0),
                Align2::CENTER_TOP,
                format!("{}\u{00B0}", display),
                font.clone(),
                Color32::WHITE,
            );
        } else if deg % 10 == 0 {
            painter.line_segment(
                [Pos2::new(x, rect.bottom() - 6.0), Pos2::new(x, rect.bottom())],
                tick_stroke,
            );
        }
    }
}

fn draw_heading_pointer(painter: &Painter, center_x: f32, bottom: f32) {
    painter.line_segment(
        [Pos2::new(center_x - 8.0, bottom), Pos2::new(center_x + 8.0, bottom)],
        Stroke::new(2.0, Color32::YELLOW),
    );
}

fn draw_depth_tape(painter: &mut Painter, rect: &Rect, depth: f64) {
    painter.set_clip_rect(*rect);

    let center_y = rect.center().y;
    let px_per_m = rect.height() / 2.0 / 10.0;
    let depth_f = depth as f32;

    draw_depth_ticks(painter, rect, center_y, depth_f, px_per_m);
    draw_depth_pointer(painter, rect.right(), center_y);
    draw_depth_readout(painter, rect, depth);
}

fn draw_depth_ticks(painter: &Painter, rect: &Rect, center_y: f32, depth: f32, px_per_m: f32) {
    let font = FontId::monospace(9.0);
    let tick_stroke = Stroke::new(1.0, Color32::WHITE);

    let start = (depth - 12.0).floor() as i32;
    let end = (depth + 12.0).ceil() as i32;

    let readout_zone = rect.bottom() - 20.0;

    for m in start..=end {
        let y = center_y - (m as f32 - depth) * px_per_m;

        if m % 5 == 0 {
            painter.line_segment(
                [Pos2::new(rect.right() - 8.0, y), Pos2::new(rect.right(), y)],
                tick_stroke,
            );
            if y < readout_zone {
                painter.text(
                    Pos2::new(rect.right() - 10.0, y),
                    Align2::RIGHT_CENTER,
                    format!("{}", m),
                    font.clone(),
                    Color32::WHITE,
                );
            }
        } else {
            painter.line_segment(
                [Pos2::new(rect.right() - 5.0, y), Pos2::new(rect.right(), y)],
                tick_stroke,
            );
        }
    }
}

fn draw_depth_pointer(painter: &Painter, right: f32, center_y: f32) {
    painter.line_segment(
        [Pos2::new(right - 12.0, center_y), Pos2::new(right, center_y)],
        Stroke::new(3.0, Color32::YELLOW),
    );
}

fn draw_depth_readout(painter: &Painter, rect: &Rect, depth: f64) {
    painter.text(
        Pos2::new(rect.center().x, rect.bottom() - 4.0),
        Align2::CENTER_BOTTOM,
        format!("{:.1}m", depth),
        FontId::monospace(11.0),
        Color32::YELLOW,
    );
}
