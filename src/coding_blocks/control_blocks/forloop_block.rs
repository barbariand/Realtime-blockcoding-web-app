use super::*;

pub struct ForLoopBlock<T: IntoTimes> {
    times: T,
    blocks: Blocks,
}
impl<T: IntoTimes> Block for ForLoopBlock<T> {
    fn preform(&self, state: &mut State) -> Result<(), BlockExecutionError> {
        for _ in 0..(self.times.into_times(state)) {
            self.blocks.execute(state)?
        }
        Ok(())
    }
}
pub trait IntoTimes {
    fn into_times(&self, state: &mut State) -> i32;
}
impl IntoTimes for i32 {
    fn into_times(&self, _: &mut State) -> i32 {
        *self
    }
}
impl IntoTimes for dyn Fn(&mut State) -> i32 {
    fn into_times(&self, state: &mut State) -> i32 {
        self(state)
    }
}
impl<T: IntoTimes> ForLoopBlock<T> {
    fn new(blocks: Blocks, times: T) -> Self {
        Self { times, blocks }
    }
}
