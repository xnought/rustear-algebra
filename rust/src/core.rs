pub struct Tensor<'a> {
    pub data: &'a mut Vec<f32>,
    pub shape: Vec<usize>,
    pub stride: Vec<usize>,
    pub offset: usize,
}

impl Tensor<'_> {
    pub fn print(&self) {
        for i in 0..self.shape[0] {
            for j in 0..self.shape[1] {
                print!(
                    "{} ",
                    self.data[self.offset + i * self.stride[0] + j * self.stride[1]]
                );
            }
            println!();
        }
    }
}
