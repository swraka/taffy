#[test]
#[allow(non_snake_case)]
fn gap_row_gap_column_child_margins__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            flex_shrink: 1f32,
            flex_basis: taffy::style::Dimension::Percent(0f32),
            margin: taffy::geometry::Rect {
                left: zero(),
                right: zero(),
                top: taffy::style::LengthPercentageAuto::Length(2f32),
                bottom: taffy::style::LengthPercentageAuto::Length(2f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            flex_shrink: 1f32,
            flex_basis: taffy::style::Dimension::Percent(0f32),
            margin: taffy::geometry::Rect {
                left: zero(),
                right: zero(),
                top: taffy::style::LengthPercentageAuto::Length(10f32),
                bottom: taffy::style::LengthPercentageAuto::Length(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1f32,
            flex_shrink: 1f32,
            flex_basis: taffy::style::Dimension::Percent(0f32),
            margin: taffy::geometry::Rect {
                left: zero(),
                right: zero(),
                top: taffy::style::LengthPercentageAuto::Length(15f32),
                bottom: taffy::style::LengthPercentageAuto::Length(15f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                gap: taffy::geometry::Size { width: zero(), height: taffy::style::LengthPercentage::Length(10f32) },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(100f32),
                    height: taffy::style::Dimension::Length(200f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node, 100f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node0, 100f32, size.width);
    assert_eq!(size.height, 42f32, "height of node {:?}. Expected {}. Actual {}", node0, 42f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 2f32, "y of node {:?}. Expected {}. Actual {}", node0, 2f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node1, 100f32, size.width);
    assert_eq!(size.height, 42f32, "height of node {:?}. Expected {}. Actual {}", node1, 42f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 66f32, "y of node {:?}. Expected {}. Actual {}", node1, 66f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node2, 100f32, size.width);
    assert_eq!(size.height, 42f32, "height of node {:?}. Expected {}. Actual {}", node2, 42f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 143f32, "y of node {:?}. Expected {}. Actual {}", node2, 143f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn gap_row_gap_column_child_margins__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_grow: 1f32,
            flex_shrink: 1f32,
            flex_basis: taffy::style::Dimension::Percent(0f32),
            margin: taffy::geometry::Rect {
                left: zero(),
                right: zero(),
                top: taffy::style::LengthPercentageAuto::Length(2f32),
                bottom: taffy::style::LengthPercentageAuto::Length(2f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_grow: 1f32,
            flex_shrink: 1f32,
            flex_basis: taffy::style::Dimension::Percent(0f32),
            margin: taffy::geometry::Rect {
                left: zero(),
                right: zero(),
                top: taffy::style::LengthPercentageAuto::Length(10f32),
                bottom: taffy::style::LengthPercentageAuto::Length(10f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_grow: 1f32,
            flex_shrink: 1f32,
            flex_basis: taffy::style::Dimension::Percent(0f32),
            margin: taffy::geometry::Rect {
                left: zero(),
                right: zero(),
                top: taffy::style::LengthPercentageAuto::Length(15f32),
                bottom: taffy::style::LengthPercentageAuto::Length(15f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                gap: taffy::geometry::Size { width: zero(), height: taffy::style::LengthPercentage::Length(10f32) },
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(100f32),
                    height: taffy::style::Dimension::Length(200f32),
                },
                ..Default::default()
            },
            &[node0, node1, node2],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node, 100f32, size.width);
    assert_eq!(size.height, 200f32, "height of node {:?}. Expected {}. Actual {}", node, 200f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node0, 100f32, size.width);
    assert_eq!(size.height, 42f32, "height of node {:?}. Expected {}. Actual {}", node0, 42f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 2f32, "y of node {:?}. Expected {}. Actual {}", node0, 2f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node1, 100f32, size.width);
    assert_eq!(size.height, 42f32, "height of node {:?}. Expected {}. Actual {}", node1, 42f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 66f32, "y of node {:?}. Expected {}. Actual {}", node1, 66f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 100f32, "width of node {:?}. Expected {}. Actual {}", node2, 100f32, size.width);
    assert_eq!(size.height, 42f32, "height of node {:?}. Expected {}. Actual {}", node2, 42f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 143f32, "y of node {:?}. Expected {}. Actual {}", node2, 143f32, location.y);
}
