// Code generated; DO NOT EDIT.

#[derive(Clone, Debug, PartialEq, FromPrimitive)]
pub enum Cpp {
    End = 0,
    Identifier = 1,
    HASHinclude = 2,
    LF = 3,
    HASHdefine = 4,
    LPAREN = 5,
    DOTDOTDOT = 6,
    COMMA = 7,
    RPAREN = 8,
    HASHif = 9,
    HASHendif = 10,
    HASHifdef = 11,
    HASHifndef = 12,
    HASHelse = 13,
    HASHelif = 14,
    PreprocDirective = 15,
    PreprocArg = 16,
    LPAREN2 = 17,
    Defined = 18,
    BANG = 19,
    TILDE = 20,
    DASH = 21,
    PLUS = 22,
    STAR = 23,
    SLASH = 24,
    PERCENT = 25,
    PIPEPIPE = 26,
    AMPAMP = 27,
    PIPE = 28,
    CARET = 29,
    AMP = 30,
    EQEQ = 31,
    BANGEQ = 32,
    GT = 33,
    GTEQ = 34,
    LTEQ = 35,
    LT = 36,
    LTLT = 37,
    GTGT = 38,
    SEMI = 39,
    Typedef = 40,
    Extern = 41,
    Attribute2 = 42,
    Declspec = 43,
    Based = 44,
    Cdecl = 45,
    Clrcall = 46,
    Stdcall = 47,
    Fastcall = 48,
    Thiscall = 49,
    Vectorcall = 50,
    MsRestrictModifier = 51,
    MsUnsignedPtrModifier = 52,
    MsSignedPtrModifier = 53,
    Unaligned = 54,
    Unaligned2 = 55,
    LBRACE = 56,
    RBRACE = 57,
    LBRACK = 58,
    RBRACK = 59,
    EQ = 60,
    Static = 61,
    Register = 62,
    Inline = 63,
    Const = 64,
    Volatile = 65,
    Restrict = 66,
    Atomic = 67,
    Mutable = 68,
    Constexpr = 69,
    Signed = 70,
    Unsigned = 71,
    Long = 72,
    Short = 73,
    PrimitiveType = 74,
    Enum = 75,
    Class = 76,
    Struct = 77,
    Union = 78,
    COLON = 79,
    If = 80,
    Else = 81,
    Switch = 82,
    Case = 83,
    Default = 84,
    While = 85,
    Do = 86,
    For = 87,
    Return = 88,
    Break = 89,
    Continue = 90,
    Goto = 91,
    QMARK = 92,
    STAREQ = 93,
    SLASHEQ = 94,
    PERCENTEQ = 95,
    PLUSEQ = 96,
    DASHEQ = 97,
    LTLTEQ = 98,
    GTGTEQ = 99,
    AMPEQ = 100,
    CARETEQ = 101,
    PIPEEQ = 102,
    DASHDASH = 103,
    PLUSPLUS = 104,
    Sizeof = 105,
    DOT = 106,
    DASHGT = 107,
    NumberLiteral = 108,
    LSQUOTE = 109,
    USQUOTE = 110,
    USQUOTE2 = 111,
    U8SQUOTE = 112,
    SQUOTE = 113,
    CharLiteralToken1 = 114,
    LDQUOTE = 115,
    UDQUOTE = 116,
    UDQUOTE2 = 117,
    U8DQUOTE = 118,
    DQUOTE = 119,
    StringLiteralToken1 = 120,
    EscapeSequence = 121,
    SystemLibString = 122,
    True = 123,
    False = 124,
    Null = 125,
    Comment = 126,
    Decltype2 = 127,
    Final = 128,
    Override = 129,
    Virtual = 130,
    Explicit = 131,
    Public = 132,
    Private = 133,
    Protected = 134,
    Auto = 135,
    Typename = 136,
    Template = 137,
    GT2 = 138,
    COLONCOLON = 139,
    Operator = 140,
    Delete = 141,
    Friend = 142,
    Noexcept2 = 143,
    Throw = 144,
    Namespace = 145,
    Using = 146,
    StaticAssert = 147,
    Try = 148,
    Catch = 149,
    LBRACKLBRACK = 150,
    RBRACKRBRACK = 151,
    New = 152,
    OperatorName = 153,
    This = 154,
    Nullptr = 155,
    AloneMacro = 156,
    AloneMacroCallToken1 = 157,
    MOZDECLUSEGUARDOBJECTNOTIFIER = 158,
    MOZALLOCATOR = 159,
    MOZALLOWTEMPORARY = 160,
    MOZALWAYSINLINE = 161,
    MOZALWAYSINLINEEVENDEBUG = 162,
    MOZASANBLACKLIST = 163,
    MOZCANRUNSCRIPT = 164,
    MOZCANRUNSCRIPTBOUNDARY = 165,
    MOZCANRUNSCRIPTFORDEFINITION = 166,
    MOZCOLD = 167,
    MOZFALLTHROUGH = 168,
    MOZFORMATPRINTF = 169,
    MOZHAVEANALYZERNORETURN = 170,
    MOZHAVEASANBLACKLIST = 171,
    MOZHAVENEVERINLINE = 172,
    MOZHAVENORETURN = 173,
    MOZHAVENORETURNPTR = 174,
    MOZHAVENOSANITIZEATTR = 175,
    MOZHAVESIGNEDOVERFLOWSANITIZEATTR = 176,
    MOZHAVEUNSIGNEDOVERFLOWSANITIZEATTR = 177,
    MOZHEAPALLOCATOR = 178,
    MOZHEAPCLASS = 179,
    MOZIMPLICIT = 180,
    MOZINHERITTYPEANNOTATIONSFROMTEMPLATEARGS = 181,
    MOZINITOUTSIDECTOR = 182,
    MOZISCLASSINIT = 183,
    MOZISREFPTR = 184,
    MOZISSMARTPTRTOREFCOUNTED = 185,
    MOZMAYBEUNUSED = 186,
    MOZMAYCALLAFTERMUSTRETURN = 187,
    MOZMUSTOVERRIDE = 188,
    MOZMUSTRETURNFROMCALLERIFTHISISARG = 189,
    MOZMUSTUSE = 190,
    MOZMUSTUSETYPE = 191,
    MOZNEEDSMEMMOVABLEMEMBERS = 192,
    MOZNEEDSMEMMOVABLETYPE = 193,
    MOZNEEDSNOVTABLETYPE = 194,
    MOZNEVERINLINE = 195,
    MOZNEVERINLINEDEBUG = 196,
    MOZNONHEAPCLASS = 197,
    MOZNONNULL = 198,
    MOZNONNULLRETURN = 199,
    MOZNONAUTOABLE = 200,
    MOZNONMEMMOVABLE = 201,
    MOZNONOWNINGREF = 202,
    MOZNONPARAM = 203,
    MOZNONTEMPORARYCLASS = 204,
    MOZNORETURN = 205,
    MOZNORETURNPTR = 206,
    MOZNOADDREFRELEASEONRETURN = 207,
    MOZNOARITHMETICEXPRINARGUMENT = 208,
    MOZNODANGLINGONTEMPORARIES = 209,
    MOZNOSANITIZESIGNEDOVERFLOW = 210,
    MOZNOSANITIZEUNSIGNEDOVERFLOW = 211,
    MOZONLYUSEDTOAVOIDSTATICCONSTRUCTORS = 212,
    MOZOWNINGREF = 213,
    MOZPOPDISABLENONTRIVIALUNIONWARNINGS = 214,
    MOZPRETENDNORETURNFORSTATICANALYSIS = 215,
    MOZPUSHDISABLENONTRIVIALUNIONWARNINGS = 216,
    MOZRAII = 217,
    MOZREQUIREDBASEMETHOD = 218,
    MOZSTACKCLASS = 219,
    MOZSTATICCLASS = 220,
    MOZSTATICLOCALCLASS = 221,
    MOZTEMPORARYCLASS = 222,
    MOZTRIVIALCTORDTOR = 223,
    MOZTSANBLACKLIST = 224,
    MOZUNSAFEREF = 225,
    MOZXPCOMABI = 226,
    RawStringLiteral = 227,
    TranslationUnit = 228,
    PreprocInclude = 229,
    PreprocDef = 230,
    PreprocFunctionDef = 231,
    PreprocParams = 232,
    PreprocCall = 233,
    PreprocIf = 234,
    PreprocIfdef = 235,
    PreprocElse = 236,
    PreprocElif = 237,
    PreprocIf2 = 238,
    PreprocIfdef2 = 239,
    PreprocElse2 = 240,
    PreprocElif2 = 241,
    PreprocExpression = 242,
    ParenthesizedExpression = 243,
    PreprocDefined = 244,
    UnaryExpression = 245,
    CallExpression = 246,
    ArgumentList = 247,
    BinaryExpression = 248,
    FunctionDefinition = 249,
    Declaration = 250,
    TypeDefinition = 251,
    DeclarationSpecifiers = 252,
    LinkageSpecification = 253,
    AttributeSpecifier = 254,
    MsDeclspecModifier = 255,
    MsBasedModifier = 256,
    MsCallModifier = 257,
    MsUnalignedPtrModifier = 258,
    MsPointerModifier = 259,
    DeclarationList = 260,
    Declarator = 261,
    FieldDeclarator = 262,
    TypeDeclarator = 263,
    AbstractDeclarator = 264,
    ParenthesizedDeclarator = 265,
    ParenthesizedDeclarator2 = 266,
    ParenthesizedDeclarator3 = 267,
    AbstractParenthesizedDeclarator = 268,
    PointerDeclarator = 269,
    PointerDeclarator2 = 270,
    PointerDeclarator3 = 271,
    AbstractPointerDeclarator = 272,
    FunctionDeclarator = 273,
    FunctionDeclarator2 = 274,
    FunctionDeclarator3 = 275,
    AbstractFunctionDeclarator = 276,
    ArrayDeclarator = 277,
    ArrayDeclarator2 = 278,
    ArrayDeclarator3 = 279,
    AbstractArrayDeclarator = 280,
    InitDeclarator = 281,
    CompoundStatement = 282,
    StorageClassSpecifier = 283,
    TypeQualifier = 284,
    TypeSpecifier = 285,
    SizedTypeSpecifier = 286,
    EnumSpecifier = 287,
    EnumeratorList = 288,
    StructSpecifier = 289,
    UnionSpecifier = 290,
    FieldDeclarationList = 291,
    FieldDeclarationListItem = 292,
    FieldDeclaration = 293,
    BitfieldClause = 294,
    Enumerator = 295,
    ParameterList = 296,
    ParameterDeclaration = 297,
    LabeledStatement = 298,
    ExpressionStatement = 299,
    IfStatement = 300,
    SwitchStatement = 301,
    CaseStatement = 302,
    WhileStatement = 303,
    DoStatement = 304,
    ForStatement = 305,
    ReturnStatement = 306,
    BreakStatement = 307,
    ContinueStatement = 308,
    GotoStatement = 309,
    Expression = 310,
    CommaExpression = 311,
    ConditionalExpression = 312,
    AssignmentExpression = 313,
    PointerExpression = 314,
    UnaryExpression2 = 315,
    BinaryExpression2 = 316,
    UpdateExpression = 317,
    CastExpression = 318,
    TypeDescriptor = 319,
    SizeofExpression = 320,
    SubscriptExpression = 321,
    CallExpression2 = 322,
    ArgumentList2 = 323,
    FieldExpression = 324,
    CompoundLiteralExpression = 325,
    ParenthesizedExpression2 = 326,
    InitializerList = 327,
    InitializerPair = 328,
    SubscriptDesignator = 329,
    FieldDesignator = 330,
    CharLiteral = 331,
    ConcatenatedString = 332,
    StringLiteral = 333,
    EmptyDeclaration = 334,
    Decltype = 335,
    ClassSpecifier = 336,
    ClassName = 337,
    VirtualSpecifier = 338,
    VirtualFunctionSpecifier = 339,
    ExplicitFunctionSpecifier = 340,
    BaseClassClause = 341,
    EnumBaseClause = 342,
    DependentType = 343,
    TemplateDeclaration = 344,
    TemplateInstantiation = 345,
    TemplateParameterList = 346,
    TypeParameterDeclaration = 347,
    VariadicTypeParameterDeclaration = 348,
    OptionalTypeParameterDeclaration = 349,
    TemplateTemplateParameterDeclaration = 350,
    OptionalParameterDeclaration = 351,
    VariadicParameterDeclaration = 352,
    VariadicDeclarator = 353,
    ReferenceDeclarator = 354,
    OperatorCast = 355,
    FieldInitializerList = 356,
    FieldInitializer = 357,
    FunctionDefinition2 = 358,
    ConstructorSpecifiers = 359,
    FunctionDefinition3 = 360,
    Declaration2 = 361,
    FunctionDefinition4 = 362,
    Declaration3 = 363,
    DefaultMethodClause = 364,
    DeleteMethodClause = 365,
    FriendDeclaration = 366,
    AccessSpecifier = 367,
    ReferenceDeclarator2 = 368,
    ReferenceDeclarator3 = 369,
    AbstractReferenceDeclarator = 370,
    StructuredBindingDeclarator = 371,
    TrailingReturnType = 372,
    Noexcept = 373,
    ThrowSpecifier = 374,
    TemplateType = 375,
    TemplateMethod = 376,
    TemplateFunction = 377,
    TemplateArgumentList = 378,
    NamespaceDefinition = 379,
    UsingDeclaration = 380,
    AliasDeclaration = 381,
    StaticAssertDeclaration = 382,
    ConditionClause = 383,
    Declaration4 = 384,
    ForRangeLoop = 385,
    ThrowStatement = 386,
    TryStatement = 387,
    CatchClause = 388,
    Attribute = 389,
    NewExpression = 390,
    NewDeclarator = 391,
    DeleteExpression = 392,
    LambdaExpression = 393,
    LambdaCaptureSpecifier = 394,
    LambdaDefaultCapture = 395,
    ParameterPackExpansion = 396,
    ParameterPackExpansion2 = 397,
    DestructorName = 398,
    ScopedFieldIdentifier = 399,
    ScopedIdentifier = 400,
    ScopedTypeIdentifier = 401,
    ScopedNamespaceIdentifier = 402,
    AloneMacroCall = 403,
    MacroStatement = 404,
    MacroAnnotation = 405,
    TranslationUnitRepeat1 = 406,
    PreprocParamsRepeat1 = 407,
    PreprocIfInFieldDeclarationListRepeat1 = 408,
    PreprocArgumentListRepeat1 = 409,
    FunctionDefinitionRepeat1 = 410,
    DeclarationRepeat1 = 411,
    TypeDefinitionRepeat1 = 412,
    TypeDefinitionRepeat2 = 413,
    DeclarationSpecifiersRepeat1 = 414,
    PointerDeclaratorRepeat1 = 415,
    FunctionDeclaratorRepeat1 = 416,
    FunctionDeclaratorRepeat2 = 417,
    AbstractFunctionDeclaratorRepeat1 = 418,
    SizedTypeSpecifierRepeat1 = 419,
    EnumeratorListRepeat1 = 420,
    FieldDeclarationRepeat1 = 421,
    ParameterListRepeat1 = 422,
    CaseStatementRepeat1 = 423,
    ArgumentListRepeat1 = 424,
    InitializerListRepeat1 = 425,
    InitializerPairRepeat1 = 426,
    ConcatenatedStringRepeat1 = 427,
    StringLiteralRepeat1 = 428,
    BaseClassClauseRepeat1 = 429,
    TemplateParameterListRepeat1 = 430,
    FieldInitializerListRepeat1 = 431,
    StructuredBindingDeclaratorRepeat1 = 432,
    ThrowSpecifierRepeat1 = 433,
    TemplateArgumentListRepeat1 = 434,
    TryStatementRepeat1 = 435,
    AttributeRepeat1 = 436,
    AloneMacroCallRepeat1 = 437,
    FieldIdentifier = 438,
    NamespaceIdentifier = 439,
    StatementIdentifier = 440,
    TypeIdentifier = 441,
    Error = 442,
}

