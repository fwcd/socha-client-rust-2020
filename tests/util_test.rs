use std::fmt::Debug;
use socha_client_2020::game::{DoubledCoords as Doubled, AxialCoords as Axial};

/// Tests whether a bidirectional conversion
/// succeeds in both directions.
fn test_bi_conversion<A, B>(a: A, b: B) where A: From<B> + Eq + Debug + Clone, B: From<A> + Eq + Debug + Clone {
    assert_eq!(B::from(a.clone()), b.clone());
    assert_eq!(A::from(b), a);
}

#[test]
fn doubled_axial_coords() {
    test_bi_conversion(Axial::new(0, 1), Doubled::new(-1, -1));
    test_bi_conversion(Axial::new(1, 0), Doubled::new(1, -1));
    test_bi_conversion(Axial::new(-1, 1), Doubled::new(-2, 0));
    test_bi_conversion(Axial::new(0, 0), Doubled::new(0, 0));
    test_bi_conversion(Axial::new(1, -1), Doubled::new(2, 0));
    test_bi_conversion(Axial::new(-2, 1), Doubled::new(-3, 1));
    test_bi_conversion(Axial::new(-1, 0), Doubled::new(-1, 1));
    test_bi_conversion(Axial::new(0, -1), Doubled::new(1, 1));
}
