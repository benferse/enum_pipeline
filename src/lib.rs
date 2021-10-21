use std::marker::PhantomData;

/// Provides an execute handler for pipelines.
pub trait Execute {
    /// Execute a pipeline call to this instance.
    /// Responsible for invoking the relevant handler(s).
    fn execute(self);
}

/// Provides an execute handler for pipelines, with a argument of type `TArg`.
pub trait ExecuteWith<TArg> {
    /// Execute a pipeline call to this instance with an argument.
    /// Responsible for invoking the relevant handler(s).
    fn execute(self, arg: &TArg);
}

/// Provides an Execute handler for pipelines, with a mutable argument of type `TArg`.
pub trait ExecuteWithMut<TArg> {
    /// Execute a pipeline call to this instance with a mutable argument.
    /// Responsible for invoking the relevant handler(s).
    fn execute(self, arg: &mut TArg);
}

/// A pipeline vector which represents a series of `Execute`-able operations.
pub struct PipelineVec<T> {
    /// The ordered step of operations.
    steps: Vec<T>,
}

/// `Execute`-ing to a `PipelineVec<T>` executing the `steps` in order.
impl<T> Execute for PipelineVec<T>
where
    T: Execute,
{
    fn execute(self) {
        for step in self.steps {
            step.execute()
        }
    }
}

/// A pipeline vector which represents a series of `ExecuteWith`-able operations with an argument of type `TArg`.
pub struct PipelineVecWith<T, TArg> {
    /// The ordered step of operations.
    steps: Vec<T>,

    /// Phantom data to remember the argument type with.
    arg_type: PhantomData<TArg>,
}

/// `Execute`-ing to a `PipelineVecWith<T, TArg>` executes the `steps` in order, passing `arg` along.
impl<T, TArg> ExecuteWith<TArg> for PipelineVecWith<T, TArg>
where
    T: ExecuteWith<TArg>,
{
    fn execute(self, arg: &TArg) {
        for step in self.steps {
            step.execute(arg)
        }
    }
}

/// `Execute`-ing to a `PipelineVecWith<T, TArg>` executes the `steps` in order, passing a mutable `arg` along.
impl<T, TArg> ExecuteWithMut<TArg> for PipelineVecWith<T, TArg>
where
    T: ExecuteWithMut<TArg>,
{
    fn execute(self, arg: &mut TArg) {
        for step in self.steps {
            step.execute(arg)
        }
    }
}

/// Provides a way to convert into a `PipelineVec` for ordered execution.
pub trait IntoPipelineVec<T>
where
    T: Execute,
{
    /// Creates a `PipelineVec` that can be executed, consuming the source.
    fn into_pipeline(self) -> PipelineVec<T>;
}

/// Provides a way to convert into a `PipelineVecWith` for ordered execution with an argument of type `TArg`.
pub trait IntoPipelineVecWith<T, TArg>
where
    T: ExecuteWith<TArg>,
{
    /// Creates a `PipelineVecWith` that can be executed with an argument, consuming the source.
    fn into_pipeline(self) -> PipelineVecWith<T, TArg>;
}

/// Provides a way to convert into a `PipelineVecWith` for ordered execution with a mutable argument of type `TArg`.
pub trait IntoPipelineVecWithMut<T, TArg>
where
    T: ExecuteWithMut<TArg>,
{
    /// Creates a `PipelineVecWith` that can be executed with a mutable argument, consuming the source.
    fn into_pipeline(self) -> PipelineVecWith<T, TArg>;
}

/// Provides a way to convert a `Vec<>` of `Execute`-able elements into a `PipelineVec` for execution.
impl<T> IntoPipelineVec<T> for Vec<T>
where
    T: Execute,
{
    /// Creates a `PipelineVec` that can be executed, consuming the source `Vec`.
    fn into_pipeline(self) -> PipelineVec<T> {
        PipelineVec { steps: self }
    }
}

/// Provides a way to convert a `Vec<>` of `Execute`-able elements into a `PipelineVecWith` for ordered execution with an argument of type `TArg`.
impl<T, TArg> IntoPipelineVecWith<T, TArg> for Vec<T>
where
    T: ExecuteWith<TArg>,
{
    /// Creates a `PipelineVecWith` that can be executed with an argument, consuming the source `Vec`.
    fn into_pipeline(self) -> PipelineVecWith<T, TArg> {
        PipelineVecWith {
            steps: self,
            arg_type: PhantomData,
        }
    }
}

