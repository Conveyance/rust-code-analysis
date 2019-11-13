// Code generated; DO NOT EDIT.

#[derive(Clone, Debug, PartialEq)]
pub enum Python {
    End = 0,
    Identifier = 1,
    Import = 2,
    DOT = 3,
    From = 4,
    Future = 5,
    LPAREN = 6,
    RPAREN = 7,
    COMMA = 8,
    As = 9,
    STAR = 10,
    Print = 11,
    GTGT = 12,
    Assert = 13,
    COLONEQ = 14,
    Return = 15,
    Del = 16,
    Raise = 17,
    Pass = 18,
    Break = 19,
    Continue = 20,
    If = 21,
    COLON = 22,
    Elif = 23,
    Else = 24,
    Async = 25,
    For = 26,
    In = 27,
    While = 28,
    Try = 29,
    Except = 30,
    Finally = 31,
    With = 32,
    Def = 33,
    DASHGT = 34,
    EQ = 35,
    Global = 36,
    Nonlocal = 37,
    Exec = 38,
    Class = 39,
    AT = 40,
    Not = 41,
    And = 42,
    Or = 43,
    PLUS = 44,
    DASH = 45,
    SLASH = 46,
    PERCENT = 47,
    SLASHSLASH = 48,
    STARSTAR = 49,
    PIPE = 50,
    AMP = 51,
    CARET = 52,
    LTLT = 53,
    TILDE = 54,
    LT = 55,
    LTEQ = 56,
    EQEQ = 57,
    BANGEQ = 58,
    GTEQ = 59,
    GT = 60,
    LTGT = 61,
    Is = 62,
    Lambda = 63,
    PLUSEQ = 64,
    DASHEQ = 65,
    STAREQ = 66,
    SLASHEQ = 67,
    ATEQ = 68,
    SLASHSLASHEQ = 69,
    PERCENTEQ = 70,
    STARSTAREQ = 71,
    GTGTEQ = 72,
    LTLTEQ = 73,
    AMPEQ = 74,
    CARETEQ = 75,
    PIPEEQ = 76,
    Yield = 77,
    LBRACK = 78,
    RBRACK = 79,
    Ellipsis = 80,
    LBRACE = 81,
    RBRACE = 82,
    EscapeSequence = 83,
    NotEscapeSequence = 84,
    FormatSpecifierToken1 = 85,
    TypeConversion = 86,
    Integer = 87,
    Float = 88,
    True = 89,
    False = 90,
    None = 91,
    Await = 92,
    Comment = 93,
    Semicolon = 94,
    Newline = 95,
    Indent = 96,
    Dedent = 97,
    DQUOTE = 98,
    StringContent = 99,
    DQUOTE2 = 100,
    Module = 101,
    Statement = 102,
    SimpleStatements = 103,
    ImportStatement = 104,
    ImportPrefix = 105,
    RelativeImport = 106,
    FutureImportStatement = 107,
    ImportFromStatement = 108,
    ImportList = 109,
    AliasedImport = 110,
    WildcardImport = 111,
    PrintStatement = 112,
    Chevron = 113,
    AssertStatement = 114,
    ExpressionStatement = 115,
    NamedExpression = 116,
    ReturnStatement = 117,
    DeleteStatement = 118,
    RaiseStatement = 119,
    PassStatement = 120,
    BreakStatement = 121,
    ContinueStatement = 122,
    IfStatement = 123,
    ElifClause = 124,
    ElseClause = 125,
    ForStatement = 126,
    WhileStatement = 127,
    TryStatement = 128,
    ExceptClause = 129,
    FinallyClause = 130,
    WithStatement = 131,
    WithItem = 132,
    FunctionDefinition = 133,
    Parameters = 134,
    LambdaParameters = 135,
    Parameters2 = 136,
    DefaultParameter = 137,
    TypedDefaultParameter = 138,
    ListSplat = 139,
    DictionarySplat = 140,
    GlobalStatement = 141,
    NonlocalStatement = 142,
    ExecStatement = 143,
    ClassDefinition = 144,
    ArgumentList = 145,
    DecoratedDefinition = 146,
    Decorator = 147,
    Block = 148,
    Variables = 149,
    ExpressionList = 150,
    DottedName = 151,
    ExpressionWithinForInClause = 152,
    Expression = 153,
    PrimaryExpression = 154,
    NotOperator = 155,
    BooleanOperator = 156,
    BinaryOperator = 157,
    UnaryOperator = 158,
    ComparisonOperator = 159,
    Lambda2 = 160,
    Lambda3 = 161,
    Assignment = 162,
    AugmentedAssignment = 163,
    RightHandSide = 164,
    Yield2 = 165,
    Attribute = 166,
    Subscript = 167,
    Slice = 168,
    Call = 169,
    TypedParameter = 170,
    Type = 171,
    KeywordArgument = 172,
    List = 173,
    ComprehensionClauses = 174,
    ListComprehension = 175,
    Dictionary = 176,
    DictionaryComprehension = 177,
    Pair = 178,
    Set = 179,
    SetComprehension = 180,
    ParenthesizedExpression = 181,
    Tuple = 182,
    GeneratorExpression = 183,
    ForInClause = 184,
    IfClause = 185,
    ConditionalExpression = 186,
    ConcatenatedString = 187,
    String = 188,
    Interpolation = 189,
    FormatSpecifier = 190,
    FormatExpression = 191,
    Await2 = 192,
    ModuleRepeat1 = 193,
    SimpleStatementsRepeat1 = 194,
    ImportPrefixRepeat1 = 195,
    ImportListRepeat1 = 196,
    PrintStatementRepeat1 = 197,
    AssertStatementRepeat1 = 198,
    IfStatementRepeat1 = 199,
    TryStatementRepeat1 = 200,
    WithStatementRepeat1 = 201,
    ParametersRepeat1 = 202,
    GlobalStatementRepeat1 = 203,
    ArgumentListRepeat1 = 204,
    DecoratedDefinitionRepeat1 = 205,
    VariablesRepeat1 = 206,
    DottedNameRepeat1 = 207,
    ComparisonOperatorRepeat1 = 208,
    SubscriptRepeat1 = 209,
    ListRepeat1 = 210,
    ComprehensionClausesRepeat1 = 211,
    DictionaryRepeat1 = 212,
    TupleRepeat1 = 213,
    ForInClauseRepeat1 = 214,
    ConcatenatedStringRepeat1 = 215,
    StringRepeat1 = 216,
    FormatSpecifierRepeat1 = 217,
    Error = 218,
}

