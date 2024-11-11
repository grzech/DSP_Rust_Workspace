use core::f64;

use plotters::prelude::*;

const RESOLUTION : (u32, u32) = (2048, 1280);
const WIDTH : u32 = 4;
const P_SIZE: u32 = 6;

pub fn plot_data(data: &[(f64, f64)], title: &str, (x_axis, y_axis): (&str, &str)) -> Result<(), Box<dyn std::error::Error>> {
    let x_rng = (data[0].0, data.last().unwrap().0);
    let y_rng = get_min_max(&data.iter().map(|(_, d)| *d).collect::<Vec<f64>>());
    let filename = &format!("{}.png", title.split(" ").collect::<Vec<&str>>().join("_"));
    let backend = BitMapBackend::new(filename, RESOLUTION).into_drawing_area();
    backend.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&backend)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(15)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_rng.0..x_rng.1, y_rng.0..y_rng.1)?;
    chart
        .configure_mesh()
        .axis_desc_style(("sans-serif", 24))
        .x_desc(x_axis)
        .y_desc(y_axis)
        .draw()?;
    chart.draw_series(LineSeries::new(
            data.into_iter().map(|(x, y)| (*x, *y)),
            MAGENTA.stroke_width(WIDTH).filled(),
        ).point_size(P_SIZE))?
        .label(title)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

fn get_min_max(data: &[f64]) -> (f64, f64) {
    let mut min = f64::MAX;
    let mut max = f64::MIN;
    for x in data {
        if *x > max {
            max = *x;
        }
        if *x < min {
            min = *x;
        }
    }
    (min, max)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_min_max_shall_return_minimum_and_maximum_value_of_slice() {
        assert_eq!(get_min_max(&[0.0, 1.1, 2.2, 3.3]), (0.0, 3.3));
        assert_eq!(get_min_max(&[343525.5, -431421.32, 43.45, 44234.2352, 235232234.3, 0.1241241]),
            (-431421.32, 235232234.3));
    }
}
