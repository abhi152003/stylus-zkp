use core::fmt;
use std::fmt::Formatter;

#[derive(Copy,Clone)]
pub enum ReportCode {
    //Parse Errors
    UnclosedComment,
    FileOs,
    NoMainFoundInProject,
    MultipleMain, 
    MissingSemicolon,
    UnrecognizedInclude,
    UnrecognizedVersion,
    UnrecognizedPragma,
    ExpectedIdentifier,
    IncludeNotFound,
    IllegalExpression,
    MultiplePragma,
    NoCompilerVersionWarning, 
    CompilerVersionError,
    WrongTypesInAssignOperationOperatorSignal,
    WrongTypesInAssignOperationOperatorNoSignal,
    WrongTypesInAssignOperationTemplate,
    WrongTypesInAssignOperationBus,
    WrongTypesInAssignOperationExpression,
    WrongTypesInAssignOperationArrayTemplates,
    WrongTypesInAssignOperationArrayBuses,
    WrongTypesInAssignOperationDims(usize, usize),
    WrongNumberOfArguments(usize, usize),
    UndefinedFunction,
    UndefinedTemplate,
    UndefinedBus,
    UninitializedSymbolInExpression,
    UnableToTypeFunction,
    UnreachableConstraints,
    UnreachableTags,
    UnreachableSignals,
    UnknownIndex,
    UnknownDimension,
    UnknownTemplateAssignment,
    SameFunctionDeclaredTwice,
    SameTemplateDeclaredTwice,
    SameSymbolDeclaredTwice,
    StaticInfoWasOverwritten,
    SignalInLineInitialization,
    SignalOutsideOriginalScope,
    FunctionWrongNumberOfArguments,
    FunctionInconsistentTyping,
    FunctionPathWithoutReturn,
    FunctionReturnError,
    ForbiddenDeclarationInFunction,
    NonHomogeneousArray(usize, usize),
    NonBooleanCondition,
    NonCompatibleBranchTypes,
    NonEqualTypesInExpression,
    NonExistentSymbol,
    MainComponentWithTags,
    IllegalMainExpression,
    TemplateCallAsArgument,
    TemplateWrongNumberOfArguments,
    TemplateWithReturnStatement,
    TypeCantBeUseAsCondition,
    EmptyArrayInlineDeclaration,
    PrefixOperatorWithWrongTypes,
    ParallelOperatorWithWrongTypes,
    InfixOperatorWithWrongTypes,
    InvalidArgumentInCall,
    InvalidArgumentInBusInstantiationT,
    InvalidArgumentInBusInstantiationB,
    InconsistentReturnTypesInBlock,
    InconsistentStaticInformation,
    InvalidArrayAccess(usize, usize),
    InvalidSignalAccess,
    InvalidTagAccess,
    InvalidTagAccessAfterArray,
    InvalidArraySize(usize),
    InvalidArraySizeT,
    InvalidArraySizeB,
    InvalidArrayType,
    InvalidArrayTypeB,
    ForStatementIllConstructed,
    BadArrayAccess,
    AssigningAComponentTwice,
    AssigningASignalTwice,
    NotAllowedOperation,
    ConstraintGeneratorInFunction,
    WrongSignalTags,
    InvalidPartialArray,
    MustBeSingleArithmetic(usize),
    MustBeSingleArithmeticT,
    MustBeSingleArithmeticB,
    MustBeArithmetic,
    OutputTagCannotBeModifiedOutside,
    InputTagCannotBeAccessedOutside,
    InputTagCannotBeModifiedOutside,
    MustBeSameDimension(usize, usize),
    ExpectedDimDiffGotDim(usize, usize),
    RuntimeError,
    RuntimeWarning,
    UnknownTemplate,
    UnknownBus,
    NonQuadratic,
    NonValidTagAssignment,
    NonConstantArrayLength,
    NonComputableExpression,
    // Constraint analysis codes
    UnconstrainedSignal,
    UnconstrainedIOSignal,
    UnusedInput,
    UnusedOutput,

    ErrorWat2Wasm,
    CustomGateIntermediateSignalWarning,
    CustomGateConstraintError,
    CustomGateSubComponentError,
    CustomGatesPragmaError,
    CustomGatesVersionError,
    AnonymousCompError,
    UnderscoreWithNoSignalWarning,
    TupleError,
    InvalidSignalTagAccess,
    UninitializedComponent,
    BusWrongNumberOfArguments,
    InvalidSignalAccessInBus,
    MustBeSameBus,
    MustBeBus,
}

