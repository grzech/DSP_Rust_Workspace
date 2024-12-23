use plotter::plot_data;
use dsp_lib::{fft, DescreteSignal, Generator};

fn main() {
    let num_of_periods = 7.0;
    let sampling_rate = 100.0;
    
    let mut sig = Generator::default()
    //.set_phase_shift(0.5*PI)
        .set_amplitude(1.0)
        .set_number_of_periods(num_of_periods)
        .set_frequency(1.0)
        .set_sampling_rate(sampling_rate)
        .set_offset(0.5)
        .generate();

    for i in (3..26).step_by(2) {
        let sine = Generator::default()
            .set_amplitude(1.0/i as f64)
            .set_number_of_periods(num_of_periods * i as f64)
            .set_frequency(i as f64)
            .set_sampling_rate(sampling_rate)
            .generate();
        sig = &sig + &sine;
        plot_data(sig.get_data(), &format!("{}_harmonics", i/2 + 1), ("Signal value", "time [s]")).unwrap();
        let mut fourier = DescreteSignal::new();
        fft(&sig, &mut fourier);
        plot_data(fourier.get_data(), &format!("Fast_Fourier_Transform_for_{}_Harmonics", i/2+1), ("Frequency [Hz]", "Amplitude")).unwrap();
      }

}
