use plotter::plot_data;

fn main() {
    plot_data(&[1.0, 2.1, 3.3, 4.5], &[1.1, 2.2, 3.3, 4.4],  "Test Plot").unwrap();
}