impl Into<&'static str> for Python {
    fn into(self) -> &'static str {
        match self {
            Python::End => "end",
            Python::Identifier => "identifier",
            Python::Import => "import",
            Python::DOT => ".",
            Python::From => "from",
            Python::Future => "__future__",
            Python::LPAREN => "(",
            Python::RPAREN => ")",
            Python::COMMA => ",",
            Python::As => "as",
            Python::STAR => "*",
            Python::Print => "print",
            Python::GTGT => ">>",
            Python::Assert => "assert",
            Python::COLONEQ => ":=",
            Python::Return => "return",
            Python::Del => "del",
            Python::Raise => "raise",
            Python::Pass => "pass",
            Python::Break => "break",
            Python::Continue => "continue",
            Python::If => "if",
            Python::COLON => ":",
            Python::Elif => "elif",
            Python::Else => "else",
            Python::Async => "async",
            Python::For => "for",
            Python::In => "in",
            Python::While => "while",
            Python::Try => "try",
            Python::Except => "except",
            Python::Finally => "finally",
            Python::With => "with",
            Python::Def => "def",
            Python::DASHGT => "->",
            Python::EQ => "=",
            Python::Global => "global",
            Python::Nonlocal => "nonlocal",
            Python::Exec => "exec",
            Python::Class => "class",
            Python::AT => "@",
            Python::Not => "not",
            Python::And => "and",
            Python::Or => "or",
            Python::PLUS => "+",
            Python::DASH => "-",
            Python::SLASH => "/",
            Python::PERCENT => "%",
            Python::SLASHSLASH => "//",
            Python::STARSTAR => "**",
            Python::PIPE => "|",
            Python::AMP => "&",
            Python::CARET => "^",
            Python::LTLT => "<<",
            Python::TILDE => "~",
            Python::LT => "<",
            Python::LTEQ => "<=",
            Python::EQEQ => "==",
            Python::BANGEQ => "!=",
            Python::GTEQ => ">=",
            Python::GT => ">",
            Python::LTGT => "<>",
            Python::Is => "is",
            Python::Lambda => "lambda",
            Python::PLUSEQ => "+=",
            Python::DASHEQ => "-=",
            Python::STAREQ => "*=",
            Python::SLASHEQ => "/=",
            Python::ATEQ => "@=",
            Python::SLASHSLASHEQ => "//=",
            Python::PERCENTEQ => "%=",
            Python::STARSTAREQ => "**=",
            Python::GTGTEQ => ">>=",
            Python::LTLTEQ => "<<=",
            Python::AMPEQ => "&=",
            Python::CARETEQ => "^=",
            Python::PIPEEQ => "|=",
            Python::Yield => "yield",
            Python::LBRACK => "[",
            Python::RBRACK => "]",
            Python::Ellipsis => "ellipsis",
            Python::LBRACE => "{",
            Python::RBRACE => "}",
            Python::EscapeSequence => "escape_sequence",
            Python::NotEscapeSequence => "_not_escape_sequence",
            Python::FormatSpecifierToken1 => "format_specifier_token1",
            Python::TypeConversion => "type_conversion",
            Python::Integer => "integer",
            Python::Float => "float",
            Python::True => "true",
            Python::False => "false",
            Python::None => "none",
            Python::Await => "await",
            Python::Comment => "comment",
            Python::Semicolon => "_semicolon",
            Python::Newline => "_newline",
            Python::Indent => "_indent",
            Python::Dedent => "_dedent",
            Python::DQUOTE => "\"",
            Python::StringContent => "_string_content",
            Python::DQUOTE2 => "\"",
            Python::Module => "module",
            Python::Statement => "_statement",
            Python::SimpleStatements => "_simple_statements",
            Python::ImportStatement => "import_statement",
            Python::ImportPrefix => "import_prefix",
            Python::RelativeImport => "relative_import",
            Python::FutureImportStatement => "future_import_statement",
            Python::ImportFromStatement => "import_from_statement",
            Python::ImportList => "_import_list",
            Python::AliasedImport => "aliased_import",
            Python::WildcardImport => "wildcard_import",
            Python::PrintStatement => "print_statement",
            Python::Chevron => "chevron",
            Python::AssertStatement => "assert_statement",
            Python::ExpressionStatement => "expression_statement",
            Python::NamedExpression => "named_expression",
            Python::ReturnStatement => "return_statement",
            Python::DeleteStatement => "delete_statement",
            Python::RaiseStatement => "raise_statement",
            Python::PassStatement => "pass_statement",
            Python::BreakStatement => "break_statement",
            Python::ContinueStatement => "continue_statement",
            Python::IfStatement => "if_statement",
            Python::ElifClause => "elif_clause",
            Python::ElseClause => "else_clause",
            Python::ForStatement => "for_statement",
            Python::WhileStatement => "while_statement",
            Python::TryStatement => "try_statement",
            Python::ExceptClause => "except_clause",
            Python::FinallyClause => "finally_clause",
            Python::WithStatement => "with_statement",
            Python::WithItem => "with_item",
            Python::FunctionDefinition => "function_definition",
            Python::Parameters => "parameters",
            Python::LambdaParameters => "lambda_parameters",
            Python::Parameters2 => "_parameters",
            Python::DefaultParameter => "default_parameter",
            Python::TypedDefaultParameter => "typed_default_parameter",
            Python::ListSplat => "list_splat",
            Python::DictionarySplat => "dictionary_splat",
            Python::GlobalStatement => "global_statement",
            Python::NonlocalStatement => "nonlocal_statement",
            Python::ExecStatement => "exec_statement",
            Python::ClassDefinition => "class_definition",
            Python::ArgumentList => "argument_list",
            Python::DecoratedDefinition => "decorated_definition",
            Python::Decorator => "decorator",
            Python::Block => "block",
            Python::Variables => "variables",
            Python::ExpressionList => "expression_list",
            Python::DottedName => "dotted_name",
            Python::ExpressionWithinForInClause => "_expression_within_for_in_clause",
            Python::Expression => "_expression",
            Python::PrimaryExpression => "_primary_expression",
            Python::NotOperator => "not_operator",
            Python::BooleanOperator => "boolean_operator",
            Python::BinaryOperator => "binary_operator",
            Python::UnaryOperator => "unary_operator",
            Python::ComparisonOperator => "comparison_operator",
            Python::Lambda2 => "lambda",
            Python::Lambda3 => "lambda",
            Python::Assignment => "assignment",
            Python::AugmentedAssignment => "augmented_assignment",
            Python::RightHandSide => "_right_hand_side",
            Python::Yield2 => "yield",
            Python::Attribute => "attribute",
            Python::Subscript => "subscript",
            Python::Slice => "slice",
            Python::Call => "call",
            Python::TypedParameter => "typed_parameter",
            Python::Type => "type",
            Python::KeywordArgument => "keyword_argument",
            Python::List => "list",
            Python::ComprehensionClauses => "_comprehension_clauses",
            Python::ListComprehension => "list_comprehension",
            Python::Dictionary => "dictionary",
            Python::DictionaryComprehension => "dictionary_comprehension",
            Python::Pair => "pair",
            Python::Set => "set",
            Python::SetComprehension => "set_comprehension",
            Python::ParenthesizedExpression => "parenthesized_expression",
            Python::Tuple => "tuple",
            Python::GeneratorExpression => "generator_expression",
            Python::ForInClause => "for_in_clause",
            Python::IfClause => "if_clause",
            Python::ConditionalExpression => "conditional_expression",
            Python::ConcatenatedString => "concatenated_string",
            Python::String => "string",
            Python::Interpolation => "interpolation",
            Python::FormatSpecifier => "format_specifier",
            Python::FormatExpression => "format_expression",
            Python::Await2 => "await",
            Python::ModuleRepeat1 => "module_repeat1",
            Python::SimpleStatementsRepeat1 => "_simple_statements_repeat1",
            Python::ImportPrefixRepeat1 => "import_prefix_repeat1",
            Python::ImportListRepeat1 => "_import_list_repeat1",
            Python::PrintStatementRepeat1 => "print_statement_repeat1",
            Python::AssertStatementRepeat1 => "assert_statement_repeat1",
            Python::IfStatementRepeat1 => "if_statement_repeat1",
            Python::TryStatementRepeat1 => "try_statement_repeat1",
            Python::WithStatementRepeat1 => "with_statement_repeat1",
            Python::ParametersRepeat1 => "_parameters_repeat1",
            Python::GlobalStatementRepeat1 => "global_statement_repeat1",
            Python::ArgumentListRepeat1 => "argument_list_repeat1",
            Python::DecoratedDefinitionRepeat1 => "decorated_definition_repeat1",
            Python::VariablesRepeat1 => "variables_repeat1",
            Python::DottedNameRepeat1 => "dotted_name_repeat1",
            Python::ComparisonOperatorRepeat1 => "comparison_operator_repeat1",
            Python::SubscriptRepeat1 => "subscript_repeat1",
            Python::ListRepeat1 => "list_repeat1",
            Python::ComprehensionClausesRepeat1 => "_comprehension_clauses_repeat1",
            Python::DictionaryRepeat1 => "dictionary_repeat1",
            Python::TupleRepeat1 => "tuple_repeat1",
            Python::ForInClauseRepeat1 => "for_in_clause_repeat1",
            Python::ConcatenatedStringRepeat1 => "concatenated_string_repeat1",
            Python::StringRepeat1 => "string_repeat1",
            Python::FormatSpecifierRepeat1 => "format_specifier_repeat1",
            Python::Error => "ERROR",
        }
    }
}

