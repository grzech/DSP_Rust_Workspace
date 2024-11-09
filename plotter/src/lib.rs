use plotters::prelude::*;

const RESOLUTION : (u32, u32) = (2048, 1280);

pub fn plot_data(x: &[f32], y: &[f32], title: &str, max: f32) -> Result<(), Box<dyn std::error::Error>> {
    let filename = &format!("{}.png", title.split(" ").collect::<Vec<&str>>()[0]);
    let backend = BitMapBackend::new(filename, RESOLUTION).into_drawing_area();
    backend.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&backend)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(15)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..*x.last().unwrap(), 0f32..max)?;
    chart.configure_mesh().draw()?;
    let data = x.iter().zip(y.iter()).map(|(&x, &y)| (x, y));
    chart.draw_series(LineSeries::new(
            data,
            &MAGENTA,
        ))?
        .label(title)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

}
