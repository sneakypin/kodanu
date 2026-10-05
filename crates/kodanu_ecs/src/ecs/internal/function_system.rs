use crate::{System, SystemParam, SystemParamFunction, WorldCell};

use std::marker::PhantomData;

pub struct FunctionSystem<F, P> {
    function: F,
    marker: PhantomData<fn() -> P>,
}

impl<F, P> FunctionSystem<F, P> {
    pub fn new(function: F) -> Self {
        Self {
            function,
            marker: PhantomData,
        }
    }
}

impl<F, P> System for FunctionSystem<F, P>
where
    P: SystemParam + 'static,
    F: SystemParamFunction<P> + 'static,
{
    fn run(&mut self, cell: WorldCell) {
        self.function.run(P::fetch(cell));
    }
}
