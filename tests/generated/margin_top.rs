#[test]
fn margin_top() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size { height: stretch::style::Dimension::Points(10f32), ..Default::default() },
            margin: stretch::geometry::Rect { top: stretch::style::Dimension::Points(10f32), ..Default::default() },
            ..Default::default()
        },
        vec![],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node0],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 100f32);
    assert_eq!(stretch.layout(node).size.height, 100f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 100f32);
    assert_eq!(stretch.layout(node0).size.height, 10f32);
    assert_eq!(stretch.layout(node0).location.x, 0f32);
    assert_eq!(stretch.layout(node0).location.y, 10f32);
}
