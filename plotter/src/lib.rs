use core::f32;

use plotters::prelude::*;

const RESOLUTION : (u32, u32) = (2048, 1280);
const WIDTH : u32 = 4;
const P_SIZE: u32 = 6;

pub fn plot_data(x: &[f32], y: &[f32], title: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filename = &format!("{}.png", title.split(" ").collect::<Vec<&str>>().join("_"));
    let backend = BitMapBackend::new(filename, RESOLUTION).into_drawing_area();
    backend.fill(&WHITE)?;
    let (min, max) = get_min_max(y);
    let mut chart = ChartBuilder::on(&backend)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(15)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x[0]..*x.last().unwrap(), min..max)?;
    chart.configure_mesh().draw()?;
    let data = x.iter().zip(y.iter()).map(|(&x, &y)| (x, y));
    chart.draw_series(LineSeries::new(
            data,
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

fn get_min_max(data: &[f32]) -> (f32, f32) {
    let mut min = f32::MAX;
    let mut max = f32::MIN;
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
