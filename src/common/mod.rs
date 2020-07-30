macro_rules! opt_builder {
    ($field:ident, $field_type:ty) => {
        pub fn $field(&mut self, $field: $field_type) -> &mut Self {
            self.$field = Some($field);
            self
        }
    };
}
