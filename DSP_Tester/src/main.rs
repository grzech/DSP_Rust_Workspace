use plotter::plot_data;
use dsp_lib::{fft, DescreteSignal, Generator};

fn main() {
    let num_of_periods = 7.0;
    let sampling_rate = 100.0;
    
    let mut sig = Generator::sine_wave(1.0)
        .set_amplitude(1.0)
        .set_number_of_periods(num_of_periods)
        .set_sampling_rate(sampling_rate)
        .set_offset(0.0)
        .generate();

    for i in (3..26).step_by(2) {
        let sine = Generator::sine_wave(i as f64)
            .set_amplitude(1.0/i as f64)
            .set_number_of_periods(num_of_periods * i as f64)
            .set_sampling_rate(sampling_rate)
            .generate();
        sig = &sig + &sine;
      }
    plot_data(sig.get_data(), &format!("Sum of Sine Waves"), ("Signal value", "time [s]")).unwrap();
    let mut fourier = DescreteSignal::new();
    fft(&sig, &mut fourier);
    plot_data(fourier.get_data(), &format!("Fast_Fourier_Transform_for_Sum_of_harmonics"), ("Frequency [Hz]", "Amplitude")).unwrap();
    let sig = Generator::rectangle_wave(1.0, 0.5)
        .set_amplitude(0.78)
        .set_sampling_rate(100.0)
        .set_offset(0.3)
        .set_phase_shift(0.5)
        .set_number_of_periods(5.0)
        .set_duty_cycle(0.1)
        .generate();
    plot_data(sig.get_data(), &format!("Rectangle Wave"), ("Signal value", "time [s]")).unwrap();
    fourier.clear();
    fft(&sig, &mut fourier);
    plot_data(fourier.get_data(), &format!("Fast_Fourier_Transform_for_Rectangle_wave"), ("Frequency [Hz]", "Amplitude")).unwrap();

    let sig = Generator::triangle_wave(1.0)
        .set_amplitude(30.0)
        .set_sampling_rate(100.0)
        .set_offset(100.0)
        .set_number_of_periods(3.0)
        .set_frequency(5.0)
        .generate();
    plot_data(sig.get_data(), &format!("Triangle Wave"), ("Signal value", "time [s]")).unwrap();
    fourier.clear();
    fft(&sig, &mut fourier);
    plot_data(fourier.get_data(), &format!("Fast_Fourier_Transform_for_Triangle_wave"), ("Frequency [Hz]", "Amplitude")).unwrap();

}
