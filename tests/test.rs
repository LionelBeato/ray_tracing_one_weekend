use lib::write_color;
use lib::hit_sphere;
use lib::vec3::Vector3;
use lib::ray::Ray;

#[test]
fn test_vec3() {
    let vec = Vector3::new(12.0, 12.0, 12.0);
    assert_eq!(vec.magnitude(), 20.784609690826528);
}

#[test]
fn write_color_test() {
    let vec = Vector3::new(12.0, 12.0, 12.0);
    write_color(vec, 100);
}

#[test]
fn hit_sphere_test() {
    let center = Vector3::new(12.0, 12.0, 12.0);
    let radius = 10f64;
    let r = 
    Ray::new(Vector3::new(12.0, 12.0, 12.0), Vector3::new(14.0, 10.0, 2.0));
    
    assert_eq!(hit_sphere(&center, radius, &r), -0.5773502691896257f64);
}