// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/components/transform3d.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// **Component**: An affine transform between two 3D spaces, represented in a given direction.
#[derive(Clone, Debug, PartialEq)]
pub struct Transform3D(
    /// Representation of the transform.
    pub crate::datatypes::Transform3D,
);

impl<T: Into<crate::datatypes::Transform3D>> From<T> for Transform3D {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<crate::datatypes::Transform3D> for Transform3D {
    #[inline]
    fn borrow(&self) -> &crate::datatypes::Transform3D {
        &self.0
    }
}

impl std::ops::Deref for Transform3D {
    type Target = crate::datatypes::Transform3D;

    #[inline]
    fn deref(&self) -> &crate::datatypes::Transform3D {
        &self.0
    }
}

impl<'a> From<Transform3D> for ::std::borrow::Cow<'a, Transform3D> {
    #[inline]
    fn from(value: Transform3D) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a Transform3D> for ::std::borrow::Cow<'a, Transform3D> {
    #[inline]
    fn from(value: &'a Transform3D) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl ::re_types_core::Loggable for Transform3D {
    type Name = ::re_types_core::ComponentName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.components.Transform3D".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::Union(
            vec![
                Field {
                    name: "_null_markers".to_owned(),
                    data_type: DataType::Null,
                    is_nullable: true,
                    metadata: [].into(),
                },
                Field {
                    name: "TranslationAndMat3x3".to_owned(),
                    data_type: <crate::datatypes::TranslationAndMat3x3>::arrow_datatype(),
                    is_nullable: false,
                    metadata: [].into(),
                },
                Field {
                    name: "TranslationRotationScale".to_owned(),
                    data_type: <crate::datatypes::TranslationRotationScale3D>::arrow_datatype(),
                    is_nullable: false,
                    metadata: [].into(),
                },
            ],
            Some(vec![0i32, 1i32, 2i32]),
            UnionMode::Dense,
        )
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> ::re_types_core::SerializationResult<Box<dyn ::arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        re_tracing::profile_function!();
        use ::arrow2::{array::*, datatypes::*};
        use ::re_types_core::{Loggable as _, ResultExt as _};
        Ok({
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| {
                        let Self(data0) = datum.into_owned();
                        data0
                    });
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            {
                _ = data0_bitmap;
                crate::datatypes::Transform3D::to_arrow_opt(data0)?
            }
        })
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn from_arrow_opt(
        arrow_data: &dyn ::arrow2::array::Array,
    ) -> ::re_types_core::DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        re_tracing::profile_function!();
        use ::arrow2::{array::*, buffer::*, datatypes::*};
        use ::re_types_core::{Loggable as _, ResultExt as _};
        Ok(crate::datatypes::Transform3D::from_arrow_opt(arrow_data)
            .with_context("rerun.components.Transform3D#repr")?
            .into_iter()
            .map(|v| v.ok_or_else(::re_types_core::DeserializationError::missing_data))
            .map(|res| res.map(|v| Some(Self(v))))
            .collect::<::re_types_core::DeserializationResult<Vec<Option<_>>>>()
            .with_context("rerun.components.Transform3D#repr")
            .with_context("rerun.components.Transform3D")?)
    }
}
