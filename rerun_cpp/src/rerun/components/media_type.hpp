// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/components/media_type.fbs".

#pragma once

#include "../data_cell.hpp"
#include "../datatypes/utf8.hpp"
#include "../result.hpp"

#include <cstdint>
#include <memory>
#include <string>
#include <utility>

namespace arrow {
    class DataType;
    class MemoryPool;
    class StringBuilder;
} // namespace arrow

namespace rerun {
    namespace components {
        ///[MIME-type](https://en.wikipedia.org/wiki/Media_type) of an entity.
        ///
        /// For instance:
        ///* `text/plain`
        ///* `text/markdown`
        struct MediaType {
            rerun::datatypes::Utf8 value;

            /// Name of the component, used for serialization.
            static const char NAME[];

          public:
            // Extensions to generated type defined in 'media_type_ext.cpp'

            MediaType(const char* media_type) : value(media_type) {}

            /// `text/plain`
            static MediaType plain_text() {
                return "text/plain";
            }

            /// `text/markdown`
            static MediaType markdown() {
                return "text/markdown";
            }

          public:
            MediaType() = default;

            MediaType(rerun::datatypes::Utf8 _value) : value(std::move(_value)) {}

            MediaType& operator=(rerun::datatypes::Utf8 _value) {
                value = std::move(_value);
                return *this;
            }

            MediaType(std::string arg) : value(std::move(arg)) {}

            /// Returns the arrow data type this type corresponds to.
            static const std::shared_ptr<arrow::DataType>& arrow_datatype();

            /// Creates a new array builder with an array of this type.
            static Result<std::shared_ptr<arrow::StringBuilder>> new_arrow_array_builder(
                arrow::MemoryPool* memory_pool
            );

            /// Fills an arrow array builder with an array of this type.
            static Error fill_arrow_array_builder(
                arrow::StringBuilder* builder, const MediaType* elements, size_t num_elements
            );

            /// Creates a Rerun DataCell from an array of MediaType components.
            static Result<rerun::DataCell> to_data_cell(
                const MediaType* instances, size_t num_instances
            );
        };
    } // namespace components
} // namespace rerun