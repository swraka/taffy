#[test]
#[allow(non_snake_case)]
fn align_content_space_evenly_single_line_negative_space__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(0.8f32),
                height: taffy::style::Dimension::Length(60f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                align_content: Some(taffy::style::AlignContent::SpaceEvenly),
                justify_content: Some(taffy::style::JustifyContent::Center),
                size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(320f32),
                    height: taffy::style::Dimension::Length(320f32),
                },
                border: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(60f32),
                    right: taffy::style::LengthPercentage::Length(60f32),
                    top: taffy::style::LengthPercentage::Length(60f32),
                    bottom: taffy::style::LengthPercentage::Length(60f32),
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
    assert_eq!(size.width, 320f32, "width of node {:?}. Expected {}. Actual {}", node, 320f32, size.width);
    assert_eq!(size.height, 320f32, "height of node {:?}. Expected {}. Actual {}", node, 320f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node0, 200f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0, 10f32, size.height);
    assert_eq!(location.x, 60f32, "x of node {:?}. Expected {}. Actual {}", node0, 60f32, location.x);
    assert_eq!(location.y, 60f32, "y of node {:?}. Expected {}. Actual {}", node0, 60f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 160f32, "width of node {:?}. Expected {}. Actual {}", node00, 160f32, size.width);
    assert_eq!(size.height, 60f32, "height of node {:?}. Expected {}. Actual {}", node00, 60f32, size.height);
    assert_eq!(location.x, 20f32, "x of node {:?}. Expected {}. Actual {}", node00, 20f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn align_content_space_evenly_single_line_negative_space__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            box_sizing: taffy::style::BoxSizing::ContentBox,
            flex_shrink: 0f32,
            size: taffy::geometry::Size {
                width: taffy::style::Dimension::Percent(0.8f32),
                height: taffy::style::Dimension::Length(60f32),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                align_content: Some(taffy::style::AlignContent::SpaceEvenly),
                justify_content: Some(taffy::style::JustifyContent::Center),
                size: taffy::geometry::Size { width: auto(), height: taffy::style::Dimension::Length(10f32) },
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Flex,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                flex_direction: taffy::style::FlexDirection::Column,
                size: taffy::geometry::Size {
                    width: taffy::style::Dimension::Length(320f32),
                    height: taffy::style::Dimension::Length(320f32),
                },
                border: taffy::geometry::Rect {
                    left: taffy::style::LengthPercentage::Length(60f32),
                    right: taffy::style::LengthPercentage::Length(60f32),
                    top: taffy::style::LengthPercentage::Length(60f32),
                    bottom: taffy::style::LengthPercentage::Length(60f32),
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
    assert_eq!(size.width, 440f32, "width of node {:?}. Expected {}. Actual {}", node, 440f32, size.width);
    assert_eq!(size.height, 440f32, "height of node {:?}. Expected {}. Actual {}", node, 440f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 320f32, "width of node {:?}. Expected {}. Actual {}", node0, 320f32, size.width);
    assert_eq!(size.height, 10f32, "height of node {:?}. Expected {}. Actual {}", node0, 10f32, size.height);
    assert_eq!(location.x, 60f32, "x of node {:?}. Expected {}. Actual {}", node0, 60f32, location.x);
    assert_eq!(location.y, 60f32, "y of node {:?}. Expected {}. Actual {}", node0, 60f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 256f32, "width of node {:?}. Expected {}. Actual {}", node00, 256f32, size.width);
    assert_eq!(size.height, 60f32, "height of node {:?}. Expected {}. Actual {}", node00, 60f32, size.height);
    assert_eq!(location.x, 32f32, "x of node {:?}. Expected {}. Actual {}", node00, 32f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
}
