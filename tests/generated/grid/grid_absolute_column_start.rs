#[test]
#[allow(non_snake_case)]
fn grid_absolute_column_start__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            position: taffy::style::Position::Absolute,
            grid_column: taffy::geometry::Line { start: line(1i16), end: taffy::style::GridPlacement::Auto },
            inset: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(4f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(1f32),
                bottom: taffy::style::LengthPercentageAuto::Length(2f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                grid_template_rows: vec![length(40f32), length(40f32), length(40f32)],
                grid_template_columns: vec![length(40f32), length(40f32), length(40f32)],
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(40f32),
                    right: taffy::style::LengthPercentage::Length(20f32),
                    top: taffy::style::LengthPercentage::Length(10f32),
                    bottom: taffy::style::LengthPercentage::Length(30f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 180f32, "width of node {:?}. Expected {}. Actual {}", node, 180f32, size.width);
    assert_eq!(size.height, 160f32, "height of node {:?}. Expected {}. Actual {}", node, 160f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 133f32, "width of node {:?}. Expected {}. Actual {}", node0, 133f32, size.width);
    assert_eq!(size.height, 157f32, "height of node {:?}. Expected {}. Actual {}", node0, 157f32, size.height);
    assert_eq!(location.x, 44f32, "x of node {:?}. Expected {}. Actual {}", node0, 44f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node0, 1f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_absolute_column_start__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            position: taffy::style::Position::Absolute,
            grid_column: taffy::geometry::Line { start: line(1i16), end: taffy::style::GridPlacement::Auto },
            inset: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(4f32),
                right: taffy::style::LengthPercentageAuto::Length(3f32),
                top: taffy::style::LengthPercentageAuto::Length(1f32),
                bottom: taffy::style::LengthPercentageAuto::Length(2f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                grid_template_rows: vec![length(40f32), length(40f32), length(40f32)],
                grid_template_columns: vec![length(40f32), length(40f32), length(40f32)],
                padding: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(40f32),
                    right: taffy::style::LengthPercentage::Length(20f32),
                    top: taffy::style::LengthPercentage::Length(10f32),
                    bottom: taffy::style::LengthPercentage::Length(30f32),
                },
                ..Default::default()
            },
            &[node0],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 180f32, "width of node {:?}. Expected {}. Actual {}", node, 180f32, size.width);
    assert_eq!(size.height, 160f32, "height of node {:?}. Expected {}. Actual {}", node, 160f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 133f32, "width of node {:?}. Expected {}. Actual {}", node0, 133f32, size.width);
    assert_eq!(size.height, 157f32, "height of node {:?}. Expected {}. Actual {}", node0, 157f32, size.height);
    assert_eq!(location.x, 44f32, "x of node {:?}. Expected {}. Actual {}", node0, 44f32, location.x);
    assert_eq!(location.y, 1f32, "y of node {:?}. Expected {}. Actual {}", node0, 1f32, location.y);
}