impl fmt::Display for ReportCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use self::ReportCode::*;
        let string_format = match self {
            NoMainFoundInProject => "P1001",
            MultipleMain => "P1002",
            CompilerVersionError => "P1003",
            NoCompilerVersionWarning => "P1004",
            UnclosedComment => "P1005",
            FileOs  => "P1006",
            MissingSemicolon => "P1008",
            UnrecognizedInclude => "P1009",
            UnrecognizedVersion => "P1010",
            UnrecognizedPragma => "P1011",
            IllegalExpression => "P1012",
            MultiplePragma => "P1013",
            IncludeNotFound => "P1014",
            ExpectedIdentifier => "P1015",
            UndefinedFunction => "T2001",
            UndefinedTemplate => "T2002",
            UninitializedSymbolInExpression => "T2003",
            UnableToTypeFunction => "T2004",
            UnreachableConstraints => "T2005",
            SameFunctionDeclaredTwice => "T2006",
            SameTemplateDeclaredTwice => "T2007",
            SameSymbolDeclaredTwice => "T2008",
            StaticInfoWasOverwritten => "T2009",
            SignalInLineInitialization => "T2010",
            SignalOutsideOriginalScope => "T2011",
            FunctionWrongNumberOfArguments => "T2012",
            FunctionInconsistentTyping => "T2013",
            FunctionPathWithoutReturn => "T2014",
            FunctionReturnError => "T2015",
            ForbiddenDeclarationInFunction => "T2016",
            NonHomogeneousArray(..) => "T2017",
            NonBooleanCondition => "T2018",
            NonCompatibleBranchTypes => "T2019",
            NonEqualTypesInExpression => "T2020",
            NonExistentSymbol => "T2021",
            TemplateCallAsArgument => "T2022",
            TemplateWrongNumberOfArguments => "T2023",
            TemplateWithReturnStatement => "T2024",
            TypeCantBeUseAsCondition => "T2025",
            EmptyArrayInlineDeclaration => "T2026",
            PrefixOperatorWithWrongTypes => "T2027",
            ParallelOperatorWithWrongTypes => "T2047",
            InfixOperatorWithWrongTypes => "T2028",
            InvalidArgumentInCall => "T2029",
            InconsistentReturnTypesInBlock => "T2030",
            InconsistentStaticInformation => "T2031",
            InvalidArrayAccess(..) => "T2032",
            InvalidSignalAccess => "T2046",
            InvalidSignalTagAccess => "T2047",
            InvalidTagAccess => "T2048",
            InvalidTagAccessAfterArray => "T2049",
            InvalidArraySize(..) => "T2033",
            InvalidArraySizeT => "T2033",
            InvalidArrayType => "T2034",
            InvalidArrayTypeB => "T2034",
            ForStatementIllConstructed => "T2035",
            BadArrayAccess => "T2035",
            AssigningAComponentTwice => "T2036",
            AssigningASignalTwice => "T2037",
            NotAllowedOperation => "T2038",
            ConstraintGeneratorInFunction => "T2039",
            WrongSignalTags => "T2040",
            UnknownIndex => "T2042",
            InvalidPartialArray => "T2043",
            MustBeSingleArithmetic(..) => "T2044",
            MustBeSingleArithmeticT => "T2044",
            MustBeSingleArithmeticB => "T2044",
            ExpectedDimDiffGotDim(..) => "T2045",
            MustBeSameDimension(..) => "T2046",
            MustBeArithmetic => "T2047",
            OutputTagCannotBeModifiedOutside => "T2048-A",
            InputTagCannotBeModifiedOutside => "T2048-B",
            InputTagCannotBeAccessedOutside => "T2048-C",
            UnreachableTags => "T2049",
            UnreachableSignals => "T2050",
            MainComponentWithTags => "T2051",
            UndefinedBus => "T2052",
            InvalidArraySizeB => "T2053",
            WrongTypesInAssignOperationOperatorSignal => "T2054",
            WrongTypesInAssignOperationOperatorNoSignal => "T2055",
            WrongTypesInAssignOperationArrayTemplates => "T2056",
            WrongTypesInAssignOperationTemplate => "T2057",
            WrongTypesInAssignOperationArrayBuses => "T2058",
            WrongTypesInAssignOperationBus => "T2059",
            WrongTypesInAssignOperationExpression => "T2060",
            WrongTypesInAssignOperationDims(..) => "T2061",
            NonValidTagAssignment => "T2062",
            IllegalMainExpression => "T2063",
            RuntimeError => "T3001",
            RuntimeWarning => "T3002",
            UnknownDimension => "T20460",
            UnknownTemplate => "T20461",
            UnknownTemplateAssignment => "T2O461-A",
            NonQuadratic => "T20462",
            NonConstantArrayLength => "T20463",
            NonComputableExpression => "T20464",
            WrongNumberOfArguments(..) => "T20465",
            UninitializedComponent => "T20466",
            UnknownBus => "T20467",
            // Constraint analysis codes
            UnconstrainedSignal => "CA01",
            UnconstrainedIOSignal => "CA02",
            UnusedInput => "CA03",
            UnusedOutput => "CA04",
            ErrorWat2Wasm => "W01",
            CustomGateIntermediateSignalWarning => "CG01",
            CustomGateConstraintError => "CG02",
            CustomGateSubComponentError => "CG03",
            CustomGatesPragmaError => "CG04",
            CustomGatesVersionError => "CG05",
            AnonymousCompError => "TAC01",
            TupleError => "TAC02",
            UnderscoreWithNoSignalWarning => "TAC03",
            BusWrongNumberOfArguments => "BU01",
            InvalidArgumentInBusInstantiationT => "BU02",
            InvalidArgumentInBusInstantiationB => "BU03",
            InvalidSignalAccessInBus => "BU04",
            MustBeSameBus => "BU05",
            MustBeBus => "BU06",
        };
        f.write_str(string_format)
    }
}