#[allow(clippy::unreadable_literal)]
static KEYS: phf::Map<&'static str, Cpp> = ::phf::Map {
    key: 3213172566270843353,
    disps: ::phf::Slice::Static(&[
        (0, 277),
        (0, 2),
        (0, 36),
        (0, 1),
        (0, 176),
        (0, 0),
        (0, 277),
        (0, 2),
        (0, 18),
        (0, 40),
        (0, 5),
        (3, 211),
        (0, 245),
        (0, 216),
        (0, 192),
        (0, 36),
        (0, 15),
        (0, 69),
        (0, 165),
        (0, 216),
        (1, 206),
        (0, 172),
        (1, 232),
        (2, 174),
        (0, 231),
        (0, 379),
        (0, 8),
        (0, 18),
        (1, 395),
        (1, 334),
        (0, 135),
        (0, 11),
        (0, 10),
        (3, 216),
        (0, 18),
        (0, 0),
        (0, 78),
        (0, 0),
        (0, 4),
        (0, 303),
        (1, 38),
        (0, 0),
        (0, 74),
        (0, 310),
        (1, 197),
        (0, 378),
        (2, 324),
        (0, 127),
        (0, 6),
        (0, 150),
        (0, 1),
        (0, 21),
        (0, 8),
        (2, 3),
        (0, 171),
        (0, 7),
        (0, 1),
        (0, 19),
        (0, 1),
        (0, 348),
        (0, 338),
        (2, 121),
        (0, 169),
        (4, 297),
        (0, 137),
        (4, 180),
        (0, 84),
        (5, 217),
        (17, 235),
        (0, 11),
        (4, 387),
        (9, 162),
        (11, 94),
        (3, 84),
        (0, 11),
        (2, 368),
        (2, 2),
        (0, 16),
        (0, 239),
        (8, 74),
        (14, 297),
        (2, 161),
    ]),
    entries: ::phf::Slice::Static(&[
        ("MOZ_FALLTHROUGH", Cpp::MOZFALLTHROUGH),
        ("MOZ_IS_REFPTR", Cpp::MOZISREFPTR),
        ("#elif", Cpp::HASHelif),
        (
            "abstract_reference_declarator",
            Cpp::AbstractReferenceDeclarator,
        ),
        ("continue_statement", Cpp::ContinueStatement),
        ("MOZ_NORETURN_PTR", Cpp::MOZNORETURNPTR),
        ("__cdecl", Cpp::Cdecl),
        ("call_expression", Cpp::CallExpression),
        ("MOZ_CAN_RUN_SCRIPT_BOUNDARY", Cpp::MOZCANRUNSCRIPTBOUNDARY),
        ("parenthesized_expression", Cpp::ParenthesizedExpression),
        ("[", Cpp::LBRACK),
        ("preproc_def", Cpp::PreprocDef),
        ("ms_based_modifier", Cpp::MsBasedModifier),
        (
            "compound_literal_expression",
            Cpp::CompoundLiteralExpression,
        ),
        ("preproc_elif", Cpp::PreprocElif),
        ("long", Cpp::Long),
        ("-=", Cpp::DASHEQ),
        ("\\n", Cpp::LF),
        (
            "explicit_function_specifier",
            Cpp::ExplicitFunctionSpecifier,
        ),
        (
            "scoped_namespace_identifier",
            Cpp::ScopedNamespaceIdentifier,
        ),
        ("end", Cpp::End),
        ("reference_declarator", Cpp::ReferenceDeclarator),
        ("register", Cpp::Register),
        ("struct_specifier", Cpp::StructSpecifier),
        (
            "_declaration_specifiers_repeat1",
            Cpp::DeclarationSpecifiersRepeat1,
        ),
        ("_preproc_expression", Cpp::PreprocExpression),
        (">>", Cpp::GTGT),
        (
            "variadic_type_parameter_declaration",
            Cpp::VariadicTypeParameterDeclaration,
        ),
        ("preproc_directive", Cpp::PreprocDirective),
        ("char_literal_token1", Cpp::CharLiteralToken1),
        ("_type_declarator", Cpp::TypeDeclarator),
        ("template_instantiation", Cpp::TemplateInstantiation),
        ("case_statement", Cpp::CaseStatement),
        ("type_descriptor", Cpp::TypeDescriptor),
        ("L\'", Cpp::LSQUOTE),
        ("type_definition_repeat2", Cpp::TypeDefinitionRepeat2),
        ("MOZ_HEAP_CLASS", Cpp::MOZHEAPCLASS),
        (
            "MOZ_INHERIT_TYPE_ANNOTATIONS_FROM_TEMPLATE_ARGS",
            Cpp::MOZINHERITTYPEANNOTATIONSFROMTEMPLATEARGS,
        ),
        ("namespace_definition", Cpp::NamespaceDefinition),
        ("_type_specifier", Cpp::TypeSpecifier),
        (
            "MOZ_NO_DANGLING_ON_TEMPORARIES",
            Cpp::MOZNODANGLINGONTEMPORARIES,
        ),
        ("comment", Cpp::Comment),
        ("switch_statement", Cpp::SwitchStatement),
        ("MOZ_NEVER_INLINE", Cpp::MOZNEVERINLINE),
        (
            "MOZ_ALWAYS_INLINE_EVEN_DEBUG",
            Cpp::MOZALWAYSINLINEEVENDEBUG,
        ),
        ("ms_unsigned_ptr_modifier", Cpp::MsUnsignedPtrModifier),
        ("do", Cpp::Do),
        ("parenthesized_declarator", Cpp::ParenthesizedDeclarator),
        ("__fastcall", Cpp::Fastcall),
        ("::", Cpp::COLONCOLON),
        ("%=", Cpp::PERCENTEQ),
        ("MOZ_NEEDS_MEMMOVABLE_TYPE", Cpp::MOZNEEDSMEMMOVABLETYPE),
        ("MOZ_NON_AUTOABLE", Cpp::MOZNONAUTOABLE),
        (
            "variadic_parameter_declaration",
            Cpp::VariadicParameterDeclaration,
        ),
        ("MOZ_FORMAT_PRINTF", Cpp::MOZFORMATPRINTF),
        ("using", Cpp::Using),
        ("MOZ_REQUIRED_BASE_METHOD", Cpp::MOZREQUIREDBASEMETHOD),
        ("type_identifier", Cpp::TypeIdentifier),
        ("cast_expression", Cpp::CastExpression),
        ("^", Cpp::CARET),
        ("null", Cpp::Null),
        ("MOZ_NEVER_INLINE_DEBUG", Cpp::MOZNEVERINLINEDEBUG),
        ("MOZ_COLD", Cpp::MOZCOLD),
        ("+=", Cpp::PLUSEQ),
        ("sizeof_expression", Cpp::SizeofExpression),
        (
            "function_definition_repeat1",
            Cpp::FunctionDefinitionRepeat1,
        ),
        ("storage_class_specifier", Cpp::StorageClassSpecifier),
        ("function_definition", Cpp::FunctionDefinition),
        ("continue", Cpp::Continue),
        (
            "MOZ_NO_SANITIZE_SIGNED_OVERFLOW",
            Cpp::MOZNOSANITIZESIGNEDOVERFLOW,
        ),
        ("virtual_function_specifier", Cpp::VirtualFunctionSpecifier),
        ("preproc_ifdef", Cpp::PreprocIfdef),
        ("volatile", Cpp::Volatile),
        ("unary_expression", Cpp::UnaryExpression),
        ("sizeof", Cpp::Sizeof),
        ("public", Cpp::Public),
        ("attribute", Cpp::Attribute),
        ("<", Cpp::LT),
        ("for", Cpp::For),
        ("char_literal", Cpp::CharLiteral),
        ("final", Cpp::Final),
        ("preproc_arg", Cpp::PreprocArg),
        (
            "field_initializer_list_repeat1",
            Cpp::FieldInitializerListRepeat1,
        ),
        ("binary_expression", Cpp::BinaryExpression),
        ("#if", Cpp::HASHif),
        ("string_literal_repeat1", Cpp::StringLiteralRepeat1),
        ("union", Cpp::Union),
        ("MOZ_HAVE_ANALYZER_NORETURN", Cpp::MOZHAVEANALYZERNORETURN),
        ("labeled_statement", Cpp::LabeledStatement),
        ("enum", Cpp::Enum),
        ("base_class_clause", Cpp::BaseClassClause),
        ("parameter_pack_expansion", Cpp::ParameterPackExpansion),
        ("delete_method_clause", Cpp::DeleteMethodClause),
        ("delete_expression", Cpp::DeleteExpression),
        ("mutable", Cpp::Mutable),
        ("initializer_pair", Cpp::InitializerPair),
        ("field_designator", Cpp::FieldDesignator),
        ("false", Cpp::False),
        ("function_declarator", Cpp::FunctionDeclarator),
        ("MOZ_MUST_USE", Cpp::MOZMUSTUSE),
        ("preproc_defined", Cpp::PreprocDefined),
        ("template", Cpp::Template),
        ("preproc_include", Cpp::PreprocInclude),
        ("switch", Cpp::Switch),
        ("-", Cpp::DASH),
        (
            "MOZ_NO_SANITIZE_UNSIGNED_OVERFLOW",
            Cpp::MOZNOSANITIZEUNSIGNEDOVERFLOW,
        ),
        ("signed", Cpp::Signed),
        ("do_statement", Cpp::DoStatement),
        ("MOZ_HAVE_NORETURN_PTR", Cpp::MOZHAVENORETURNPTR),
        ("field_identifier", Cpp::FieldIdentifier),
        ("ms_unaligned_ptr_modifier", Cpp::MsUnalignedPtrModifier),
        ("break", Cpp::Break),
        (
            "MOZ_HAVE_SIGNED_OVERFLOW_SANITIZE_ATTR",
            Cpp::MOZHAVESIGNEDOVERFLOWSANITIZEATTR,
        ),
        ("constexpr", Cpp::Constexpr),
        ("case_statement_repeat1", Cpp::CaseStatementRepeat1),
        ("MOZ_NONNULL_RETURN", Cpp::MOZNONNULLRETURN),
        (
            "MOZ_HAVE_UNSIGNED_OVERFLOW_SANITIZE_ATTR",
            Cpp::MOZHAVEUNSIGNEDOVERFLOWSANITIZEATTR,
        ),
        (".", Cpp::DOT),
        ("assignment_expression", Cpp::AssignmentExpression),
        ("pointer_expression", Cpp::PointerExpression),
        ("MOZ_NEEDS_NO_VTABLE_TYPE", Cpp::MOZNEEDSNOVTABLETYPE),
        ("|", Cpp::PIPE),
        ("_constructor_specifiers", Cpp::ConstructorSpecifiers),
        ("type_qualifier", Cpp::TypeQualifier),
        ("friend", Cpp::Friend),
        ("sized_type_specifier", Cpp::SizedTypeSpecifier),
        ("template_method", Cpp::TemplateMethod),
        ("string_literal", Cpp::StringLiteral),
        ("defined", Cpp::Defined),
        ("update_expression", Cpp::UpdateExpression),
        ("u\'", Cpp::USQUOTE),
        ("]]", Cpp::RBRACKRBRACK),
        ("_enum_base_clause", Cpp::EnumBaseClause),
        ("operator_cast", Cpp::OperatorCast),
        (
            "MOZ_NEEDS_MEMMOVABLE_MEMBERS",
            Cpp::MOZNEEDSMEMMOVABLEMEMBERS,
        ),
        (
            "abstract_pointer_declarator",
            Cpp::AbstractPointerDeclarator,
        ),
        ("auto", Cpp::Auto),
        ("typename", Cpp::Typename),
        ("L\\\"", Cpp::LDQUOTE),
        ("type_parameter_declaration", Cpp::TypeParameterDeclaration),
        ("==", Cpp::EQEQ),
        ("return_statement", Cpp::ReturnStatement),
        ("field_initializer", Cpp::FieldInitializer),
        ("#ifdef", Cpp::HASHifdef),
        ("base_class_clause_repeat1", Cpp::BaseClassClauseRepeat1),
        ("this", Cpp::This),
        ("_expression", Cpp::Expression),
        ("extern", Cpp::Extern),
        ("template_function", Cpp::TemplateFunction),
        ("+", Cpp::PLUS),
        ("linkage_specification", Cpp::LinkageSpecification),
        ("declaration", Cpp::Declaration),
        ("template_type", Cpp::TemplateType),
        ("MOZ_TSAN_BLACKLIST", Cpp::MOZTSANBLACKLIST),
        ("macro_statement", Cpp::MacroStatement),
        ("MOZ_NONHEAP_CLASS", Cpp::MOZNONHEAPCLASS),
        ("&&", Cpp::AMPAMP),
        ("try_statement", Cpp::TryStatement),
        ("trailing_return_type", Cpp::TrailingReturnType),
        ("string_literal_token1", Cpp::StringLiteralToken1),
        ("\'", Cpp::SQUOTE),
        ("MOZ_MUST_USE_TYPE", Cpp::MOZMUSTUSETYPE),
        ("return", Cpp::Return),
        ("%", Cpp::PERCENT),
        ("MOZ_XPCOM_ABI", Cpp::MOZXPCOMABI),
        ("namespace", Cpp::Namespace),
        ("MOZ_NON_OWNING_REF", Cpp::MOZNONOWNINGREF),
        ("primitive_type", Cpp::PrimitiveType),
        ("type_definition_repeat1", Cpp::TypeDefinitionRepeat1),
        ("protected", Cpp::Protected),
        (
            "preproc_argument_list_repeat1",
            Cpp::PreprocArgumentListRepeat1,
        ),
        ("\\\"", Cpp::DQUOTE),
        ("identifier", Cpp::Identifier),
        (
            "structured_binding_declarator",
            Cpp::StructuredBindingDeclarator,
        ),
        ("unsigned", Cpp::Unsigned),
        ("namespace_identifier", Cpp::NamespaceIdentifier),
        ("field_declaration_repeat1", Cpp::FieldDeclarationRepeat1),
        ("enumerator_list", Cpp::EnumeratorList),
        ("while", Cpp::While),
        ("MOZ_IS_CLASS_INIT", Cpp::MOZISCLASSINIT),
        ("const", Cpp::Const),
        ("*", Cpp::STAR),
        ("MOZ_HEAP_ALLOCATOR", Cpp::MOZHEAPALLOCATOR),
        (";", Cpp::SEMI),
        (":", Cpp::COLON),
        (
            "abstract_function_declarator_repeat1",
            Cpp::AbstractFunctionDeclaratorRepeat1,
        ),
        ("preproc_params_repeat1", Cpp::PreprocParamsRepeat1),
        ("MOZ_STATIC_LOCAL_CLASS", Cpp::MOZSTATICLOCALCLASS),
        (
            "_field_declaration_list_item",
            Cpp::FieldDeclarationListItem,
        ),
        ("union_specifier", Cpp::UnionSpecifier),
        ("_field_declarator", Cpp::FieldDeclarator),
        (
            "abstract_function_declarator",
            Cpp::AbstractFunctionDeclarator,
        ),
        ("__declspec", Cpp::Declspec),
        ("class", Cpp::Class),
        (
            "preproc_if_in_field_declaration_list_repeat1",
            Cpp::PreprocIfInFieldDeclarationListRepeat1,
        ),
        ("#else", Cpp::HASHelse),
        ("initializer_pair_repeat1", Cpp::InitializerPairRepeat1),
        (
            "MOZ_PRETEND_NORETURN_FOR_STATIC_ANALYSIS",
            Cpp::MOZPRETENDNORETURNFORSTATICANALYSIS,
        ),
        ("pointer_declarator_repeat1", Cpp::PointerDeclaratorRepeat1),
        (
            "MOZ_MAY_CALL_AFTER_MUST_RETURN",
            Cpp::MOZMAYCALLAFTERMUSTRETURN,
        ),
        ("]", Cpp::RBRACK),
        ("initializer_list", Cpp::InitializerList),
        ("virtual", Cpp::Virtual),
        ("system_lib_string", Cpp::SystemLibString),
        ("field_declaration_list", Cpp::FieldDeclarationList),
        ("<<", Cpp::LTLT),
        (",", Cpp::COMMA),
        ("__thiscall", Cpp::Thiscall),
        ("noexcept", Cpp::Noexcept),
        ("/", Cpp::SLASH),
        ("ms_declspec_modifier", Cpp::MsDeclspecModifier),
        ("_unaligned", Cpp::Unaligned),
        ("declaration_repeat1", Cpp::DeclarationRepeat1),
        ("for_statement", Cpp::ForStatement),
        ("static_assert_declaration", Cpp::StaticAssertDeclaration),
        (
            "template_argument_list_repeat1",
            Cpp::TemplateArgumentListRepeat1,
        ),
        ("translation_unit", Cpp::TranslationUnit),
        ("catch", Cpp::Catch),
        ("ms_call_modifier", Cpp::MsCallModifier),
        ("MOZ_STATIC_CLASS", Cpp::MOZSTATICCLASS),
        ("operator_name", Cpp::OperatorName),
        ("parameter_declaration", Cpp::ParameterDeclaration),
        ("throw_specifier", Cpp::ThrowSpecifier),
        ("MOZ_TRIVIAL_CTOR_DTOR", Cpp::MOZTRIVIALCTORDTOR),
        (
            "structured_binding_declarator_repeat1",
            Cpp::StructuredBindingDeclaratorRepeat1,
        ),
        ("->", Cpp::DASHGT),
        ("preproc_function_def", Cpp::PreprocFunctionDef),
        ("{", Cpp::LBRACE),
        ("enum_specifier", Cpp::EnumSpecifier),
        ("throw_statement", Cpp::ThrowStatement),
        ("MOZ_RAII", Cpp::MOZRAII),
        ("raw_string_literal", Cpp::RawStringLiteral),
        ("else", Cpp::Else),
        ("expression_statement", Cpp::ExpressionStatement),
        ("preproc_call", Cpp::PreprocCall),
        ("/=", Cpp::SLASHEQ),
        ("private", Cpp::Private),
        ("init_declarator", Cpp::InitDeclarator),
        ("field_expression", Cpp::FieldExpression),
        (
            "MOZ_ONLY_USED_TO_AVOID_STATIC_CONSTRUCTORS",
            Cpp::MOZONLYUSEDTOAVOIDSTATICCONSTRUCTORS,
        ),
        ("--", Cpp::DASHDASH),
        ("try_statement_repeat1", Cpp::TryStatementRepeat1),
        (
            "MOZ_MUST_RETURN_FROM_CALLER_IF_THIS_IS_ARG",
            Cpp::MOZMUSTRETURNFROMCALLERIFTHISISARG,
        ),
        ("scoped_type_identifier", Cpp::ScopedTypeIdentifier),
        (">=", Cpp::GTEQ),
        ("lambda_capture_specifier", Cpp::LambdaCaptureSpecifier),
        ("pointer_declarator", Cpp::PointerDeclarator),
        ("nullptr", Cpp::Nullptr),
        ("variadic_declarator", Cpp::VariadicDeclarator),
        ("u\\\"", Cpp::UDQUOTE),
        ("||", Cpp::PIPEPIPE),
        ("MOZ_HAVE_ASAN_BLACKLIST", Cpp::MOZHAVEASANBLACKLIST),
        ("override", Cpp::Override),
        ("__clrcall", Cpp::Clrcall),
        ("throw", Cpp::Throw),
        (
            "optional_parameter_declaration",
            Cpp::OptionalParameterDeclaration,
        ),
        ("dependent_type", Cpp::DependentType),
        ("escape_sequence", Cpp::EscapeSequence),
        (
            "function_declarator_repeat2",
            Cpp::FunctionDeclaratorRepeat2,
        ),
        ("_class_name", Cpp::ClassName),
        ("MOZ_NON_PARAM", Cpp::MOZNONPARAM),
        ("short", Cpp::Short),
        ("static", Cpp::Static),
        ("<<=", Cpp::LTLTEQ),
        ("alone_macro", Cpp::AloneMacro),
        ("<=", Cpp::LTEQ),
        ("#include", Cpp::HASHinclude),
        ("MOZ_MUST_OVERRIDE", Cpp::MOZMUSTOVERRIDE),
        ("!=", Cpp::BANGEQ),
        (
            "template_template_parameter_declaration",
            Cpp::TemplateTemplateParameterDeclaration,
        ),
        ("delete", Cpp::Delete),
        ("using_declaration", Cpp::UsingDeclaration),
        ("struct", Cpp::Struct),
        (
            "abstract_parenthesized_declarator",
            Cpp::AbstractParenthesizedDeclarator,
        ),
        ("ERROR", Cpp::Error),
        ("lambda_expression", Cpp::LambdaExpression),
        ("new", Cpp::New),
        ("attribute_repeat1", Cpp::AttributeRepeat1),
        ("subscript_expression", Cpp::SubscriptExpression),
        ("declaration_list", Cpp::DeclarationList),
        ("inline", Cpp::Inline),
        ("MOZ_STACK_CLASS", Cpp::MOZSTACKCLASS),
        ("subscript_designator", Cpp::SubscriptDesignator),
        ("ms_signed_ptr_modifier", Cpp::MsSignedPtrModifier),
        ("MOZ_ALLOCATOR", Cpp::MOZALLOCATOR),
        ("enumerator", Cpp::Enumerator),
        ("field_declaration", Cpp::FieldDeclaration),
        ("goto", Cpp::Goto),
        ("lambda_default_capture", Cpp::LambdaDefaultCapture),
        ("#ifndef", Cpp::HASHifndef),
        ("macro_annotation", Cpp::MacroAnnotation),
        ("concatenated_string", Cpp::ConcatenatedString),
        ("_declarator", Cpp::Declarator),
        ("&=", Cpp::AMPEQ),
        ("scoped_identifier", Cpp::ScopedIdentifier),
        ("_Atomic", Cpp::Atomic),
        ("__vectorcall", Cpp::Vectorcall),
        (
            "MOZ_NO_ADDREF_RELEASE_ON_RETURN",
            Cpp::MOZNOADDREFRELEASEONRETURN,
        ),
        ("array_declarator", Cpp::ArrayDeclarator),
        ("enumerator_list_repeat1", Cpp::EnumeratorListRepeat1),
        ("}", Cpp::RBRACE),
        ("condition_clause", Cpp::ConditionClause),
        ("abstract_array_declarator", Cpp::AbstractArrayDeclarator),
        ("default", Cpp::Default),
        (
            "sized_type_specifier_repeat1",
            Cpp::SizedTypeSpecifierRepeat1,
        ),
        (
            "template_parameter_list_repeat1",
            Cpp::TemplateParameterListRepeat1,
        ),
        ("for_range_loop", Cpp::ForRangeLoop),
        ("alias_declaration", Cpp::AliasDeclaration),
        ("compound_statement", Cpp::CompoundStatement),
        ("virtual_specifier", Cpp::VirtualSpecifier),
        (
            "MOZ_CAN_RUN_SCRIPT_FOR_DEFINITION",
            Cpp::MOZCANRUNSCRIPTFORDEFINITION,
        ),
        ("|=", Cpp::PIPEEQ),
        ("_abstract_declarator", Cpp::AbstractDeclarator),
        ("MOZ_HAVE_NO_SANITIZE_ATTR", Cpp::MOZHAVENOSANITIZEATTR),
        ("static_assert", Cpp::StaticAssert),
        ("goto_statement", Cpp::GotoStatement),
        ("MOZ_INIT_OUTSIDE_CTOR", Cpp::MOZINITOUTSIDECTOR),
        ("u8\'", Cpp::U8SQUOTE),
        ("MOZ_TEMPORARY_CLASS", Cpp::MOZTEMPORARYCLASS),
        ("parameter_list_repeat1", Cpp::ParameterListRepeat1),
        ("attribute_specifier", Cpp::AttributeSpecifier),
        ("MOZ_MAYBE_UNUSED", Cpp::MOZMAYBEUNUSED),
        ("alone_macro_call_repeat1", Cpp::AloneMacroCallRepeat1),
        ("preproc_if", Cpp::PreprocIf),
        ("...", Cpp::DOTDOTDOT),
        ("__stdcall", Cpp::Stdcall),
        ("break_statement", Cpp::BreakStatement),
        ("MOZ_CAN_RUN_SCRIPT", Cpp::MOZCANRUNSCRIPT),
        ("!", Cpp::BANG),
        ("_empty_declaration", Cpp::EmptyDeclaration),
        ("if_statement", Cpp::IfStatement),
        ("field_initializer_list", Cpp::FieldInitializerList),
        ("alone_macro_call_token1", Cpp::AloneMacroCallToken1),
        ("MOZ_HAVE_NEVER_INLINE", Cpp::MOZHAVENEVERINLINE),
        ("conditional_expression", Cpp::ConditionalExpression),
        ("explicit", Cpp::Explicit),
        ("ms_pointer_modifier", Cpp::MsPointerModifier),
        ("throw_specifier_repeat1", Cpp::ThrowSpecifierRepeat1),
        ("alone_macro_call", Cpp::AloneMacroCall),
        ("#define", Cpp::HASHdefine),
        ("catch_clause", Cpp::CatchClause),
        ("ms_restrict_modifier", Cpp::MsRestrictModifier),
        ("=", Cpp::EQ),
        ("MOZ_HAVE_NORETURN", Cpp::MOZHAVENORETURN),
        ("statement_identifier", Cpp::StatementIdentifier),
        ("new_expression", Cpp::NewExpression),
        ("while_statement", Cpp::WhileStatement),
        ("new_declarator", Cpp::NewDeclarator),
        ("MOZ_NON_MEMMOVABLE", Cpp::MOZNONMEMMOVABLE),
        ("operator", Cpp::Operator),
        ("#endif", Cpp::HASHendif),
        ("template_argument_list", Cpp::TemplateArgumentList),
        ("~", Cpp::TILDE),
        ("template_parameter_list", Cpp::TemplateParameterList),
        ("bitfield_clause", Cpp::BitfieldClause),
        ("MOZ_ASAN_BLACKLIST", Cpp::MOZASANBLACKLIST),
        ("comma_expression", Cpp::CommaExpression),
        ("friend_declaration", Cpp::FriendDeclaration),
        ("*=", Cpp::STAREQ),
        ("if", Cpp::If),
        ("argument_list", Cpp::ArgumentList),
        ("parameter_list", Cpp::ParameterList),
        ("u8\\\"", Cpp::U8DQUOTE),
        ("?", Cpp::QMARK),
        ("number_literal", Cpp::NumberLiteral),
        ("class_specifier", Cpp::ClassSpecifier),
        (
            "concatenated_string_repeat1",
            Cpp::ConcatenatedStringRepeat1,
        ),
        ("try", Cpp::Try),
        (">", Cpp::GT),
        ("&", Cpp::AMP),
        ("MOZ_NORETURN", Cpp::MOZNORETURN),
        (
            "MOZ_NO_ARITHMETIC_EXPR_IN_ARGUMENT",
            Cpp::MOZNOARITHMETICEXPRINARGUMENT,
        ),
        ("[[", Cpp::LBRACKLBRACK),
        ("^=", Cpp::CARETEQ),
        ("template_declaration", Cpp::TemplateDeclaration),
        ("initializer_list_repeat1", Cpp::InitializerListRepeat1),
        ("type_definition", Cpp::TypeDefinition),
        ("restrict", Cpp::Restrict),
        (">>=", Cpp::GTGTEQ),
        ("MOZ_NON_TEMPORARY_CLASS", Cpp::MOZNONTEMPORARYCLASS),
        (
            "MOZ_IS_SMARTPTR_TO_REFCOUNTED",
            Cpp::MOZISSMARTPTRTOREFCOUNTED,
        ),
        ("MOZ_IMPLICIT", Cpp::MOZIMPLICIT),
        ("argument_list_repeat1", Cpp::ArgumentListRepeat1),
        (
            "MOZ_DECL_USE_GUARD_OBJECT_NOTIFIER",
            Cpp::MOZDECLUSEGUARDOBJECTNOTIFIER,
        ),
        ("MOZ_ALLOW_TEMPORARY", Cpp::MOZALLOWTEMPORARY),
        ("typedef", Cpp::Typedef),
        ("access_specifier", Cpp::AccessSpecifier),
        ("translation_unit_repeat1", Cpp::TranslationUnitRepeat1),
        ("case", Cpp::Case),
        ("default_method_clause", Cpp::DefaultMethodClause),
        (
            "optional_type_parameter_declaration",
            Cpp::OptionalTypeParameterDeclaration,
        ),
        ("MOZ_UNSAFE_REF", Cpp::MOZUNSAFEREF),
        ("__based", Cpp::Based),
        (
            "MOZ_PUSH_DISABLE_NONTRIVIAL_UNION_WARNINGS",
            Cpp::MOZPUSHDISABLENONTRIVIALUNIONWARNINGS,
        ),
        ("MOZ_OWNING_REF", Cpp::MOZOWNINGREF),
        ("preproc_else", Cpp::PreprocElse),
        ("true", Cpp::True),
        ("MOZ_NONNULL", Cpp::MOZNONNULL),
        (")", Cpp::RPAREN),
        ("MOZ_ALWAYS_INLINE", Cpp::MOZALWAYSINLINE),
        (
            "function_declarator_repeat1",
            Cpp::FunctionDeclaratorRepeat1,
        ),
        ("preproc_params", Cpp::PreprocParams),
        (
            "MOZ_POP_DISABLE_NONTRIVIAL_UNION_WARNINGS",
            Cpp::MOZPOPDISABLENONTRIVIALUNIONWARNINGS,
        ),
        ("_declaration_specifiers", Cpp::DeclarationSpecifiers),
        ("destructor_name", Cpp::DestructorName),
        ("(", Cpp::LPAREN),
        ("scoped_field_identifier", Cpp::ScopedFieldIdentifier),
        ("decltype", Cpp::Decltype),
        ("++", Cpp::PLUSPLUS),
    ]),
};

impl From<&str> for Cpp {
    #[inline(always)]
    fn from(key: &str) -> Self {
        KEYS.get(key).unwrap().clone()
    }
}

impl From<u16> for Cpp {
    #[inline(always)]
    fn from(x: u16) -> Self {
        num::FromPrimitive::from_u16(x).unwrap_or(Self::Error)
    }
}

// Cpp == u16
impl PartialEq<u16> for Cpp {
    #[inline(always)]
    fn eq(&self, x: &u16) -> bool {
        *self == Cpp::from(*x)
    }
}

// u16 == Cpp
impl PartialEq<Cpp> for u16 {
    #[inline(always)]
    fn eq(&self, x: &Cpp) -> bool {
        *x == *self
    }
}
