#[test]
#[allow(non_snake_case)]
fn grid_margins_auto_margins__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node1 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            justify_self: Some(taffy::style::JustifySelf::Start),
            size: taffy::geometry::Size { width: taffy::style::Dimension::Length(20f32), height: auto() },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Auto,
                right: taffy::style::LengthPercentageAuto::Auto,
                top: taffy::style::LengthPercentageAuto::Length(0f32),
                bottom: taffy::style::LengthPercentageAuto::Length(0f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style {
            align_self: Some(taffy::style::AlignSelf::Start),
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(20f32) },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(0f32),
                right: taffy::style::LengthPercentageAuto::Length(0f32),
                top: taffy::style::LengthPercentageAuto::Auto,
                bottom: taffy::style::LengthPercentageAuto::Auto,
            },
            ..Default::default()
        })
        .unwrap();
    let node5 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node6 = taffy
        .new_leaf(taffy::style::Style {
            align_self: Some(taffy::style::AlignSelf::Start),
            justify_self: Some(taffy::style::JustifySelf::Start),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(20f32),
                height: taffy::style::Dimension::Length(20f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Auto,
                right: taffy::style::LengthPercentageAuto::Auto,
                top: taffy::style::LengthPercentageAuto::Auto,
                bottom: taffy::style::LengthPercentageAuto::Auto,
            },
            ..Default::default()
        })
        .unwrap();
    let node7 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
    let node8 = taffy.new_leaf(taffy::style::Style { ..Default::default() }).unwrap();
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
            &[node0, node1, node2, node3, node4, node5, node6, node7, node8],
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
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node0, 40f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node0, 40f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node0, 10f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node1, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node1, 40f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node1, 80f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node1, 10f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node2, 20f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node2, 40f32, size.height);
    assert_eq!(location.x, 130f32, "x of node {:?}. Expected {}. Actual {}", node2, 130f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node2, 10f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node3, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node3, 40f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node3, 40f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node3, 50f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node4, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node4, 20f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node4, 80f32, location.x);
    assert_eq!(location.y, 60f32, "y of node {:?}. Expected {}. Actual {}", node4, 60f32, location.y);
    let layout = taffy.layout(node5).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node5, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node5, 40f32, size.height);
    assert_eq!(location.x, 120f32, "x of node {:?}. Expected {}. Actual {}", node5, 120f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node5, 50f32, location.y);
    let layout = taffy.layout(node6).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node6, 20f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node6, 20f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node6, 50f32, location.x);
    assert_eq!(location.y, 100f32, "y of node {:?}. Expected {}. Actual {}", node6, 100f32, location.y);
    let layout = taffy.layout(node7).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node7, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node7, 40f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node7, 80f32, location.x);
    assert_eq!(location.y, 90f32, "y of node {:?}. Expected {}. Actual {}", node7, 90f32, location.y);
    let layout = taffy.layout(node8).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node8, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node8, 40f32, size.height);
    assert_eq!(location.x, 120f32, "x of node {:?}. Expected {}. Actual {}", node8, 120f32, location.x);
    assert_eq!(location.y, 90f32, "y of node {:?}. Expected {}. Actual {}", node8, 90f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_margins_auto_margins__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node0 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node1 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node2 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            justify_self: Some(taffy::style::JustifySelf::Start),
            size: taffy::geometry::Size { width: taffy::style::Dimension::Length(20f32), height: auto() },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Auto,
                right: taffy::style::LengthPercentageAuto::Auto,
                top: taffy::style::LengthPercentageAuto::Length(0f32),
                bottom: taffy::style::LengthPercentageAuto::Length(0f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node3 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node4 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            align_self: Some(taffy::style::AlignSelf::Start),
            size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(20f32) },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Length(0f32),
                right: taffy::style::LengthPercentageAuto::Length(0f32),
                top: taffy::style::LengthPercentageAuto::Auto,
                bottom: taffy::style::LengthPercentageAuto::Auto,
            },
            ..Default::default()
        })
        .unwrap();
    let node5 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node6 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            align_self: Some(taffy::style::AlignSelf::Start),
            justify_self: Some(taffy::style::JustifySelf::Start),
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Length(20f32),
                height: taffy::style::Dimension::Length(20f32),
            },
            margin: taffy::geometry::Rect {
                left: taffy::style::LengthPercentageAuto::Auto,
                right: taffy::style::LengthPercentageAuto::Auto,
                top: taffy::style::LengthPercentageAuto::Auto,
                bottom: taffy::style::LengthPercentageAuto::Auto,
            },
            ..Default::default()
        })
        .unwrap();
    let node7 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
        .unwrap();
    let node8 = taffy
        .new_leaf(taffy::style::Style { box_sizing: taffy::style::BoxSizing::ContentBox, ..Default::default() })
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
            &[node0, node1, node2, node3, node4, node5, node6, node7, node8],
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
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node0, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node0, 40f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node0, 40f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node0, 10f32, location.y);
    let layout = taffy.layout(node1).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node1, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node1, 40f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node1, 80f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node1, 10f32, location.y);
    let layout = taffy.layout(node2).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node2, 20f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node2, 40f32, size.height);
    assert_eq!(location.x, 130f32, "x of node {:?}. Expected {}. Actual {}", node2, 130f32, location.x);
    assert_eq!(location.y, 10f32, "y of node {:?}. Expected {}. Actual {}", node2, 10f32, location.y);
    let layout = taffy.layout(node3).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node3, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node3, 40f32, size.height);
    assert_eq!(location.x, 40f32, "x of node {:?}. Expected {}. Actual {}", node3, 40f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node3, 50f32, location.y);
    let layout = taffy.layout(node4).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node4, 40f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node4, 20f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node4, 80f32, location.x);
    assert_eq!(location.y, 60f32, "y of node {:?}. Expected {}. Actual {}", node4, 60f32, location.y);
    let layout = taffy.layout(node5).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node5, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node5, 40f32, size.height);
    assert_eq!(location.x, 120f32, "x of node {:?}. Expected {}. Actual {}", node5, 120f32, location.x);
    assert_eq!(location.y, 50f32, "y of node {:?}. Expected {}. Actual {}", node5, 50f32, location.y);
    let layout = taffy.layout(node6).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 20f32, "width of node {:?}. Expected {}. Actual {}", node6, 20f32, size.width);
    assert_eq!(size.height, 20f32, "height of node {:?}. Expected {}. Actual {}", node6, 20f32, size.height);
    assert_eq!(location.x, 50f32, "x of node {:?}. Expected {}. Actual {}", node6, 50f32, location.x);
    assert_eq!(location.y, 100f32, "y of node {:?}. Expected {}. Actual {}", node6, 100f32, location.y);
    let layout = taffy.layout(node7).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node7, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node7, 40f32, size.height);
    assert_eq!(location.x, 80f32, "x of node {:?}. Expected {}. Actual {}", node7, 80f32, location.x);
    assert_eq!(location.y, 90f32, "y of node {:?}. Expected {}. Actual {}", node7, 90f32, location.y);
    let layout = taffy.layout(node8).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 40f32, "width of node {:?}. Expected {}. Actual {}", node8, 40f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node8, 40f32, size.height);
    assert_eq!(location.x, 120f32, "x of node {:?}. Expected {}. Actual {}", node8, 120f32, location.x);
    assert_eq!(location.y, 90f32, "y of node {:?}. Expected {}. Actual {}", node8, 90f32, location.y);
}
