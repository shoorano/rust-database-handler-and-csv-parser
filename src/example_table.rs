#[derive(Debug, PartialEq, Eq)]
pub struct Example {
    pub arg_1: i32,
    pub arg_2: i32,
    pub arg_3: Option<String>,
}

impl Example {
    pub fn init(arg_1: i32, arg_2: i32, arg_3: Option<String> ) -> Self {
        Self {
            arg_1,
            arg_2,
            arg_3
        }
    }
    // https://stackoverflow.com/questions/54057589/how-to-make-a-new-associated-function-with-a-struct-that-has-a-closure-member/54060929#54060929
    // fn mapper<F, T, U>(arg_1: i32, arg_2: i32, arg_3: Option<String>) ->  Example<impl FnMut()>
    // where
    //     T: FromRow,
    //     F: FnMut(T) -> U,
    // {
    //     |(arg_1, arg_2, arg_3)| {
    //         Example { arg_1, arg_2, arg_3 }
    //     }
    // }
}