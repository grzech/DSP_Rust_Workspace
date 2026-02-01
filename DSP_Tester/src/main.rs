use plotter::plot_data;
use dsp_lib::{fft, DescreteSignal, Generator, fir_filter, get_low_pass_fir_coefficients};

fn main() {
    /*
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
    println!("Draw sum of Sine Waves");
    plot_data(sig.get_data(), &format!("Sum of Sine Waves"), ("time [s]", "Signal value")).unwrap();
    let mut fourier = DescreteSignal::new();
    fft(&sig, &mut fourier);
    plot_data(fourier.get_data(), &format!("Fast_Fourier_Transform_for_Sum_of_harmonics"), ("Frequency [Hz]", "Amplitude")).unwrap();
    
    let sig = Generator::rectangle_wave(1.0, 0.5)
        .set_amplitude(0.78)
        .set_sampling_rate(1000.0)
        .set_offset(0.3)
        .set_phase_shift(0.5)
        .set_number_of_periods(5.0)
        //.set_duty_cycle(0.1)
        .generate();
    println!("Draw Rectangle Wave");
    plot_data(sig.get_data(), &format!("Rectangle Wave"), ("time [s]", "Signal value")).unwrap();
    fourier.clear();
    fft(&sig, &mut fourier);
    plot_data(fourier.get_data(), &format!("Fast_Fourier_Transform_for_Rectangle_wave"), ("Frequency [Hz]", "Amplitude")).unwrap();
    
    println!("Filter rectangle wave");
    let mut fir = [0.0; 50];
    let mut output = DescreteSignal::new();
    get_low_pass_fir_coefficients(50, &mut fir);
    fir_filter(&sig, &fir, &mut output);
    println!("Draw filtered signal");
    plot_data(output.get_data(), &format!("Filtered Rectangle Wave"), ("time [s]", "Signal value")).unwrap();
    fourier.clear();
    fft(&output, &mut fourier);
    plot_data(fourier.get_data(), &format!("Fast_Fourier_Transform_for_Filtered_Rectangle"), ("Frequency [Hz]", "Amplitude")).unwrap();
    */
    let mut fir = [0.0; 10];
    let mut output = DescreteSignal::new();
    get_low_pass_fir_coefficients(10, &mut fir);
    let mut fourier = DescreteSignal::new();
    let mut output = DescreteSignal::new();
    println!("Calculate impulse response");
    let dirac = Generator::dirac_delta()
        .set_amplitude(1.0)
        .set_sampling_rate(1000.0)
        .set_phase_shift(0.5)
        .generate();
    plot_data(dirac.get_data(), &format!("Dirac Delta"), ("time [s]", "Signal value")).unwrap();
    output.clear();
    let mut fir_plot_data = [(0.0, 0.0); 10];
    for (i, val) in fir.into_iter().enumerate() {
        fir_plot_data[i] = (i as f64, val);
    }
    plot_data(&fir_plot_data, &format!("FIR coefficients"), ("sample", "Coefficient value")).unwrap();
    println!("Filter");
    fir_filter(&dirac, &fir, &mut output);
    println!("Plot impulse response");
    plot_data(output.get_data(), &format!("FIR impulse response"), ("time [s]", "Signal value")).unwrap();
    fourier.clear();
    println!("Calculate FFT");
    fft(&output, &mut fourier);
    println!("Plot FFT");
    plot_data(fourier.get_data(), &format!("Spectrum_For_FIR_Impulse_Response"), ("Frequency [Hz]", "Amplitude")).unwrap();
    /*
    let sig = Generator::triangle_wave(1.0)
        .set_amplitude(30.0)
        .set_sampling_rate(100.0)
        .set_offset(100.0)
        .set_number_of_periods(3.0)
        .set_frequency(5.0)
        .generate();
    println!("Draw Triangle Wave");
    plot_data(sig.get_data(), &format!("Triangle Wave"), ("time [s]", "Signal value")).unwrap();
    fourier.clear();
    fft(&sig, &mut fourier);
    plot_data(fourier.get_data(), &format!("Fast_Fourier_Transform_for_Triangle_wave"), ("Frequency [Hz]", "Amplitude")).unwrap();
    */
    
}
