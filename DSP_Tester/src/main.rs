use plotter::plot_data;
use DspLib::Generator;

fn main() {
    let mut sine = Generator::default()
        .set_phase_shift(0.34)
        .set_amplitude(11.34)
        .set_sampling_rate(19.3);
    plot_data(sine.generate(),  "Test Plot", ("Time [s]", "Value")).unwrap();
}
