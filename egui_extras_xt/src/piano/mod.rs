mod key_metrics;
mod widget;

pub use widget::PianoWidget;

/*
pub fn hit_test(x: isize, y: isize) -> Option<usize> {
    if (x < 0) || (y < 0) || (y > OCTAVE_HEIGHT) {
        return None;
    }

    let (key_octave, x) = ((x / OCTAVE_WIDTH) as usize, x % OCTAVE_WIDTH);

    KEY_METRICS
        .iter()
        .enumerate()
        .filter(|(_, key_metrics)| {
            let ((left, top), (right, bottom)) = key_metrics.bounds;
            (x >= left) && (x < right) && (y >= top) && (y < bottom)
        })
        .sorted_by_key(|(_, key_metrics)| key_metrics.z_index)
        .next()
        .map(|(key_index, _)| key_octave * 12 + key_index)
}
*/
