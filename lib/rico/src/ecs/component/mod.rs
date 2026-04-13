pub mod camera;
pub mod transform;

pub trait Bundle {
    fn into_iter(self) -> Box<dyn Iterator<Item = Box<dyn Component>>>;
}

impl<T: Component + 'static> Bundle for T {
    fn into_iter(self) -> Box<dyn Iterator<Item = Box<dyn Component>>> {
        Box::new(std::iter::once(Box::new(self) as Box<dyn Component>))
    }
}

macro_rules! impl_bundle_tuple {
    ($($name:ident),+) => {
        #[allow(non_snake_case)]
        impl<$($name: Component + 'static),+> Bundle for ($($name,)+) {
            fn into_iter(self) -> Box<dyn Iterator<Item = Box<dyn Component>>> {
                let ($($name,)+) = self;
                Box::new(vec![$(Box::new($name) as Box<dyn Component>),+].into_iter())
            }
        }
    };
}

impl_bundle_tuple!(A);
impl_bundle_tuple!(A, B);
impl_bundle_tuple!(A, B, C);
impl_bundle_tuple!(A, B, C, D);
impl_bundle_tuple!(A, B, C, D, E);
impl_bundle_tuple!(A, B, C, D, E, F);
impl_bundle_tuple!(A, B, C, D, E, F, G);
impl_bundle_tuple!(A, B, C, D, E, F, G, H);
impl_bundle_tuple!(A, B, C, D, E, F, G, H, I);
impl_bundle_tuple!(A, B, C, D, E, F, G, H, I, J);

pub trait Component {
    fn as_any(&self) -> &dyn std::any::Any;

    fn setup(&mut self) {}

    fn update(&mut self) {}

    fn shutdown(&mut self) {}
}
