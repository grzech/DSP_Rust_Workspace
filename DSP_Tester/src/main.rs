use plotter::plot_data;
use dsp_lib::Generator;

fn main() {
    let mut sine = Generator::default()
        .set_phase_shift(0.11)
        .set_amplitude(11.34)
        .set_number_of_periods(3.0)
        .set_frequency(100.0)
        .set_sampling_rate(400.0);
    plot_data(sine.generate(),  "Test Plot", ("Time [s]", "Value")).unwrap();
}
