use plotter::plot_data;
use DspLib::generators;

fn main() {
    let sine = generators::generate_sine(3.0, 10.0, 5.0, 200.0);
    plot_data(&sine,  "Test Plot", ("Time [s]", "Value")).unwrap();
}
