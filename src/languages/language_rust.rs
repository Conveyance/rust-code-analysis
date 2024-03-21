// Code generated; DO NOT EDIT.

use num_derive::FromPrimitive;

#[derive(Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum Rust {
    End = 0,
    Identifier = 1,
    SEMI = 2,
    MacroRulesBANG = 3,
    LPAREN = 4,
    RPAREN = 5,
    LBRACK = 6,
    RBRACK = 7,
    LBRACE = 8,
    RBRACE = 9,
    EQGT = 10,
    COLON = 11,
    DOLLAR = 12,
    TokenRepetitionPatternToken1 = 13,
    PLUS = 14,
    STAR = 15,
    QMARK = 16,
    Block2 = 17,
    Expr = 18,
    Ident = 19,
    Item = 20,
    Lifetime2 = 21,
    Literal = 22,
    Meta = 23,
    Pat = 24,
    Path = 25,
    Stmt = 26,
    Tt = 27,
    Ty = 28,
    Vis = 29,
    PrimitiveType = 30,
    PrimitiveType2 = 31,
    PrimitiveType3 = 32,
    PrimitiveType4 = 33,
    PrimitiveType5 = 34,
    PrimitiveType6 = 35,
    PrimitiveType7 = 36,
    PrimitiveType8 = 37,
    PrimitiveType9 = 38,
    PrimitiveType10 = 39,
    PrimitiveType11 = 40,
    PrimitiveType12 = 41,
    PrimitiveType13 = 42,
    PrimitiveType14 = 43,
    PrimitiveType15 = 44,
    PrimitiveType16 = 45,
    PrimitiveType17 = 46,
    SLASH = 47,
    UNDERSCORE = 48,
    BSLASH = 49,
    DASH = 50,
    EQ = 51,
    DASHGT = 52,
    COMMA = 53,
    COLONCOLON = 54,
    BANG = 55,
    DOT = 56,
    AT = 57,
    AMP = 58,
    HASH = 59,
    PERCENT = 60,
    CARET = 61,
    LT = 62,
    GT = 63,
    PIPE = 64,
    TILDE = 65,
    SQUOTE = 66,
    As = 67,
    Async = 68,
    Await = 69,
    Break = 70,
    Const = 71,
    Continue = 72,
    Default = 73,
    Enum = 74,
    Fn = 75,
    For = 76,
    If = 77,
    Impl = 78,
    Let = 79,
    Loop = 80,
    Match = 81,
    Mod = 82,
    Pub = 83,
    Return = 84,
    Static = 85,
    Struct = 86,
    Trait = 87,
    Type = 88,
    Union = 89,
    Unsafe = 90,
    Use = 91,
    Where = 92,
    While = 93,
    Extern = 94,
    Ref = 95,
    Else = 96,
    DOTDOTDOT = 97,
    In = 98,
    LT2 = 99,
    Dyn = 100,
    MutableSpecifier = 101,
    DOTDOT = 102,
    DOTDOTEQ = 103,
    AMPAMP = 104,
    PIPEPIPE = 105,
    EQEQ = 106,
    BANGEQ = 107,
    LTEQ = 108,
    GTEQ = 109,
    LTLT = 110,
    GTGT = 111,
    PLUSEQ = 112,
    DASHEQ = 113,
    STAREQ = 114,
    SLASHEQ = 115,
    PERCENTEQ = 116,
    AMPEQ = 117,
    PIPEEQ = 118,
    CARETEQ = 119,
    LTLTEQ = 120,
    GTGTEQ = 121,
    Yield = 122,
    Move = 123,
    Try = 124,
    IntegerLiteral = 125,
    DQUOTE = 126,
    DQUOTE2 = 127,
    CharLiteral = 128,
    EscapeSequence = 129,
    True = 130,
    False = 131,
    SLASHSLASH = 132,
    LineCommentToken1 = 133,
    LineCommentToken2 = 134,
    LineCommentToken3 = 135,
    InnerDocComment = 136,
    OuterDocComment = 137,
    SLASHSTAR = 138,
    STARSLASH = 139,
    Shebang = 140,
    Zelf = 141,
    Super = 142,
    Crate = 143,
    Metavariable = 144,
    StringContent = 145,
    RawStringLiteral = 146,
    FloatLiteral = 147,
    OuterDocComment2 = 148,
    InnerDocComment2 = 149,
    ErrorSentinel = 150,
    SourceFile = 151,
    Statement = 152,
    EmptyStatement = 153,
    ExpressionStatement = 154,
    MacroDefinition = 155,
    MacroRule = 156,
    TokenPattern = 157,
    TokenTreePattern = 158,
    TokenBindingPattern = 159,
    TokenRepetitionPattern = 160,
    FragmentSpecifier = 161,
    TokenTree = 162,
    TokenRepetition = 163,
    AttributeItem = 164,
    InnerAttributeItem = 165,
    Attribute = 166,
    ModItem = 167,
    ForeignModItem = 168,
    DeclarationList = 169,
    StructItem = 170,
    UnionItem = 171,
    EnumItem = 172,
    EnumVariantList = 173,
    EnumVariant = 174,
    FieldDeclarationList = 175,
    FieldDeclaration = 176,
    OrderedFieldDeclarationList = 177,
    ExternCrateDeclaration = 178,
    ConstItem = 179,
    StaticItem = 180,
    TypeItem = 181,
    FunctionItem = 182,
    FunctionSignatureItem = 183,
    FunctionModifiers = 184,
    WhereClause = 185,
    WherePredicate = 186,
    ImplItem = 187,
    TraitItem = 188,
    AssociatedType = 189,
    TraitBounds = 190,
    HigherRankedTraitBound = 191,
    RemovedTraitBound = 192,
    TypeParameters = 193,
    ConstParameter = 194,
    ConstrainedTypeParameter = 195,
    OptionalTypeParameter = 196,
    LetDeclaration = 197,
    UseDeclaration = 198,
    UseClause = 199,
    ScopedUseList = 200,
    UseList = 201,
    UseAsClause = 202,
    UseWildcard = 203,
    Parameters = 204,
    SelfParameter = 205,
    VariadicParameter = 206,
    Parameter = 207,
    ExternModifier = 208,
    VisibilityModifier = 209,
    Type2 = 210,
    BracketedType = 211,
    QualifiedType = 212,
    Lifetime = 213,
    ArrayType = 214,
    ForLifetimes = 215,
    FunctionType = 216,
    TupleType = 217,
    UnitType = 218,
    GenericFunction = 219,
    GenericType = 220,
    GenericTypeWithTurbofish = 221,
    BoundedType = 222,
    TypeArguments = 223,
    TypeBinding = 224,
    ReferenceType = 225,
    PointerType = 226,
    EmptyType = 227,
    AbstractType = 228,
    DynamicType = 229,
    ExpressionExceptRange = 230,
    Expression = 231,
    MacroInvocation = 232,
    TokenTree2 = 233,
    DelimTokens = 234,
    NonDelimToken = 235,
    ScopedIdentifier = 236,
    ScopedTypeIdentifier = 237,
    ScopedTypeIdentifier2 = 238,
    RangeExpression = 239,
    UnaryExpression = 240,
    TryExpression = 241,
    ReferenceExpression = 242,
    BinaryExpression = 243,
    AssignmentExpression = 244,
    CompoundAssignmentExpr = 245,
    TypeCastExpression = 246,
    ReturnExpression = 247,
    YieldExpression = 248,
    CallExpression = 249,
    Arguments = 250,
    ArrayExpression = 251,
    ParenthesizedExpression = 252,
    TupleExpression = 253,
    UnitExpression = 254,
    StructExpression = 255,
    FieldInitializerList = 256,
    ShorthandFieldInitializer = 257,
    FieldInitializer = 258,
    BaseFieldInitializer = 259,
    IfExpression = 260,
    LetCondition = 261,
    LetChain2 = 262,
    Condition = 263,
    ElseClause = 264,
    MatchExpression = 265,
    MatchBlock = 266,
    MatchArm = 267,
    MatchArm2 = 268,
    MatchPattern = 269,
    WhileExpression = 270,
    LoopExpression = 271,
    ForExpression = 272,
    ConstBlock = 273,
    ClosureExpression = 274,
    ClosureParameters = 275,
    Label = 276,
    BreakExpression = 277,
    ContinueExpression = 278,
    IndexExpression = 279,
    AwaitExpression = 280,
    FieldExpression = 281,
    UnsafeBlock = 282,
    AsyncBlock = 283,
    TryBlock = 284,
    Block = 285,
    Pattern = 286,
    TuplePattern = 287,
    SlicePattern = 288,
    TupleStructPattern = 289,
    StructPattern = 290,
    FieldPattern = 291,
    RemainingFieldPattern = 292,
    MutPattern = 293,
    RangePattern = 294,
    RefPattern = 295,
    CapturedPattern = 296,
    ReferencePattern = 297,
    OrPattern = 298,
    Literal2 = 299,
    LiteralPattern = 300,
    NegativeLiteral = 301,
    StringLiteral = 302,
    BooleanLiteral = 303,
    LineComment = 304,
    DocComment = 305,
    BlockComment = 306,
    DocComment2 = 307,
    SourceFileRepeat1 = 308,
    MacroDefinitionRepeat1 = 309,
    TokenTreePatternRepeat1 = 310,
    TokenTreeRepeat1 = 311,
    NonSpecialTokenRepeat1 = 312,
    DeclarationListRepeat1 = 313,
    EnumVariantListRepeat1 = 314,
    EnumVariantListRepeat2 = 315,
    FieldDeclarationListRepeat1 = 316,
    OrderedFieldDeclarationListRepeat1 = 317,
    FunctionModifiersRepeat1 = 318,
    WhereClauseRepeat1 = 319,
    TraitBoundsRepeat1 = 320,
    TypeParametersRepeat1 = 321,
    UseListRepeat1 = 322,
    ParametersRepeat1 = 323,
    ForLifetimesRepeat1 = 324,
    TupleTypeRepeat1 = 325,
    TypeArgumentsRepeat1 = 326,
    DelimTokenTreeRepeat1 = 327,
    ArgumentsRepeat1 = 328,
    TupleExpressionRepeat1 = 329,
    FieldInitializerListRepeat1 = 330,
    MatchBlockRepeat1 = 331,
    ClosureParametersRepeat1 = 332,
    TuplePatternRepeat1 = 333,
    SlicePatternRepeat1 = 334,
    StructPatternRepeat1 = 335,
    StringLiteralRepeat1 = 336,
    ClosurePattern = 337,
    FieldIdentifier = 338,
    LetChain = 339,
    ShorthandFieldIdentifier = 340,
    TypeIdentifier = 341,
    Error = 342,
    
}

