// 座標の二乗を合計して平方根を取り、
// ベクターの大きさを計算します。`sqrt()` メソッドを使用して、`v.sqrt()` と同様に
// 平方根を計算します。


fn magnitude(arr: &[f64]) -> f64 {
    arr.iter().map(|x| x * x).sum::<f64>().sqrt()
}

// 大きさを計算し、すべての座標をその大きさで割ることで
// ベクターを正規化します。


fn normalize(arr: &mut [f64]) {
    let mag = magnitude(arr);
    if mag > 0.0 {
        for x in arr.iter_mut() {
            *x /= mag;
        }
    }
}

// 次の `main` を使用して処理をテストします。

fn main() {
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}
