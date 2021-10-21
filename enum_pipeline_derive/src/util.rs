use syn::{Attribute, Fields};

pub trait OfRelevantType<T> {
    fn of_relevant_type(self, ty: &str) -> T;
}

impl OfRelevantType<Vec<Attribute>> for Vec<Attribute> {
    fn of_relevant_type(self, ty: &str) -> Vec<Attribute> {
        self.into_iter()
            .filter_map(
                |attr| match attr.path.get_ident().map_or(false, |p| p == ty) {
                    true => Some(attr),
                    _ => None,
                },
            )
            .collect()
    }
}

pub trait AsGeneratedIdent<T> {
    fn as_generated_ident(self, prefix: &str) -> Vec<T>;
}

impl AsGeneratedIdent<String> for Fields {
    fn as_generated_ident(self, prefix: &str) -> Vec<String> {
        self.into_iter()
            .enumerate()
            .map(|(i, _)| format!("{}{}", prefix, i + 1))
            .collect()
    }
}
