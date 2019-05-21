pub fn run() {
    let buffer: &mut [i32; 15] = &mut [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let coefficients: [i64; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let qlp_shift: i16 = 2;

    for i in 12..buffer.len() {
        let prediction = coefficients
            .iter()
            .zip(&buffer[i - 12..i])
            .map(|(&c, &s)| c * s as i64)
            .sum::<i64>()
            >> qlp_shift;

        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }

    println!("{:?}", buffer);
}