#[allow(clippy::unreadable_literal)]
static KEYS: phf::Map<&'static str, Python> = ::phf::Map {
    key: 3213172566270843353,
    disps: ::phf::Slice::Static(&[
        (0, 12),
        (1, 8),
        (0, 37),
        (0, 26),
        (0, 36),
        (1, 14),
        (1, 109),
        (0, 6),
        (0, 38),
        (0, 50),
        (1, 75),
        (0, 202),
        (0, 183),
        (0, 62),
        (12, 86),
        (7, 48),
        (0, 10),
        (0, 16),
        (13, 94),
        (10, 111),
        (0, 47),
        (0, 4),
        (3, 153),
        (0, 9),
        (0, 38),
        (0, 0),
        (0, 94),
        (0, 106),
        (1, 1),
        (0, 115),
        (1, 101),
        (0, 0),
        (0, 2),
        (0, 197),
        (0, 3),
        (6, 43),
        (0, 86),
        (14, 151),
        (0, 105),
        (2, 25),
        (132, 149),
        (0, 139),
        (0, 1),
    ]),
    entries: ::phf::Slice::Static(&[
        ("*", Python::STAR),
        ("import", Python::Import),
        ("call", Python::Call),
        ("_import_list", Python::ImportList),
        ("@=", Python::ATEQ),
        ("else", Python::Else),
        ("delete_statement", Python::DeleteStatement),
        ("tuple_repeat1", Python::TupleRepeat1),
        ("except", Python::Except),
        (":", Python::COLON),
        ("continue", Python::Continue),
        ("*=", Python::STAREQ),
        ("_semicolon", Python::Semicolon),
        ("lambda_parameters", Python::LambdaParameters),
        ("set_comprehension", Python::SetComprehension),
        ("if_statement", Python::IfStatement),
        (">>=", Python::GTGTEQ),
        ("import_prefix_repeat1", Python::ImportPrefixRepeat1),
        ("end", Python::End),
        ("comment", Python::Comment),
        ("try", Python::Try),
        ("\\\"", Python::DQUOTE),
        ("dictionary_repeat1", Python::DictionaryRepeat1),
        ("{", Python::LBRACE),
        ("is", Python::Is),
        ("print", Python::Print),
        ("argument_list", Python::ArgumentList),
        ("import_prefix", Python::ImportPrefix),
        ("set", Python::Set),
        (
            "comparison_operator_repeat1",
            Python::ComparisonOperatorRepeat1,
        ),
        ("format_specifier_repeat1", Python::FormatSpecifierRepeat1),
        ("raise", Python::Raise),
        ("and", Python::And),
        ("->", Python::DASHGT),
        ("await", Python::Await),
        ("_expression", Python::Expression),
        ("exec", Python::Exec),
        ("dotted_name", Python::DottedName),
        ("[", Python::LBRACK),
        ("ERROR", Python::Error),
        ("string_repeat1", Python::StringRepeat1),
        ("as", Python::As),
        ("finally", Python::Finally),
        ("print_statement_repeat1", Python::PrintStatementRepeat1),
        ("integer", Python::Integer),
        ("break_statement", Python::BreakStatement),
        ("(", Python::LPAREN),
        ("list_repeat1", Python::ListRepeat1),
        (">=", Python::GTEQ),
        ("==", Python::EQEQ),
        ("__future__", Python::Future),
        (")", Python::RPAREN),
        ("/=", Python::SLASHEQ),
        ("subscript_repeat1", Python::SubscriptRepeat1),
        ("@", Python::AT),
        (">", Python::GT),
        ("**=", Python::STARSTAREQ),
        ("_right_hand_side", Python::RightHandSide),
        ("tuple", Python::Tuple),
        ("_dedent", Python::Dedent),
        ("import_statement", Python::ImportStatement),
        ("dictionary_splat", Python::DictionarySplat),
        ("subscript", Python::Subscript),
        ("import_from_statement", Python::ImportFromStatement),
        ("decorator", Python::Decorator),
        ("elif_clause", Python::ElifClause),
        ("async", Python::Async),
        ("_string_content", Python::StringContent),
        ("<<=", Python::LTLTEQ),
        ("expression_list", Python::ExpressionList),
        ("_statement", Python::Statement),
        ("format_specifier", Python::FormatSpecifier),
        ("dictionary", Python::Dictionary),
        ("try_statement", Python::TryStatement),
        ("true", Python::True),
        ("print_statement", Python::PrintStatement),
        ("assert", Python::Assert),
        ("not", Python::Not),
        ("<>", Python::LTGT),
        ("_import_list_repeat1", Python::ImportListRepeat1),
        ("class", Python::Class),
        ("_comprehension_clauses", Python::ComprehensionClauses),
        ("for_in_clause_repeat1", Python::ForInClauseRepeat1),
        ("typed_parameter", Python::TypedParameter),
        ("nonlocal", Python::Nonlocal),
        ("named_expression", Python::NamedExpression),
        ("pair", Python::Pair),
        ("+", Python::PLUS),
        ("module", Python::Module),
        ("-=", Python::DASHEQ),
        ("raise_statement", Python::RaiseStatement),
        ("/", Python::SLASH),
        (":=", Python::COLONEQ),
        ("variables_repeat1", Python::VariablesRepeat1),
        ("future_import_statement", Python::FutureImportStatement),
        ("keyword_argument", Python::KeywordArgument),
        ("nonlocal_statement", Python::NonlocalStatement),
        ("escape_sequence", Python::EscapeSequence),
        ("_newline", Python::Newline),
        ("else_clause", Python::ElseClause),
        ("yield", Python::Yield),
        ("for_statement", Python::ForStatement),
        ("&", Python::AMP),
        (
            "_expression_within_for_in_clause",
            Python::ExpressionWithinForInClause,
        ),
        ("dictionary_comprehension", Python::DictionaryComprehension),
        ("|=", Python::PIPEEQ),
        ("^=", Python::CARETEQ),
        ("for", Python::For),
        ("assert_statement", Python::AssertStatement),
        ("assignment", Python::Assignment),
        ("conditional_expression", Python::ConditionalExpression),
        (
            "_comprehension_clauses_repeat1",
            Python::ComprehensionClausesRepeat1,
        ),
        ("boolean_operator", Python::BooleanOperator),
        ("generator_expression", Python::GeneratorExpression),
        ("try_statement_repeat1", Python::TryStatementRepeat1),
        ("except_clause", Python::ExceptClause),
        ("_not_escape_sequence", Python::NotEscapeSequence),
        ("block", Python::Block),
        ("from", Python::From),
        (">>", Python::GTGT),
        ("}", Python::RBRACE),
        ("^", Python::CARET),
        ("module_repeat1", Python::ModuleRepeat1),
        ("with", Python::With),
        ("return", Python::Return),
        ("//", Python::SLASHSLASH),
        ("<=", Python::LTEQ),
        ("if_clause", Python::IfClause),
        ("**", Python::STARSTAR),
        ("pass", Python::Pass),
        ("expression_statement", Python::ExpressionStatement),
        ("typed_default_parameter", Python::TypedDefaultParameter),
        ("variables", Python::Variables),
        ("binary_operator", Python::BinaryOperator),
        ("parenthesized_expression", Python::ParenthesizedExpression),
        ("slice", Python::Slice),
        ("false", Python::False),
        ("wildcard_import", Python::WildcardImport),
        (",", Python::COMMA),
        ("list_comprehension", Python::ListComprehension),
        ("finally_clause", Python::FinallyClause),
        ("<<", Python::LTLT),
        ("del", Python::Del),
        ("type_conversion", Python::TypeConversion),
        ("while_statement", Python::WhileStatement),
        ("string", Python::String),
        ("continue_statement", Python::ContinueStatement),
        ("_primary_expression", Python::PrimaryExpression),
        ("aliased_import", Python::AliasedImport),
        ("format_expression", Python::FormatExpression),
        ("_indent", Python::Indent),
        ("%", Python::PERCENT),
        ("default_parameter", Python::DefaultParameter),
        ("return_statement", Python::ReturnStatement),
        ("break", Python::Break),
        ("-", Python::DASH),
        ("for_in_clause", Python::ForInClause),
        ("~", Python::TILDE),
        (".", Python::DOT),
        ("none", Python::None),
        ("_simple_statements", Python::SimpleStatements),
        ("global_statement_repeat1", Python::GlobalStatementRepeat1),
        ("concatenated_string", Python::ConcatenatedString),
        ("list", Python::List),
        ("type", Python::Type),
        ("in", Python::In),
        ("relative_import", Python::RelativeImport),
        ("%=", Python::PERCENTEQ),
        ("with_item", Python::WithItem),
        ("decorated_definition", Python::DecoratedDefinition),
        ("function_definition", Python::FunctionDefinition),
        ("]", Python::RBRACK),
        ("pass_statement", Python::PassStatement),
        ("lambda", Python::Lambda),
        ("augmented_assignment", Python::AugmentedAssignment),
        ("float", Python::Float),
        ("ellipsis", Python::Ellipsis),
        (
            "decorated_definition_repeat1",
            Python::DecoratedDefinitionRepeat1,
        ),
        ("if_statement_repeat1", Python::IfStatementRepeat1),
        ("global", Python::Global),
        ("dotted_name_repeat1", Python::DottedNameRepeat1),
        ("_parameters_repeat1", Python::ParametersRepeat1),
        (
            "_simple_statements_repeat1",
            Python::SimpleStatementsRepeat1,
        ),
        ("or", Python::Or),
        ("class_definition", Python::ClassDefinition),
        ("comparison_operator", Python::ComparisonOperator),
        (
            "concatenated_string_repeat1",
            Python::ConcatenatedStringRepeat1,
        ),
        ("unary_operator", Python::UnaryOperator),
        ("chevron", Python::Chevron),
        ("=", Python::EQ),
        ("|", Python::PIPE),
        ("//=", Python::SLASHSLASHEQ),
        ("while", Python::While),
        ("with_statement", Python::WithStatement),
        ("list_splat", Python::ListSplat),
        ("+=", Python::PLUSEQ),
        ("assert_statement_repeat1", Python::AssertStatementRepeat1),
        ("exec_statement", Python::ExecStatement),
        ("format_specifier_token1", Python::FormatSpecifierToken1),
        ("!=", Python::BANGEQ),
        ("&=", Python::AMPEQ),
        ("parameters", Python::Parameters),
        ("def", Python::Def),
        ("argument_list_repeat1", Python::ArgumentListRepeat1),
        ("elif", Python::Elif),
        ("<", Python::LT),
        ("with_statement_repeat1", Python::WithStatementRepeat1),
        ("attribute", Python::Attribute),
        ("global_statement", Python::GlobalStatement),
        ("not_operator", Python::NotOperator),
        ("interpolation", Python::Interpolation),
        ("identifier", Python::Identifier),
        ("if", Python::If),
    ]),
};

impl From<&str> for Python {
    #[inline(always)]
    fn from(key: &str) -> Self {
        KEYS.get(key).unwrap().clone()
    }
}

impl From<u16> for Python {
    #[inline(always)]
    fn from(x: u16) -> Self {
        unsafe { std::mem::transmute(x as u8) }
    }
}

// Python == u16
impl PartialEq<u16> for Python {
    #[inline(always)]
    fn eq(&self, x: &u16) -> bool {
        *self == Python::from(*x)
    }
}

// u16 == Python
impl PartialEq<Python> for u16 {
    #[inline(always)]
    fn eq(&self, x: &Python) -> bool {
        *x == *self
    }
}
