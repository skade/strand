pub trait State {}

impl State for int {}
impl State for Box<int> {}