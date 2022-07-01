use fix_float::*;

#[test]
fn std() {
    assert_eq!(format!("{}", ff64!(2.42)), "fix 2.42");
    assert_eq!(format!("{:?}", ff64!(2.42)), "fix 2.42");
    assert_eq!(format!("{:}", ff64!(2.42)), "fix 2.42");
    assert_eq!(format!("{:10}", ff64!(2.42)), "fix 2.42  ");
    assert_eq!(format!("{:15}", ff64!(2.42)), "fix 2.42       ");
    assert_eq!(format!("{:0<15}", ff64!(2.42)), "fix 2.420000000");
    assert_eq!(format!("{:>15}", ff64!(2.42)), "       fix 2.42");
    assert_eq!(format!("{:0>15}", ff64!(2.42)), "0000000fix 2.42");
}
