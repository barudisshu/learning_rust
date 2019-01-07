/// 合并，将两个集合，逐个比较，写入新的集合
fn merge<T: Copy + PartialOrd + std::fmt::Debug>(x1: &[T], x2: &[T], y: &mut [T]) {
    assert_eq!(x1.len() + x2.len(), y.len());
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < x1.len() && j < x2.len() {
        if x1[i] < x2[j] {
            y[k] = x1[i];
            k += 1;
            i += 1;
        } else {
            y[k] = x2[j];
            k += 1;
            j += 1;
        }
    }
    if i < x1.len() {
        y[k..].copy_from_slice(&x1[i..]);
    }
    if j < x2.len() {
        y[k..].copy_from_slice(&x2[j..]);
    }
    println!("{:?} + {:?} => {:?}", &x1, &x2, &y);
}

fn merge_sort<T: Copy + PartialOrd + std::fmt::Debug>(x: &mut [T]) {
    let n = x.len();
    let mut y = x.to_vec();
    let mut len = 1;
    while len < n {
        let mut i = 0;
        while i < n {
            if i + len >= n {
                y[i..].copy_from_slice(&x[i..]);
            } else if i + 2 * len > n {
                merge(&x[i..i + len], &x[i + len..], &mut y[i..]);
            } else {
                merge(&x[i..i + len], &x[i + len..i + 2 * len], &mut y[i..i + 2 * len]);
            }
            i += 2 * len;
        }
        len *= 2;
        if len >= n {
            x.copy_from_slice(&y);
            return;
        }
        i = 0;
        while i < n {
            if i + len >= n {
                x[i..].copy_from_slice(&y[i..]);
            } else if i + 2 * len > n {
                merge(&y[i..i + len], &y[i + len..], &mut x[i..]);
            } else {
                merge(&y[i..i + len], &y[i + len..i + 2 * len], &mut x[i..i + 2 * len]);
            }
            i += 2 * len;
        }
        len *= 2;
    }
}

#[test]
fn sort() {
    let mut vec = vec![4, -1, 0, 2, 3, 5, 1];
    println!("{:?}", vec);
    println!();
    let expect = vec![-1, 0, 1, 2, 3, 4, 5];

    merge_sort(&mut vec);
    assert_eq!(expect, vec);
}