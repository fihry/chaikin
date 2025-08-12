mod chaikin;
use chaikin::*;
use macroquad::prelude::*;
#[macroquad::main(
    "Chaikin Curve",
    window_width = 800,
    window_height = 600,
    window_resizable = true
)]
async fn main() {
    let mut points: Vec<Vec2> = Vec::new();
    let mut new_points: Vec<Vec2> = Vec::new();
    let mut show_smoothed = false;
    let mut counter = 1;
    loop {
        draw_text(
            "Click to add points, Delete to clear, Enter to smooth, Escape to exit",
            10.0,
            20.0,
            20.0,
            WHITE
        );
        draw_text(format!("animation stpes {}/7", counter / 60).as_str(), 10.0, 40.0, 20.0, YELLOW);

        // Draw raw points
        for &p in &points {
            draw_circle_lines(p.x, p.y, 4.0, 1.0, WHITE);
        }
        // Draw smoothed polyline if available
        if show_smoothed && new_points.len() > 1 {
            if counter == 480 {
                counter = 1;
                new_points = points.clone();
            }
            if counter % 60 == 0 {
                new_points = chaikin(&new_points);
            }
            counter += 1;
            draw_curve(&new_points);
        }
        if is_key_pressed(KeyCode::Escape) {
            println!("Exiting...");
            break;
        }

        if is_key_pressed(KeyCode::Delete) {
            points.clear();
            new_points.clear();
            show_smoothed = false;
            counter = 1;
        }

        if is_key_pressed(KeyCode::Enter) {
            // Start smoothing from original points
            new_points = points.clone();
            show_smoothed = true;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            points.push(vec2(x, y));
            if show_smoothed {
                new_points.push(vec2(x, y));
            }
        }

        next_frame().await;
    }
}

fn draw_curve(points: &[Vec2]) {
    for point in 0..points.len() - 1 {
        draw_line(
            points[point].x,
            points[point].y,
            points[(point + 1) % points.len()].x,
            points[(point + 1) % points.len()].y,
            1.0,
            GREEN
        );
    }
}
