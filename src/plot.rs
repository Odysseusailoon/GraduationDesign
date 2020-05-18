use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{LineStyle, PointMarker, PointStyle};
use plotlib::view::ContinuousView;

pub fn new_plot(original_data: Vec<u32>, colour: &str) -> Plot {
    let data1 = original_data
        .iter()
        .enumerate()
        .map(|(index, datum)| (*datum as f64, index as f64))
        .collect();

    // We create our scatter plot from the data
    let s1: Plot = Plot::new(data1)
        .line_style(LineStyle::new().colour(colour).width(2.0))
        .point_style(
            PointStyle::new()
                .marker(PointMarker::Square) // setting the marker to be a square
                .colour(colour),
        ); // and a custom colour

    s1
}

//plot the val of objective function  related to  T-slot in different E
pub unsafe fn plot_two_lines(vec1: Vec<u32>, vec2: Vec<u32>, colour1: &str, colour2: &str) {
    let s1 = new_plot(vec1, colour1);
    let s2 = new_plot(vec2, colour2);
    // The 'view' describes what set of data is drawn
    let v = ContinuousView::new()
        .add(s1)
        .add(s2)
        .x_range(0., 500.)
        .y_range(0., 500.)
        .x_label("Some varying variable")
        .y_label("The response of something");

    // A page with a single view is then saved to an SVG file
    Page::single(&v).save("scatter.svg").unwrap();
}

//plot the q_t backlogs  related to T-slot

//plot the latency_bond related to T-slot in different V
pub unsafe fn plot_four_lines(
    vec1: Vec<u32>,
    vec2: Vec<u32>,
    vec3: Vec<u32>,
    vec4: Vec<u32>,
    colour1: &str,
    colour2: &str,
    colour3: &str,
    colour4: &str,
) {
    let s1 = new_plot(vec1, colour1);
    let s2 = new_plot(vec2, colour2);
    let s3 = new_plot(vec3, colour3);
    let s4 = new_plot(vec4, colour4);

    // The 'view' describes what set of data is drawn
    let v = ContinuousView::new()
        .add(s1)
        .add(s2)
        .add(s3)
        .add(s4)
        .x_range(0., 500.)
        .y_range(0., 500.)
        .x_label("Some varying variable")
        .y_label("The response of something");

    // A page with a single view is then saved to an SVG file
    Page::single(&v).save("scatter.svg").unwrap();
}

//plot the latency_bond
