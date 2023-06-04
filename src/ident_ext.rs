use quote::format_ident;
use syn::Ident;

/// Convenience methods on `Ident`s.
pub trait IdentExt {
    /// Returns a new `Ident` by appending this Ident and the specified suffix.
    ///
    /// # Parameters
    ///
    /// * `suffix`: Suffix to append.
    fn append<S>(&self, suffix: S) -> Ident
    where
        S: quote::IdentFragment;

    /// Returns a new `Ident` by prepending this Ident with the specified
    /// prefix.
    ///
    /// # Parameters
    ///
    /// * `prefix`: Prefix to prepend.
    fn prepend<S>(&self, prefix: S) -> Ident
    where
        S: quote::IdentFragment;
}

impl IdentExt for Ident {
    fn append<S>(&self, suffix: S) -> Ident
    where
        S: quote::IdentFragment,
    {
        format_ident!("{}{}", self, suffix)
    }

    fn prepend<S>(&self, suffix: S) -> Ident
    where
        S: quote::IdentFragment,
    {
        format_ident!("{}{}", suffix, self)
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use syn::Ident;

    use super::IdentExt;

    #[test]
    fn append_str_returns_appended_ident() {
        let one = Ident::new("One", Span::call_site());

        assert_eq!(Ident::new("OneTwo", Span::call_site()), one.append("Two"));
    }

    #[test]
    fn append_ident_returns_appended_ident() {
        let one = Ident::new("One", Span::call_site());
        let two = Ident::new("Two", Span::call_site());

        assert_eq!(Ident::new("OneTwo", Span::call_site()), one.append(two));
    }

    #[test]
    fn append_ident_ref_returns_appended_ident() {
        let one = Ident::new("One", Span::call_site());
        let two = Ident::new("Two", Span::call_site());

        assert_eq!(Ident::new("OneTwo", Span::call_site()), one.append(two));
    }

    #[test]
    fn prepend_str_returns_prepended_ident() {
        let one = Ident::new("One", Span::call_site());

        assert_eq!(Ident::new("TwoOne", Span::call_site()), one.prepend("Two"));
    }

    #[test]
    fn prepend_ident_returns_prepended_ident() {
        let one = Ident::new("One", Span::call_site());
        let two = Ident::new("Two", Span::call_site());

        assert_eq!(Ident::new("TwoOne", Span::call_site()), one.prepend(two));
    }

    #[test]
    fn prepend_ident_ref_returns_prepended_ident() {
        let one = Ident::new("One", Span::call_site());
        let two = Ident::new("Two", Span::call_site());

        assert_eq!(Ident::new("TwoOne", Span::call_site()), one.prepend(two));
    }
}
