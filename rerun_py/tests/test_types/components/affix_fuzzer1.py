# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/testing/components/fuzzy.fbs".

# You can extend this class by creating a "AffixFuzzer1Ext" class in "affix_fuzzer1_ext.py".

from __future__ import annotations

from rerun._baseclasses import (
    BaseDelegatingExtensionArray,
    BaseDelegatingExtensionType,
)

from .. import datatypes

__all__ = ["AffixFuzzer1", "AffixFuzzer1Array", "AffixFuzzer1Type"]


class AffixFuzzer1(datatypes.AffixFuzzer1):
    # You can define your own __init__ function as a member of AffixFuzzer1Ext in affix_fuzzer1_ext.py

    # Note: there are no fields here because AffixFuzzer1 delegates to datatypes.AffixFuzzer1
    pass


class AffixFuzzer1Type(BaseDelegatingExtensionType):
    _TYPE_NAME = "rerun.testing.components.AffixFuzzer1"
    _DELEGATED_EXTENSION_TYPE = datatypes.AffixFuzzer1Type


class AffixFuzzer1Array(BaseDelegatingExtensionArray[datatypes.AffixFuzzer1ArrayLike]):
    _EXTENSION_NAME = "rerun.testing.components.AffixFuzzer1"
    _EXTENSION_TYPE = AffixFuzzer1Type
    _DELEGATED_ARRAY_TYPE = datatypes.AffixFuzzer1Array


AffixFuzzer1Type._ARRAY_TYPE = AffixFuzzer1Array

# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(AffixFuzzer1Type())