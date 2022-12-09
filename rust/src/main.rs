mod core;

fn main() {
    let mut data: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let a = core::Tensor {
        data: &mut data,
        shape: vec![3, 3],
        stride: vec![3, 1],
        offset: 0,
    };
    a.print();
    println!();

    // get column
    let b = core::Tensor {
        data: &mut data,
        shape: vec![3, 1],
        stride: vec![3, 0],
        offset: 0,
    };
    b.print();
    println!();

    // get row
    let c = core::Tensor {
        data: &mut data,
        shape: vec![1, 3],
        stride: vec![0, 1],
        offset: 0,
    };
    c.print();
    println!();
}
