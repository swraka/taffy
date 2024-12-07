#[test]
#[allow(non_snake_case)]
fn rounding_total_fractial__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 0.7f32,
            flex_basis: taffy::style::Dimension::Length(50.3f32),
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(20.3f32) },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1.6f32,
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            flex_grow: 1.1f32,
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10.7f32) },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(87.4f32),
                    height: taffy::style::Dimension::Length(113.4f32),
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
    assert_eq!(size.width, 87f32, "width of node {:?}. Expected {}. Actual {}", node, 87f32, size.width);
    assert_eq!(size.height, 113f32, "height of node {:?}. Expected {}. Actual {}", node, 113f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 87f32, "width of node {:?}. Expected {}. Actual {}", node0, 87f32, size.width);
    assert_eq!(size.height, 59f32, "height of node {:?}. Expected {}. Actual {}", node0, 59f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 87f32, "width of node {:?}. Expected {}. Actual {}", node1, 87f32, size.width);
    assert_eq!(size.height, 30f32, "height of node {:?}. Expected {}. Actual {}", node1, 30f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 59f32, "y of node {:?}. Expected {}. Actual {}", node1, 59f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 87f32, "width of node {:?}. Expected {}. Actual {}", node2, 87f32, size.width);
    assert_eq!(size.height, 24f32, "height of node {:?}. Expected {}. Actual {}", node2, 24f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 89f32, "y of node {:?}. Expected {}. Actual {}", node2, 89f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn rounding_total_fractial__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_grow: 0.7f32,
            flex_basis: taffy::style::Dimension::Length(50.3f32),
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(20.3f32) },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_grow: 1.6f32,
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
            ..Default::default()
        })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_grow: 1.1f32,
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10.7f32) },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(87.4f32),
                    height: taffy::style::Dimension::Length(113.4f32),
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
    assert_eq!(size.width, 87f32, "width of node {:?}. Expected {}. Actual {}", node, 87f32, size.width);
    assert_eq!(size.height, 113f32, "height of node {:?}. Expected {}. Actual {}", node, 113f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 87f32, "width of node {:?}. Expected {}. Actual {}", node0, 87f32, size.width);
    assert_eq!(size.height, 59f32, "height of node {:?}. Expected {}. Actual {}", node0, 59f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 87f32, "width of node {:?}. Expected {}. Actual {}", node1, 87f32, size.width);
    assert_eq!(size.height, 30f32, "height of node {:?}. Expected {}. Actual {}", node1, 30f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node1, 0f32, location.x);
    assert_eq!(location.y, 59f32, "y of node {:?}. Expected {}. Actual {}", node1, 59f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 87f32, "width of node {:?}. Expected {}. Actual {}", node2, 87f32, size.width);
    assert_eq!(size.height, 24f32, "height of node {:?}. Expected {}. Actual {}", node2, 24f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node2, 0f32, location.x);
    assert_eq!(location.y, 89f32, "y of node {:?}. Expected {}. Actual {}", node2, 89f32, location.y);
}
