#[test]
#[allow(non_snake_case)]
fn align_self_center_undefined_max_height__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(240f32),
                height: taffy::style::Dimension::Length(44f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            align_self: Some(taffy::style::AlignSelf::Center),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(40f32),
                height: taffy::style::Dimension::Length(56f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                size: taffy::geometry::Size { width: taffy::style::Dimension::Length(280f32), height: auto() },
                min_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(52f32) },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 280f32, "width of node {:?}. Expected {}. Actual {}", node, 280f32, size.width);
    assert_eq!(size.height, 56f32, "height of node {:?}. Expected {}. Actual {}", node, 56f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 240f32, "width of node {:?}. Expected {}. Actual {}", node0, 240f32, size.width);
    assert_eq!(size.height, 44f32, "height of node {:?}. Expected {}. Actual {}", node0, 44f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node1, 40f32, size.width);
    assert_eq!(size.height, 56f32, "height of node {:?}. Expected {}. Actual {}", node1, 56f32, size.height);
    assert_eq!(location.x, 240f32, "x of node {:?}. Expected {}. Actual {}", node1, 240f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn align_self_center_undefined_max_height__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(240f32),
                height: taffy::style::Dimension::Length(44f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            align_self: Some(taffy::style::AlignSelf::Center),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(40f32),
                height: taffy::style::Dimension::Length(56f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                box_sizing: taffy::style::BoxSizing::ContentBox,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Length(280f32), height: auto() },
                min_size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(52f32) },
                ..Default::default()
            },
            &[node0, node1],
        )
        .unwrap();
    taffy.compute_layout_with_measure(node, taffy::geometry::Size::MAX_CONTENT, crate::test_measure_function).unwrap();
    println!("\nComputed tree:");
    taffy.print_tree(node);
    println!();
    let layout = taffy.layout(node).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 280f32, "width of node {:?}. Expected {}. Actual {}", node, 280f32, size.width);
    assert_eq!(size.height, 56f32, "height of node {:?}. Expected {}. Actual {}", node, 56f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 240f32, "width of node {:?}. Expected {}. Actual {}", node0, 240f32, size.width);
    assert_eq!(size.height, 44f32, "height of node {:?}. Expected {}. Actual {}", node0, 44f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node1, 40f32, size.width);
    assert_eq!(size.height, 56f32, "height of node {:?}. Expected {}. Actual {}", node1, 56f32, size.height);
    assert_eq!(location.x, 240f32, "x of node {:?}. Expected {}. Actual {}", node1, 240f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node1, 0f32, location.y);
}
