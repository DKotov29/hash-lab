use charts::{Chart, ScaleBand, ScaleLinear, VerticalBarView};

pub fn chhhart(vec: &Vec<(String, f32)>, file_name: &str) {
    let width = 2000;
    let height = 1000;
    let (top, right, bottom, left) = (70, 10, 50, 60);
    let x = ScaleBand::new()
        .set_domain(vec.iter().map(|d| d.0.clone()).collect())
        .set_range(vec![0, width - left - right]);

    let y = ScaleLinear::new()
        .set_domain(vec![0_f32, vec.iter().map(|d| d.1 as usize ).max().unwrap() as f32])
        .set_range(vec![height - top - bottom, 0]);

    let view = VerticalBarView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .load_data(vec).unwrap();

    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_view(&view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("Iterations")
        .save(file_name).unwrap();
}