mod areas_volumes;
use crate::areas_volumes::{
    triangle_area, square_area, rectangle_area, circle_area,
    triangular_pyramid_volume, sphere_volume, parallelepiped_volume, cube_volume, cone_volume,
};
pub use crate::areas_volumes::{GeometricalShapes, GeometricalVolumes};

pub fn area_fit(x: usize, y: usize, objects: areas_volumes::GeometricalShapes, times: usize, a: usize, b: usize) -> bool {
    let content: f64; match objects {
        GeometricalShapes::Circle => {content = circle_area(a)}
        GeometricalShapes::Rectangle => {content = rectangle_area(a, b) as f64}
        GeometricalShapes::Square => {content = square_area(a) as f64}
        GeometricalShapes::Triangle => {content = triangle_area(a, b)}
    }
    areas_volumes::rectangle_area(x, y) as f64 >= content * times as f64 
}

pub fn volume_fit(x: usize, y: usize, z: usize, objects: areas_volumes::GeometricalVolumes, times: usize, a: usize, b: usize, c: usize) -> bool {
    let content: f64; match objects {
        GeometricalVolumes::Cube => {content = cube_volume(a) as f64}
        GeometricalVolumes::Cone => {content = cone_volume(a, b)}
        GeometricalVolumes::Parallelepiped => {content = parallelepiped_volume(a, b, c) as f64}
        GeometricalVolumes::Pyramid => {content = triangular_pyramid_volume(a as f64, b)}
        GeometricalVolumes::Sphere => {content = sphere_volume(a)}
    }
    areas_volumes::parallelepiped_volume(x, y, z) as f64 >= content * times as f64
}
