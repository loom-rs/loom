/// Imports
use crate::assert_sema;

// Note: should bail
#[test]
fn test_sema_1() {
    assert_sema!(
        r#"
        break;
        "#
    )
}

// Note: should bail
#[test]
fn test_sema_2() {
    assert_sema!(
        r#"
        continue;
        "#
    )
}

// Note: should bail
#[test]
fn test_sema_3() {
    assert_sema!(
        r#"
        return;
        "#
    )
}

// Note: should bail
#[test]
fn test_sema_4() {
    assert_sema!(
        r#"
        fn outer() {
            while true {
                fn inner() {
                    break;
                }
            }
        }
        "#
    )
}

// Note: should bail
#[test]
fn test_sema_5() {
    assert_sema!(
        r#"
        fn outer() {
            while true {
                fn inner() {
                    continue;
                }
            }
        }
        "#
    )
}
