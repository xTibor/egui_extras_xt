pub struct CompassAxisLabels {
    pub(crate) inner: [String; 4],
}

impl From<[&str; 4]> for CompassAxisLabels {
    fn from(source: [&str; 4]) -> Self {
        CompassAxisLabels {
            inner: source.map(String::from),
        }
    }
}

impl CompassAxisLabels {
    pub fn from_slice<T>(source: &[T]) -> CompassAxisLabels
    where
        T: ToString,
    {
        CompassAxisLabels {
            inner: source
                .iter()
                .map(T::to_string)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}
