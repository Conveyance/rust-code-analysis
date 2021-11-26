// Code generated; DO NOT EDIT.

const SPECIALS: &[&str] = &[
    "NULL",
    "bool",
    "char",
    "char16_t",
    "char32_t",
    "char64_t",
    "char8_t",
    "charptr_t",
    "const",
    "constexpr",
    "double",
    "explicit",
    "false",
    "float",
    "inline",
    "int",
    "int16_t",
    "int32_t",
    "int64_t",
    "int8_t",
    "int_fast16_t",
    "int_fast32_t",
    "int_fast64_t",
    "int_fast8_t",
    "int_least16_t",
    "int_least32_t",
    "int_least64_t",
    "int_least8_t",
    "intmax_t",
    "intptr_t",
    "long",
    "max_align_t",
    "mutable",
    "namespace",
    "nullptr",
    "ptrdiff_t",
    "restrict",
    "short",
    "signed",
    "size_t",
    "ssize_t",
    "static",
    "true",
    "uint16_t",
    "uint32_t",
    "uint64_t",
    "uint8_t",
    "uint_fast16_t",
    "uint_fast32_t",
    "uint_fast64_t",
    "uint_fast8_t",
    "uint_least16_t",
    "uint_least32_t",
    "uint_least64_t",
    "uint_least8_t",
    "uintmax_t",
    "uintptr_t",
    "unsigned",
    "wchar_t",
];

pub fn is_specials(mac: &str) -> bool {
    SPECIALS.contains(&mac)
}