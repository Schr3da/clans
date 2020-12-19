use std::slice::Iter;

pub trait EnumId {
    fn as_id(&self) -> &'static str;
}

pub trait EnumIterator<T: 'static>
where
    T: EnumIterator<T>,
{
    fn iter() -> Iter<'static, T>;

    fn has_match(to_match: &'static str) -> Option<&T> {
        for item in Self::iter() {
            let id = item.as_id();
            let value = item.as_label();

            if id == to_match || value == to_match {
                return Option::Some(&item);
            }
        }

        Option::None
    }

    fn as_id(&self) -> &'static str;

    fn as_label(&self) -> &'static str;

    fn as_glyph(&self) -> char {
        ' '
    }
}
