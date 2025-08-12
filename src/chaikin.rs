use macroquad::prelude::Vec2;
pub fn chaikin(points: &[Vec2]) -> Vec<Vec2> {
    if points.len() < 2 {
        return points.to_vec();
    }
    let mut new_points = Vec::with_capacity(points.len() * 2);
    new_points.push(points[0]);

    let percent = 0.25;

    for i in 0..points.len() - 1 {
        let p0 = points[i];
        let p1 = points[i + 1];
        // delta points for Chaikin's algorithm
        let q = p0 + (p1 - p0) * percent;
        let r = p0 + (p1 - p0) * (1.0 - percent);

        new_points.push(q);
        new_points.push(r);
    }
    new_points.push(*points.last().unwrap());

    new_points
}