impl From<Rust> for &'static str {
    #[inline(always)]
    fn from(tok: Rust) -> Self {
        match tok {
            Rust::End => "end",
            Rust::Identifier => "identifier",
            Rust::SEMI => ";",
            Rust::MacroRulesBANG => "macro_rules!",
            Rust::LPAREN => "(",
            Rust::RPAREN => ")",
            Rust::LBRACK => "[",
            Rust::RBRACK => "]",
            Rust::LBRACE => "{",
            Rust::RBRACE => "}",
            Rust::EQGT => "=>",
            Rust::COLON => ":",
            Rust::DOLLAR => "$",
            Rust::TokenRepetitionPatternToken1 => "token_repetition_pattern_token1",
            Rust::PLUS => "+",
            Rust::STAR => "*",
            Rust::QMARK => "?",
            Rust::Block2 => "block",
            Rust::Expr => "expr",
            Rust::Ident => "ident",
            Rust::Item => "item",
            Rust::Lifetime2 => "lifetime",
            Rust::Literal => "literal",
            Rust::Meta => "meta",
            Rust::Pat => "pat",
            Rust::Path => "path",
            Rust::Stmt => "stmt",
            Rust::Tt => "tt",
            Rust::Ty => "ty",
            Rust::Vis => "vis",
            Rust::PrimitiveType => "primitive_type",
            Rust::PrimitiveType2 => "primitive_type",
            Rust::PrimitiveType3 => "primitive_type",
            Rust::PrimitiveType4 => "primitive_type",
            Rust::PrimitiveType5 => "primitive_type",
            Rust::PrimitiveType6 => "primitive_type",
            Rust::PrimitiveType7 => "primitive_type",
            Rust::PrimitiveType8 => "primitive_type",
            Rust::PrimitiveType9 => "primitive_type",
            Rust::PrimitiveType10 => "primitive_type",
            Rust::PrimitiveType11 => "primitive_type",
            Rust::PrimitiveType12 => "primitive_type",
            Rust::PrimitiveType13 => "primitive_type",
            Rust::PrimitiveType14 => "primitive_type",
            Rust::PrimitiveType15 => "primitive_type",
            Rust::PrimitiveType16 => "primitive_type",
            Rust::PrimitiveType17 => "primitive_type",
            Rust::SLASH => "/",
            Rust::UNDERSCORE => "_",
            Rust::BSLASH => "\\",
            Rust::DASH => "-",
            Rust::EQ => "=",
            Rust::DASHGT => "->",
            Rust::COMMA => ",",
            Rust::COLONCOLON => "::",
            Rust::BANG => "!",
            Rust::DOT => ".",
            Rust::AT => "@",
            Rust::AMP => "&",
            Rust::HASH => "#",
            Rust::PERCENT => "%",
            Rust::CARET => "^",
            Rust::LT => "<",
            Rust::GT => ">",
            Rust::PIPE => "|",
            Rust::TILDE => "~",
            Rust::SQUOTE => "'",
            Rust::As => "as",
            Rust::Async => "async",
            Rust::Await => "await",
            Rust::Break => "break",
            Rust::Const => "const",
            Rust::Continue => "continue",
            Rust::Default => "default",
            Rust::Enum => "enum",
            Rust::Fn => "fn",
            Rust::For => "for",
            Rust::If => "if",
            Rust::Impl => "impl",
            Rust::Let => "let",
            Rust::Loop => "loop",
            Rust::Match => "match",
            Rust::Mod => "mod",
            Rust::Pub => "pub",
            Rust::Return => "return",
            Rust::Static => "static",
            Rust::Struct => "struct",
            Rust::Trait => "trait",
            Rust::Type => "type",
            Rust::Union => "union",
            Rust::Unsafe => "unsafe",
            Rust::Use => "use",
            Rust::Where => "where",
            Rust::While => "while",
            Rust::Extern => "extern",
            Rust::Ref => "ref",
            Rust::Else => "else",
            Rust::DOTDOTDOT => "...",
            Rust::In => "in",
            Rust::LT2 => "<",
            Rust::Dyn => "dyn",
            Rust::MutableSpecifier => "mutable_specifier",
            Rust::DOTDOT => "..",
            Rust::DOTDOTEQ => "..=",
            Rust::AMPAMP => "&&",
            Rust::PIPEPIPE => "||",
            Rust::EQEQ => "==",
            Rust::BANGEQ => "!=",
            Rust::LTEQ => "<=",
            Rust::GTEQ => ">=",
            Rust::LTLT => "<<",
            Rust::GTGT => ">>",
            Rust::PLUSEQ => "+=",
            Rust::DASHEQ => "-=",
            Rust::STAREQ => "*=",
            Rust::SLASHEQ => "/=",
            Rust::PERCENTEQ => "%=",
            Rust::AMPEQ => "&=",
            Rust::PIPEEQ => "|=",
            Rust::CARETEQ => "^=",
            Rust::LTLTEQ => "<<=",
            Rust::GTGTEQ => ">>=",
            Rust::Yield => "yield",
            Rust::Move => "move",
            Rust::Try => "try",
            Rust::IntegerLiteral => "integer_literal",
            Rust::DQUOTE => "\"",
            Rust::DQUOTE2 => "\"",
            Rust::CharLiteral => "char_literal",
            Rust::EscapeSequence => "escape_sequence",
            Rust::True => "true",
            Rust::False => "false",
            Rust::SLASHSLASH => "//",
            Rust::LineCommentToken1 => "line_comment_token1",
            Rust::LineCommentToken2 => "line_comment_token2",
            Rust::LineCommentToken3 => "line_comment_token3",
            Rust::InnerDocComment => "inner_doc_comment",
            Rust::OuterDocComment => "outer_doc_comment",
            Rust::SLASHSTAR => "/*",
            Rust::STARSLASH => "*/",
            Rust::Shebang => "shebang",
            Rust::Zelf => "self",
            Rust::Super => "super",
            Rust::Crate => "crate",
            Rust::Metavariable => "metavariable",
            Rust::StringContent => "_string_content",
            Rust::RawStringLiteral => "raw_string_literal",
            Rust::FloatLiteral => "float_literal",
            Rust::OuterDocComment2 => "outer_doc_comment",
            Rust::InnerDocComment2 => "inner_doc_comment",
            Rust::ErrorSentinel => "_error_sentinel",
            Rust::SourceFile => "source_file",
            Rust::Statement => "_statement",
            Rust::EmptyStatement => "empty_statement",
            Rust::ExpressionStatement => "expression_statement",
            Rust::MacroDefinition => "macro_definition",
            Rust::MacroRule => "macro_rule",
            Rust::TokenPattern => "_token_pattern",
            Rust::TokenTreePattern => "token_tree_pattern",
            Rust::TokenBindingPattern => "token_binding_pattern",
            Rust::TokenRepetitionPattern => "token_repetition_pattern",
            Rust::FragmentSpecifier => "fragment_specifier",
            Rust::TokenTree => "token_tree",
            Rust::TokenRepetition => "token_repetition",
            Rust::AttributeItem => "attribute_item",
            Rust::InnerAttributeItem => "inner_attribute_item",
            Rust::Attribute => "attribute",
            Rust::ModItem => "mod_item",
            Rust::ForeignModItem => "foreign_mod_item",
            Rust::DeclarationList => "declaration_list",
            Rust::StructItem => "struct_item",
            Rust::UnionItem => "union_item",
            Rust::EnumItem => "enum_item",
            Rust::EnumVariantList => "enum_variant_list",
            Rust::EnumVariant => "enum_variant",
            Rust::FieldDeclarationList => "field_declaration_list",
            Rust::FieldDeclaration => "field_declaration",
            Rust::OrderedFieldDeclarationList => "ordered_field_declaration_list",
            Rust::ExternCrateDeclaration => "extern_crate_declaration",
            Rust::ConstItem => "const_item",
            Rust::StaticItem => "static_item",
            Rust::TypeItem => "type_item",
            Rust::FunctionItem => "function_item",
            Rust::FunctionSignatureItem => "function_signature_item",
            Rust::FunctionModifiers => "function_modifiers",
            Rust::WhereClause => "where_clause",
            Rust::WherePredicate => "where_predicate",
            Rust::ImplItem => "impl_item",
            Rust::TraitItem => "trait_item",
            Rust::AssociatedType => "associated_type",
            Rust::TraitBounds => "trait_bounds",
            Rust::HigherRankedTraitBound => "higher_ranked_trait_bound",
            Rust::RemovedTraitBound => "removed_trait_bound",
            Rust::TypeParameters => "type_parameters",
            Rust::ConstParameter => "const_parameter",
            Rust::ConstrainedTypeParameter => "constrained_type_parameter",
            Rust::OptionalTypeParameter => "optional_type_parameter",
            Rust::LetDeclaration => "let_declaration",
            Rust::UseDeclaration => "use_declaration",
            Rust::UseClause => "_use_clause",
            Rust::ScopedUseList => "scoped_use_list",
            Rust::UseList => "use_list",
            Rust::UseAsClause => "use_as_clause",
            Rust::UseWildcard => "use_wildcard",
            Rust::Parameters => "parameters",
            Rust::SelfParameter => "self_parameter",
            Rust::VariadicParameter => "variadic_parameter",
            Rust::Parameter => "parameter",
            Rust::ExternModifier => "extern_modifier",
            Rust::VisibilityModifier => "visibility_modifier",
            Rust::Type2 => "_type",
            Rust::BracketedType => "bracketed_type",
            Rust::QualifiedType => "qualified_type",
            Rust::Lifetime => "lifetime",
            Rust::ArrayType => "array_type",
            Rust::ForLifetimes => "for_lifetimes",
            Rust::FunctionType => "function_type",
            Rust::TupleType => "tuple_type",
            Rust::UnitType => "unit_type",
            Rust::GenericFunction => "generic_function",
            Rust::GenericType => "generic_type",
            Rust::GenericTypeWithTurbofish => "generic_type_with_turbofish",
            Rust::BoundedType => "bounded_type",
            Rust::TypeArguments => "type_arguments",
            Rust::TypeBinding => "type_binding",
            Rust::ReferenceType => "reference_type",
            Rust::PointerType => "pointer_type",
            Rust::EmptyType => "empty_type",
            Rust::AbstractType => "abstract_type",
            Rust::DynamicType => "dynamic_type",
            Rust::ExpressionExceptRange => "_expression_except_range",
            Rust::Expression => "_expression",
            Rust::MacroInvocation => "macro_invocation",
            Rust::TokenTree2 => "token_tree",
            Rust::DelimTokens => "_delim_tokens",
            Rust::NonDelimToken => "_non_delim_token",
            Rust::ScopedIdentifier => "scoped_identifier",
            Rust::ScopedTypeIdentifier => "scoped_type_identifier",
            Rust::ScopedTypeIdentifier2 => "scoped_type_identifier",
            Rust::RangeExpression => "range_expression",
            Rust::UnaryExpression => "unary_expression",
            Rust::TryExpression => "try_expression",
            Rust::ReferenceExpression => "reference_expression",
            Rust::BinaryExpression => "binary_expression",
            Rust::AssignmentExpression => "assignment_expression",
            Rust::CompoundAssignmentExpr => "compound_assignment_expr",
            Rust::TypeCastExpression => "type_cast_expression",
            Rust::ReturnExpression => "return_expression",
            Rust::YieldExpression => "yield_expression",
            Rust::CallExpression => "call_expression",
            Rust::Arguments => "arguments",
            Rust::ArrayExpression => "array_expression",
            Rust::ParenthesizedExpression => "parenthesized_expression",
            Rust::TupleExpression => "tuple_expression",
            Rust::UnitExpression => "unit_expression",
            Rust::StructExpression => "struct_expression",
            Rust::FieldInitializerList => "field_initializer_list",
            Rust::ShorthandFieldInitializer => "shorthand_field_initializer",
            Rust::FieldInitializer => "field_initializer",
            Rust::BaseFieldInitializer => "base_field_initializer",
            Rust::IfExpression => "if_expression",
            Rust::LetCondition => "let_condition",
            Rust::LetChain2 => "_let_chain",
            Rust::Condition => "_condition",
            Rust::ElseClause => "else_clause",
            Rust::MatchExpression => "match_expression",
            Rust::MatchBlock => "match_block",
            Rust::MatchArm => "match_arm",
            Rust::MatchArm2 => "match_arm",
            Rust::MatchPattern => "match_pattern",
            Rust::WhileExpression => "while_expression",
            Rust::LoopExpression => "loop_expression",
            Rust::ForExpression => "for_expression",
            Rust::ConstBlock => "const_block",
            Rust::ClosureExpression => "closure_expression",
            Rust::ClosureParameters => "closure_parameters",
            Rust::Label => "label",
            Rust::BreakExpression => "break_expression",
            Rust::ContinueExpression => "continue_expression",
            Rust::IndexExpression => "index_expression",
            Rust::AwaitExpression => "await_expression",
            Rust::FieldExpression => "field_expression",
            Rust::UnsafeBlock => "unsafe_block",
            Rust::AsyncBlock => "async_block",
            Rust::TryBlock => "try_block",
            Rust::Block => "block",
            Rust::Pattern => "_pattern",
            Rust::TuplePattern => "tuple_pattern",
            Rust::SlicePattern => "slice_pattern",
            Rust::TupleStructPattern => "tuple_struct_pattern",
            Rust::StructPattern => "struct_pattern",
            Rust::FieldPattern => "field_pattern",
            Rust::RemainingFieldPattern => "remaining_field_pattern",
            Rust::MutPattern => "mut_pattern",
            Rust::RangePattern => "range_pattern",
            Rust::RefPattern => "ref_pattern",
            Rust::CapturedPattern => "captured_pattern",
            Rust::ReferencePattern => "reference_pattern",
            Rust::OrPattern => "or_pattern",
            Rust::Literal2 => "_literal",
            Rust::LiteralPattern => "_literal_pattern",
            Rust::NegativeLiteral => "negative_literal",
            Rust::StringLiteral => "string_literal",
            Rust::BooleanLiteral => "boolean_literal",
            Rust::LineComment => "line_comment",
            Rust::DocComment => "doc_comment",
            Rust::BlockComment => "block_comment",
            Rust::DocComment2 => "doc_comment",
            Rust::SourceFileRepeat1 => "source_file_repeat1",
            Rust::MacroDefinitionRepeat1 => "macro_definition_repeat1",
            Rust::TokenTreePatternRepeat1 => "token_tree_pattern_repeat1",
            Rust::TokenTreeRepeat1 => "token_tree_repeat1",
            Rust::NonSpecialTokenRepeat1 => "_non_special_token_repeat1",
            Rust::DeclarationListRepeat1 => "declaration_list_repeat1",
            Rust::EnumVariantListRepeat1 => "enum_variant_list_repeat1",
            Rust::EnumVariantListRepeat2 => "enum_variant_list_repeat2",
            Rust::FieldDeclarationListRepeat1 => "field_declaration_list_repeat1",
            Rust::OrderedFieldDeclarationListRepeat1 => "ordered_field_declaration_list_repeat1",
            Rust::FunctionModifiersRepeat1 => "function_modifiers_repeat1",
            Rust::WhereClauseRepeat1 => "where_clause_repeat1",
            Rust::TraitBoundsRepeat1 => "trait_bounds_repeat1",
            Rust::TypeParametersRepeat1 => "type_parameters_repeat1",
            Rust::UseListRepeat1 => "use_list_repeat1",
            Rust::ParametersRepeat1 => "parameters_repeat1",
            Rust::ForLifetimesRepeat1 => "for_lifetimes_repeat1",
            Rust::TupleTypeRepeat1 => "tuple_type_repeat1",
            Rust::TypeArgumentsRepeat1 => "type_arguments_repeat1",
            Rust::DelimTokenTreeRepeat1 => "delim_token_tree_repeat1",
            Rust::ArgumentsRepeat1 => "arguments_repeat1",
            Rust::TupleExpressionRepeat1 => "tuple_expression_repeat1",
            Rust::FieldInitializerListRepeat1 => "field_initializer_list_repeat1",
            Rust::MatchBlockRepeat1 => "match_block_repeat1",
            Rust::ClosureParametersRepeat1 => "closure_parameters_repeat1",
            Rust::TuplePatternRepeat1 => "tuple_pattern_repeat1",
            Rust::SlicePatternRepeat1 => "slice_pattern_repeat1",
            Rust::StructPatternRepeat1 => "struct_pattern_repeat1",
            Rust::StringLiteralRepeat1 => "string_literal_repeat1",
            Rust::ClosurePattern => "closure_pattern",
            Rust::FieldIdentifier => "field_identifier",
            Rust::LetChain => "let_chain",
            Rust::ShorthandFieldIdentifier => "shorthand_field_identifier",
            Rust::TypeIdentifier => "type_identifier",
            Rust::Error => "ERROR",
            
        }
    }
}

impl From<u16> for Rust {
    #[inline(always)]
    fn from(x: u16) -> Self {
        num::FromPrimitive::from_u16(x).unwrap_or(Self::Error)
    }
}

// Rust == u16
impl PartialEq<u16> for Rust {
    #[inline(always)]
    fn eq(&self, x: &u16) -> bool {
        *self == Rust::from(*x)
    }
}

// u16 == Rust
impl PartialEq<Rust> for u16 {
    #[inline(always)]
    fn eq(&self, x: &Rust) -> bool {
        *x == *self
    }
}