fn main() {
    let fwav = fliters::tts("excuse me while I kiss the sky");
    let samples = fwav.get_samples();
    println!("number of samples: {:?}", samples.iter().len());
    println!("sample rate: {:?}", fwav.sample_rate);

    // Waveform visualizations typically show amplitude over time
    // in chart speak, y-axis is amplitude and x-axis is time
    // below, we map our samples to (x,y) tuples representing (time, amplitude)
    // following the equations:
    //
    // time = sample index / sample rate
    // amplitude = sample value / max sample value (cheating a little by not using bit depth)
    //
    // side note: if we were using bit depth the equation would be:
    // amp = sample value / 2^(16 - 1) for 16bit audio
    // (substitute 16 for 8, 24, 32 for 8-bit, 24-bit, etc)
    let max = samples.iter().max().expect("Whoops, can't compute max");
    let points: Vec<(f64, f64)> = samples
        .iter()
        .enumerate()
        .map(|(index, &sample)| {
            let time = index as f64 / fwav.sample_rate as f64;
            let amp = sample as f64 / *max as f64;
            (time, amp)
        })
        .collect();

    // Now that we have points, we apply a basic linear transformation (min-max scaling)
    // in order to "plot" them in our terminal.
    let width = 140;
    let height = 20;
    let mut screen = vec![vec![' '; width]; height];

    // Min/Max values for both time and amplitude are required for the linear transformation
    // using fold here, similar to reduce in other languages
    let (min_time, max_time, min_amp, max_amp) = points.iter().fold(
        (
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
        ),
        |(min_t, max_t, min_a, max_a), (t, a)| {
            (min_t.min(*t), max_t.max(*t), min_a.min(*a), max_a.max(*a))
        },
    );

    // For min-max scaling we follow:
    // given a range |imin, imax| to transform to the range |i'min, i'max|
    // we iterate through each i, and transform to i' using the equation:
    // i' = i'min + (i - imin) / (imax - imin) * (i'max - i'min)
    //
    // Below, we are transforming |tmin, tmax| to |t'min, t'max| for time
    // and |amin, amax| to |a'min, a'max| for amplitude
    for (time, amp) in points {
        let x = ((time - min_time) / (max_time - min_time) * (width - 1) as f64) as usize;
        let y = ((amp - min_amp) / (max_amp - min_amp) * (height - 1) as f64) as usize;
        if x < width && y < height {
            screen[y][x] = '*';
        }
    }

    for row in screen {
        println!("{}", row.iter().collect::<String>());
    }

    fwav.play();
}
