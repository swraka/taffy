#[test]
fn padding_flex_child() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(
        stretch::style::Style {
            flex_grow: 1f32,
            size: stretch::geometry::Size { width: stretch::style::Dimension::Points(10f32), ..Default::default() },
            ..Default::default()
        },
        vec![],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100f32),
                height: stretch::style::Dimension::Points(100f32),
                ..Default::default()
            },
            padding: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(10f32),
                end: stretch::style::Dimension::Points(10f32),
                top: stretch::style::Dimension::Points(10f32),
                bottom: stretch::style::Dimension::Points(10f32),
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
    assert_eq!(stretch.layout(node0).size.width, 80f32);
    assert_eq!(stretch.layout(node0).size.height, 80f32);
    assert_eq!(stretch.layout(node0).location.x, 10f32);
    assert_eq!(stretch.layout(node0).location.y, 10f32);
}
