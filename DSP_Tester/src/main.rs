use plotter::plot_data;
use dsp_lib::{Generator};

fn main() {
    let mut sig = Generator::default()
    //.set_phase_shift(0.5*PI)
        .set_amplitude(1.0)
        .set_number_of_periods(3.0)
        .set_frequency(1.0)
        .set_sampling_rate(20.0)
        .generate();
    
    for i in (3..21).step_by(2) {
        let sine = Generator::default()
            .set_amplitude(1.0/i as f64)
            .set_number_of_periods(3.0 * i as f64)
            .set_frequency(1.0 * i as f64)
            .set_sampling_rate(20.0 * i as f64)
            .generate();
        sig = &sig + &sine;
        plot_data(sig.get_data(), &format!("{}_harmonics", i/2 + 1), ("Signal value", "time [s]")).unwrap();
    }
}
