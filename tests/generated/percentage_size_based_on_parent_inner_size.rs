#[test]
fn percentage_size_based_on_parent_inner_size() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Percent(0.5f32),
                height: stretch::style::Dimension::Percent(0.5f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            flex_direction: stretch::style::FlexDirection::Column,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200f32),
                height: stretch::style::Dimension::Points(400f32),
                ..Default::default()
            },
            padding: stretch::geometry::Rect {
                start: stretch::style::Dimension::Points(20f32),
                end: stretch::style::Dimension::Points(20f32),
                top: stretch::style::Dimension::Points(20f32),
                bottom: stretch::style::Dimension::Points(20f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node0],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 200f32);
    assert_eq!(stretch.layout(node).size.height, 400f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 80f32);
    assert_eq!(stretch.layout(node0).size.height, 180f32);
    assert_eq!(stretch.layout(node0).location.x, 20f32);
    assert_eq!(stretch.layout(node0).location.y, 20f32);
}
