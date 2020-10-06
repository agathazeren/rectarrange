pub trait Arrange {
    fn size(&self) -> (f64, f64);
}

pub struct PageLayout {
    pub height: f64,
    pub width: f64,
}

pub struct Placement {
    pub x: f64,
    pub y: f64,
    pub transpose: bool,
}

#[non_exhaustive]
pub enum ArrangeError {
    
}

pub fn arrange<'obj, Object: Arrange>(
    objects: &'obj [Object],
    page: PageLayout,
) -> Result<Vec<(&'obj Object, Placement)>, ArrangeError> {
    unimplemented!()
}