/// Provides a way to convert a `Vec<>` of `Execute`-able elements into a `PipelineVecWithMut` for ordered execution with an argument of type `TArg`.
impl<T, TArg> IntoPipelineVecWithMut<T, TArg> for Vec<T>
where
    T: ExecuteWithMut<TArg>,
{
    /// Creates a `PipelineVecWith` that can be executed with a mutable argument, consuming the source `Vec`.
    fn into_pipeline(self) -> PipelineVecWith<T, TArg> {
        PipelineVecWith {
            steps: self,
            arg_type: PhantomData,
        }
    }
}

#[cfg(test)]
mod readme_test {
    use crate::{Execute, IntoPipelineVec};

    enum Operations {
        Allocate(f32, f32),
        Init,
        Run(f32),
    }

    impl Execute for Operations {
        fn execute(self) {
            match self {
                Operations::Allocate(_x, _y) => println!("allocate something"),
                Operations::Init => println!("init"),
                Operations::Run(_delta) => println!("do work"),
            }
        }
    }

    #[test]
    fn do_work() {
        let my_op_pipeline = vec![
            Operations::Init,
            Operations::Allocate(1.0, 1.0),
            Operations::Init,
            Operations::Run(1.0),
        ]
        .into_pipeline();

        my_op_pipeline.execute();
        // prints:
        // init
        // allocate something
        // init
        // do work
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Execute, ExecuteWith, ExecuteWithMut, IntoPipelineVec, IntoPipelineVecWith,
        IntoPipelineVecWithMut,
    };
    use enum_pipeline_derive::Execute;

    #[derive(Execute)]
    enum VoidDispatchPipeline {
        #[handler(VoidDispatchPipeline::handle_one)]
        One,
        #[handler(handle_two)]
        Two,
    }

    static mut VOID_ONE_COUNT: i32 = 0;
    static mut VOID_TWO_COUNT: i32 = 0;

    impl VoidDispatchPipeline {
        fn handle_one() {
            unsafe {
                VOID_ONE_COUNT += 1;
            }
        }

        fn handle_two() {
            unsafe {
                VOID_TWO_COUNT += 1;
            }
        }
    }

    #[test]
    fn void_dispatch_works() {
        let pipeline = vec![VoidDispatchPipeline::One, VoidDispatchPipeline::Two].into_pipeline();

        pipeline.execute();

        unsafe {
            assert_eq!(1, VOID_ONE_COUNT);
            assert_eq!(1, VOID_TWO_COUNT);
        }
    }

    enum RefDataPipeline {
        One(f32),
        Two,
    }

    static mut REF_ONE_VALUE: f32 = 0.0;
    static mut REF_TWO_COUNT: i32 = 0;

    struct RefDataPipelineData {
        mult: f32,
    }

    impl RefDataPipeline {
        fn handle_one(v: f32, arg: &RefDataPipelineData) {
            unsafe {
                REF_ONE_VALUE += v * arg.mult;
            }
        }

        fn handle_two(_arg: &RefDataPipelineData) {
            unsafe {
                REF_TWO_COUNT += 1;
            }
        }
    }

    impl ExecuteWith<RefDataPipelineData> for RefDataPipeline {
        fn execute(self, arg: &RefDataPipelineData) {
            match self {
                RefDataPipeline::One(f) => RefDataPipeline::handle_one(f, arg),
                RefDataPipeline::Two => RefDataPipeline::handle_two(arg),
            }
        }
    }

    #[test]
    fn ref_data_pipeline_works() {
        let pipeline = vec![RefDataPipeline::One(24.0), RefDataPipeline::Two].into_pipeline();

        let data = RefDataPipelineData { mult: 2.0 };

        pipeline.execute(&data);

        unsafe {
            assert_eq!(48.0, REF_ONE_VALUE);
            assert_eq!(1, REF_TWO_COUNT);
        }
    }

    enum MutDataPipeline {
        One(f32),
        Two,
    }

    #[derive(Default)]
    struct MutDataPipelineData {
        one_value: f32,
        two_count: i32,
    }

    // no macro yet, srry
    impl ExecuteWithMut<MutDataPipelineData> for MutDataPipeline {
        fn execute(self, arg: &mut MutDataPipelineData) {
            match self {
                MutDataPipeline::One(f) => arg.one_value += f,
                MutDataPipeline::Two => arg.two_count += 1,
            }
        }
    }

    #[test]
    fn mut_data_pipeline_works() {
        let pipeline = vec![MutDataPipeline::One(12.0), MutDataPipeline::Two].into_pipeline();

        let mut data = MutDataPipelineData::default();
        pipeline.execute(&mut data);

        assert_eq!(12.0, data.one_value);
        assert_eq!(1, data.two_count);
    }
}
