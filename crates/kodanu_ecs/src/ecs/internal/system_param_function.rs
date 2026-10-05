use crate::SystemParam;

pub trait SystemParamFunction<P: SystemParam> {
    fn run<'w>(&mut self, params: P::Item<'w>);
}

impl<F> SystemParamFunction<()> for F
where
    F: FnMut(),
{
    fn run<'w>(&mut self, _: ()) {
        self()
    }
}
