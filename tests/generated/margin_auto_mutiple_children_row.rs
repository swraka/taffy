#[test]
fn margin_auto_mutiple_children_row() {
    let mut stretch = stretch::Stretch::new();
    let node0 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(50f32),
                height: stretch::style::Dimension::Points(50f32),
                ..Default::default()
            },
            margin: stretch::geometry::Rect { end: stretch::style::Dimension::Auto, ..Default::default() },
            ..Default::default()
        },
        vec![],
    );
    let node1 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(50f32),
                height: stretch::style::Dimension::Points(50f32),
                ..Default::default()
            },
            margin: stretch::geometry::Rect { end: stretch::style::Dimension::Auto, ..Default::default() },
            ..Default::default()
        },
        vec![],
    );
    let node2 = stretch.new_node(
        stretch::style::Style {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(50f32),
                height: stretch::style::Dimension::Points(50f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![],
    );
    let node = stretch.new_node(
        stretch::style::Style {
            align_items: stretch::style::AlignItems::Center,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200f32),
                height: stretch::style::Dimension::Points(200f32),
                ..Default::default()
            },
            ..Default::default()
        },
        vec![node0, node1, node2],
    );
    stretch.compute_layout(node, stretch::geometry::Size::undefined()).unwrap();
    assert_eq!(stretch.layout(node).size.width, 200f32);
    assert_eq!(stretch.layout(node).size.height, 200f32);
    assert_eq!(stretch.layout(node).location.x, 0f32);
    assert_eq!(stretch.layout(node).location.y, 0f32);
    assert_eq!(stretch.layout(node0).size.width, 50f32);
    assert_eq!(stretch.layout(node0).size.height, 50f32);
    assert_eq!(stretch.layout(node0).location.x, 0f32);
    assert_eq!(stretch.layout(node0).location.y, 75f32);
    assert_eq!(stretch.layout(node1).size.width, 50f32);
    assert_eq!(stretch.layout(node1).size.height, 50f32);
    assert_eq!(stretch.layout(node1).location.x, 75f32);
    assert_eq!(stretch.layout(node1).location.y, 75f32);
    assert_eq!(stretch.layout(node2).size.width, 50f32);
    assert_eq!(stretch.layout(node2).size.height, 50f32);
    assert_eq!(stretch.layout(node2).location.x, 150f32);
    assert_eq!(stretch.layout(node2).location.y, 75f32);
}
