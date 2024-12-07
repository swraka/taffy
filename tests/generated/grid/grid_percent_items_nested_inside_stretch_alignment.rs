#[test]
#[allow(non_snake_case)]
fn grid_percent_items_nested_inside_stretch_alignment__border_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Grid,
            padding: taffy::geometry::Rect {
                left: zero(),
                right: zero(),
                top: taffy::style::LengthPercentage::Percent(0.2f32),
                bottom: zero(),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style { display: taffy::style::Display::Grid, ..Default::default() },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Length(200f32), height: auto() },
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
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node, 200f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node, 40f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node0, 200f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node0, 40f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node00, 200f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node00, 40f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
}

#[test]
#[allow(non_snake_case)]
fn grid_percent_items_nested_inside_stretch_alignment__content_box() {
    #[allow(unused_imports)]
    use taffy::{prelude::*, Layout};
    let mut taffy = crate::new_test_tree();
    let node00 = taffy
        .new_leaf(taffy::style::Style {
            display: taffy::style::Display::Grid,
            box_sizing: taffy::style::BoxSizing::ContentBox,
            padding: taffy::geometry::Rect {
                left: zero(),
                right: zero(),
                top: taffy::style::LengthPercentage::Percent(0.2f32),
                bottom: zero(),
            },
            ..Default::default()
        })
        .unwrap();
    let node0 = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                ..Default::default()
            },
            &[node00],
        )
        .unwrap();
    let node = taffy
        .new_with_children(
            taffy::style::Style {
                display: taffy::style::Display::Grid,
                box_sizing: taffy::style::BoxSizing::ContentBox,
                size: taffy::geometry::Size { width: taffy::style::Dimension::Length(200f32), height: auto() },
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
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node, 200f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node, 40f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node, 0f32, location.y);
    let layout = taffy.layout(node0).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node0, 200f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node0, 40f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node0, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node0, 0f32, location.y);
    let layout = taffy.layout(node00).unwrap();
    let Layout { size, location, .. } = layout;
    assert_eq!(size.width, 200f32, "width of node {:?}. Expected {}. Actual {}", node00, 200f32, size.width);
    assert_eq!(size.height, 40f32, "height of node {:?}. Expected {}. Actual {}", node00, 40f32, size.height);
    assert_eq!(location.x, 0f32, "x of node {:?}. Expected {}. Actual {}", node00, 0f32, location.x);
    assert_eq!(location.y, 0f32, "y of node {:?}. Expected {}. Actual {}", node00, 0f32, location.y);
}
