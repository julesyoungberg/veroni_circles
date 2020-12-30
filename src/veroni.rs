use nannou::prelude::*;

pub static GRID_RES: usize = 8;

// classic GLSL-style random vector generator
fn rand2(c: Vector2<f64>) -> Vector2<f64> {
    let x = c.dot(pt2(127.1, 311.7)).sin();
    let y = c.dot(pt2(269.5, 183.3)).sin();
    let p = pt2(x, y) * 43758.5453 % 1.0;
    return pt2(p.x.abs(), p.y.abs());
}

// generate a grid of random points
pub fn get_points() -> Vec<Vec<Vector2<f64>>> {
    let mut points = vec![vec![pt2(0.0 as f64, 0.0 as f64); GRID_RES]; GRID_RES];

    for y in 0..GRID_RES {
        for x in 0..GRID_RES {
            let bin_coord = pt2(x as f64, y as f64);
            let point = bin_coord + rand2(bin_coord);
            points[x][y] = point;
        }
    }

    return points;
}

// compute the maximum radius for each point such that the circle is contained in the cell
pub fn get_radiuses(points: &Vec<Vec<Vector2<f64>>>) -> Vec<Vec<f64>> {
    let mut radiuses = vec![vec![0.0 as f64; GRID_RES]; GRID_RES];

    for y in 0..GRID_RES {
        for x in 0..GRID_RES {
            let center_point = points[x][y];
            let mut radius: f64 = 0.0;

            // search neighbors for nearest
            for yoff in 0..2 {
                let yoffset = yoff - 1;

                for xoff in 0..2 {
                    let xoffset = xoff - 1;

                    if yoffset == 0 && xoffset == 0 {
                        continue;
                    }

                    let nx = (x as i32 + xoffset) % GRID_RES as i32;
                    let ny = (y as i32 + yoffset) % GRID_RES as i32;
                    let neighbor = points[nx.abs() as usize][ny.abs() as usize];

                    let diff = center_point - neighbor;
                    let hyp = diff.x * diff.x + diff.y * diff.y;
                    let dist = hyp.sqrt();

                    if radius == 0.0 || dist < radius {
                        radius = dist;
                    }
                }
            }

            radiuses[x as usize][y as usize] = radius;
        }
    }

    return radiuses;
}