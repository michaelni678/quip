pub fn into_array<T, const N: usize>(value: impl TryInto<[T; N]>) -> [T; N] {
    let Ok(array) = value.try_into() else {
        panic!("failed to convert value into an array of size {N}");
    };

    array
}
