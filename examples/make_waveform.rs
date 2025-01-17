fn main() {
    let fwav = fliters::tts("excuse me while I kiss the sky");
    let samples = fwav.get_samples();
    println!("number of samples: {:?}", samples.iter().len());
    println!("sample rate: {:?}", fwav.sample_rate);

    let max = samples.iter().max().expect("Whoops, can't compute max");

    let points: Vec<(f64, f64)> = samples
        .iter()
        .enumerate()
        .map(|(index, &sample)| {
            let time = index as f64 / fwav.sample_rate as f64;
            let amp = sample as f64 / *max as f64;

            // println!("index and sample: {:?}, {:?}", index, sample);
            // println!("computed time and amp: {:?}, {:?}", time, amp);
            (time, amp)
        })
        .collect();

    let width = 140;
    let height = 20;
    let mut screen = vec![vec![' '; width]; height];

    // Find actual min/max values
    let max_time = points.iter().map(|(t, _)| *t).fold(0.0, f64::max);
    let min_time = points.iter().map(|(t, _)| *t).fold(0.0, f64::min);
    let max_amp = points.iter().map(|(_, a)| *a).fold(0.0, f64::max);
    let min_amp = points.iter().map(|(_, a)| *a).fold(0.0, f64::min);

    for (time, amp) in points {
        // Normalize time to screen width
        let x = ((time - min_time) / (max_time - min_time) * (width - 1) as f64) as usize;
        // Map amplitude from [-1,1] to [0,height-1]
        // let y = ((amp + 1.0) * (height - 1) as f64 / 2.0) as usize;
        // let y = (height - 1) - ((amp + 1.0) * (height - 1) as f64 / 2.0) as usize;
        let y = ((amp - min_amp) / (max_amp - min_amp) * (height - 1) as f64) as usize;
        if x < width && y < height {
            screen[y][x] = '*';
        }
    }

    for row in screen {
        println!("{}", row.iter().collect::<String>());
    }
}
