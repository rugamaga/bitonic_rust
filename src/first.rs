pub fn sort(x: &mut [u32], up: bool) {
    if x.len() <= 1 { return }
    let mid_point = x.len() / 2;
    sort(&mut x[..mid_point], true);
    sort(&mut x[mid_point..], false);
    sub_sort(x, up);
}

fn sub_sort(x: &mut [u32], up: bool) {
    if x.len() <= 1 { return }
    compare_and_swap(x, up);
    let mid_point = x.len() / 2;
    sub_sort(&mut x[..mid_point], up);
    sub_sort(&mut x[mid_point..], up);
}

fn compare_and_swap(x: &mut [u32], up: bool) {
    let mid_point = x.len() / 2;
    for i in  0..mid_point {
        if (x[i] > x[mid_point + i]) == up {
            x.swap(i, mid_point + i);
        }
    }
}
