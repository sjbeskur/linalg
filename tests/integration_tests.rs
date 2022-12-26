
use linalg::vector3::*;

#[test]
fn test_new_vector() { 
    let vector = Vector3::new(3.0, 4.0, 0.0);
    assert_eq!(vector.x, 3.0);
    assert_eq!(vector.y, 4.0);
    assert_eq!(vector.z, 0.0);
}    

#[test]
fn test_dot_product() { 
    let v1 = Vector3::new(3.0, 4.0, 0.0);
    let v2 = Vector3::new(3.0, 4.0, 0.0);
    println!("dot {}", v1.dot(&v2));
    assert_eq!(v1.dot(&v2), 25.0);
    assert_eq!(v2.dot(&v1), 25.0);

    let v3 = Vector3::new(-6.0, 8.0, 0.0);
    let v4 = Vector3::new(5.0, 12.0, 0.0);
    //println!("dot {:.3}", v3.dot(&v4));
    assert_eq!(v3.dot(&v4), 66.0);
    assert_eq!(v4.dot(&v3), 66.0);
}    

#[test]
fn test_length(){
    let v1 = Vector3::new(3.0, 4.0, 0.0);
    assert_eq!(v1.length(), 5.0);

    let v3 = Vector3::new(-6.0, 8.0, 0.0);
    let v4 = Vector3::new(5.0, 12.0, 0.0);
    assert_eq!(v3.length(), 10.0);
    assert_eq!(v4.length(), 13.0);

    let v5 = Vector3::new(-6.6, 8.834, 0.0);
    assert_eq!(v5.mag(), 11.027219);
}

#[test]
fn test_cross_product() {
    let v1 = Vector3::new(2.0, 3.0, 4.0);
    let v2 = Vector3::new(5.0, 6.0, 7.0);
    let v3 = v1.cross(&v2);
    assert_eq!(v3.x, -3.0);
    assert_eq!(v3.y, 6.0);
    assert_eq!(v3.z, -3.0);
}

#[test]
fn test_angle_between(){
    let v1 = Vector3::new(-4.0,4.0,0.0);
    let v2 = Vector3::new(-2.0,-5.0,4.0);

    //assert_eq!(0.0, v1.angle_between_r(&origin));
    assert_eq!(1.8925469, v1.angle_between_r(&v2));
    assert_eq!(108.43495, v1.angle_between_d(&v2));
    assert_eq!(1.8925469, v2.angle_between_r(&v1));
    assert_eq!(108.43495, v2.angle_between_d(&v1));

    let v3 = Vector3::new(50.0,50.0,0.0);
    let v4 = Vector3::new(0.0,50.0,0.0);
    assert_eq!(45.0,v4.angle_between_d(&v3));
    let v5 = Vector3::new(50.0,0.0,0.0);
    assert_eq!(90.0, v4.angle_between_d(&v5));

    let v6 = Vector3::new(0.0,-50.0,0.0);
    assert_eq!(180.0, v4.angle_between_d(&v6));
}

#[test]
fn test_equality(){
    let v6 = Vector3::new(0.0,-50.0,0.0);
    assert!(&v6 == &v6);
    assert_eq!(v6 , v6);
    let origin = Vector3::zeros();
    let zeros = Vector3::new(0.0,0.0,0.0);
    assert_eq!(&origin,&zeros);
}

#[test]
fn test_mult_scalar(){
    let input = Vector3::new(5.0,-5.0,5.0);
    let expected = Vector3::new(25.0,-25.0,25.0);
    let test = input * 5.0;
    //let test = 5.0 * input; // This no worky
    assert_eq!(test, expected);

}

