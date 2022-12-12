use rt_rs::*;

#[test]
fn add() {
    let x = Vec3::<f64>{ x: 2., y: 3., z: 4.};
    let y = Vec3::<f64>{ x: 4., y: 5., z: 6.};

    let res = x + y;

    assert!(res.x - 6. < 1e-8);
    assert!(res.y - 8. < 1e-8);
    assert!(res.z - 10. < 1e-8);
}

#[test]
fn sub() {
    let x = Vec3::<f64>{ x: 2., y: 3., z: 4.};
    let y = Vec3::<f64>{ x: 4., y: 5., z: 6.};

    let res = x - y;

    assert!(res.x + 2. < 1e-8);
    assert!(res.y + 2. < 1e-8);
    assert!(res.z + 2. < 1e-8);
}


#[test]
fn mul() {
    let x = Vec3::<f64>{ x: 2., y: 3., z: 4.};
    let y = 4f64;

    let res = x * y;

    assert!(res.x - 8. < 1e-8);
    assert!(res.y - 12. < 1e-8);
    assert!(res.z - 16. < 1e-8);
}

#[test]
fn div() {
    let x = Vec3::<f64>{ x: 2., y: 3., z: 4.};
    let y = 4f64;

    let res = x / y;

    assert!(res.x - 0.5 < 1e-8);
    assert!(res.y - 0.75 < 1e-8);
    assert!(res.z - 1.0 < 1e-8);
}

#[test]
fn neg() {
    let x = Vec3::<f64>{ x: 2., y: 3., z: 4.};
    let res = -x;

    assert!(res.x + 2. < 1e-8);
    assert!(res.y + 3. < 1e-8);
    assert!(res.z + 4. < 1e-8);
}
