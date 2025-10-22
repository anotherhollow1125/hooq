use std::ops::Deref;

#[derive(Debug)]
pub struct FlavorPath(Vec<String>);

impl Deref for FlavorPath {
    type Target = [String];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&syn::Path> for FlavorPath {
    type Error = String;

    fn try_from(value: &syn::Path) -> Result<Self, Self::Error> {
        let inner = value
            .segments
            .iter()
            .map(|seg| {
                if !seg.arguments.is_none() {
                    return Err("path with generic arguments is not supported here".to_string());
                }

                let flavor_name = seg.ident.to_string();

                if flavor_name.is_empty() {
                    return Err("flavor names must not include empty one".to_string());
                }

                Ok(flavor_name)
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(Self(inner))
    }
}

impl TryFrom<syn::Path> for FlavorPath {
    type Error = String;

    fn try_from(value: syn::Path) -> Result<Self, Self::Error> {
        FlavorPath::try_from(&value)
    }
}

impl TryFrom<String> for FlavorPath {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let inner = value
            .split('.')
            .flat_map(|s| s.split("::"))
            .map(|s| {
                if s.is_empty() {
                    return Err("flavor names must not include empty one".to_string());
                }

                Ok(s.to_string())
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(Self(inner))
    }
}
