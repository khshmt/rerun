// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/datatypes/class_description_map_elem.fbs".

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

/// **Datatype**: A helper type for mapping class IDs to class descriptions.
///
/// This is internal to the `AnnotationContext` structure.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ClassDescriptionMapElem {
    /// The key: the class ID.
    pub class_id: crate::datatypes::ClassId,

    /// The value: class name, color, etc.
    pub class_description: crate::datatypes::ClassDescription,
}

impl<'a> From<ClassDescriptionMapElem> for ::std::borrow::Cow<'a, ClassDescriptionMapElem> {
    #[inline]
    fn from(value: ClassDescriptionMapElem) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a ClassDescriptionMapElem> for ::std::borrow::Cow<'a, ClassDescriptionMapElem> {
    #[inline]
    fn from(value: &'a ClassDescriptionMapElem) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl ::re_types_core::Loggable for ClassDescriptionMapElem {
    type Name = ::re_types_core::DatatypeName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.ClassDescriptionMapElem".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::Struct(vec![
            Field {
                name: "class_id".to_owned(),
                data_type: <crate::datatypes::ClassId>::arrow_datatype(),
                is_nullable: false,
                metadata: [].into(),
            },
            Field {
                name: "class_description".to_owned(),
                data_type: <crate::datatypes::ClassDescription>::arrow_datatype(),
                is_nullable: false,
                metadata: [].into(),
            },
        ])
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
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let bitmap: Option<::arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            StructArray::new(
                <crate::datatypes::ClassDescriptionMapElem>::arrow_datatype(),
                vec![
                    {
                        let (somes, class_id): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| {
                                    let Self { class_id, .. } = &**datum;
                                    class_id.clone()
                                });
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let class_id_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        PrimitiveArray::new(
                            DataType::UInt16,
                            class_id
                                .into_iter()
                                .map(|datum| {
                                    datum
                                        .map(|datum| {
                                            let crate::datatypes::ClassId(data0) = datum;
                                            data0
                                        })
                                        .unwrap_or_default()
                                })
                                .collect(),
                            class_id_bitmap,
                        )
                        .boxed()
                    },
                    {
                        let (somes, class_description): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| {
                                    let Self {
                                        class_description, ..
                                    } = &**datum;
                                    class_description.clone()
                                });
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let class_description_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            _ = class_description_bitmap;
                            crate::datatypes::ClassDescription::to_arrow_opt(class_description)?
                        }
                    },
                ],
                bitmap,
            )
            .boxed()
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
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<::arrow2::array::StructArray>()
                .ok_or_else(|| {
                    ::re_types_core::DeserializationError::datatype_mismatch(
                        DataType::Struct(vec![
                            Field {
                                name: "class_id".to_owned(),
                                data_type: <crate::datatypes::ClassId>::arrow_datatype(),
                                is_nullable: false,
                                metadata: [].into(),
                            },
                            Field {
                                name: "class_description".to_owned(),
                                data_type: <crate::datatypes::ClassDescription>::arrow_datatype(),
                                is_nullable: false,
                                metadata: [].into(),
                            },
                        ]),
                        arrow_data.data_type().clone(),
                    )
                })
                .with_context("rerun.datatypes.ClassDescriptionMapElem")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_fields, arrow_data_arrays) =
                    (arrow_data.fields(), arrow_data.values());
                let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data_fields
                    .iter()
                    .map(|field| field.name.as_str())
                    .zip(arrow_data_arrays)
                    .collect();
                let class_id = {
                    if !arrays_by_name.contains_key("class_id") {
                        return Err(::re_types_core::DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "class_id",
                        ))
                        .with_context("rerun.datatypes.ClassDescriptionMapElem");
                    }
                    let arrow_data = &**arrays_by_name["class_id"];
                    arrow_data
                        .as_any()
                        .downcast_ref::<UInt16Array>()
                        .ok_or_else(|| {
                            ::re_types_core::DeserializationError::datatype_mismatch(
                                DataType::UInt16,
                                arrow_data.data_type().clone(),
                            )
                        })
                        .with_context("rerun.datatypes.ClassDescriptionMapElem#class_id")?
                        .into_iter()
                        .map(|opt| opt.copied())
                        .map(|res_or_opt| res_or_opt.map(|v| crate::datatypes::ClassId(v)))
                };
                let class_description = {
                    if !arrays_by_name.contains_key("class_description") {
                        return Err(::re_types_core::DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "class_description",
                        ))
                        .with_context("rerun.datatypes.ClassDescriptionMapElem");
                    }
                    let arrow_data = &**arrays_by_name["class_description"];
                    crate::datatypes::ClassDescription::from_arrow_opt(arrow_data)
                        .with_context("rerun.datatypes.ClassDescriptionMapElem#class_description")?
                        .into_iter()
                };
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    ::itertools::izip!(class_id, class_description),
                    arrow_data.validity(),
                )
                .map(|opt| {
                    opt.map(|(class_id, class_description)| {
                        Ok(Self {
                            class_id: class_id
                                .ok_or_else(::re_types_core::DeserializationError::missing_data)
                                .with_context("rerun.datatypes.ClassDescriptionMapElem#class_id")?,
                            class_description: class_description
                                .ok_or_else(::re_types_core::DeserializationError::missing_data)
                                .with_context(
                                    "rerun.datatypes.ClassDescriptionMapElem#class_description",
                                )?,
                        })
                    })
                    .transpose()
                })
                .collect::<::re_types_core::DeserializationResult<Vec<_>>>()
                .with_context("rerun.datatypes.ClassDescriptionMapElem")?
            }
        })
    }
}
