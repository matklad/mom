extern crate rand;

fn main() {
    for i in 0..1000 {
        let xs = (0..i).map(|_ | rand::random::<i32>() % 92).collect::<Vec<_>>();
        for k in 0..i {
            compare(k, &xs);
        }
    }

    for i in 0..1000 {
        let xs = (0..i).map(|_ | rand::random::<i32>()).collect::<Vec<_>>();
        for k in 0..i {
            compare(k, &xs);
        }
    }

    println!("OK");
}

fn compare<T: Copy + Ord>(k: usize, xs: &[T]) {
    let lhs = trivial_select(k, xs);
    let rhs = mom_select(k, xs);
    assert!(lhs == rhs);
}

fn trivial_select<T: Ord>(k: usize, items: &[T]) -> &T {
    assert!(k < items.len());
    let mut xs: Vec<_> = items.iter().collect();
    xs.sort();
    return xs[k];
}

fn mom_select<T: Ord>(k: usize, items: &[T]) -> &T {
    let refs: Vec<_> = items.iter().collect();
    return inner(k, &refs);

    fn inner<'a, T: Ord>(k: usize, items: &[&'a T]) -> &'a T {
        assert!(k < items.len());
        if items.len() <= 20 {
            return *trivial_select(k, items);
        }

        let medians: Vec<&T> = items.chunks(5).map(|chunk| {
            *trivial_select(chunk.len() / 2, chunk)
        }).collect();
        let true_median: &T = inner(medians.len() / 2, &medians);

        let mut left_bias = true;
        let (left, right): (Vec<_>, Vec<_>) = items.iter().cloned().partition(|&it| {
            it < true_median || (it == true_median && { left_bias = !left_bias; left_bias })
        });

        assert!(left.len() < items.len() * 9 / 10);
        assert!(right.len() < items.len() * 9 / 10);

        if k < left.len() {
            inner(k, &left)
        } else {
            inner(k - left.len(), &right)
        }
    }
}

