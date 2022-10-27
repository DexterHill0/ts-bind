Generics {
    lt_token: Some(
        Lt,
    ),
    params: [
        Type(
            TypeParam {
                attrs: [],
                ident: Ident {
                    ident: "T",
                    span: #0 bytes(91..92),
                },
                colon_token: Some(
                    Colon,
                ),
                bounds: [
                    Trait(
                        TraitBound {
                            paren_token: None,
                            modifier: None,
                            lifetimes: None,
                            path: Path {
                                leading_colon: None,
                                segments: [
                                    PathSegment {
                                        ident: Ident {
                                            ident: "Trait",
                                            span: #0 bytes(94..99),
                                        },
                                        arguments: None,
                                    },
                                ],
                            },
                        },
                    ),
                ],
                eq_token: Some(
                    Eq,
                ),
                default: Some(
                    Path(
                        TypePath {
                            qself: None,
                            path: Path {
                                leading_colon: None,
                                segments: [
                                    PathSegment {
                                        ident: Ident {
                                            ident: "Foo",
                                            span: #0 bytes(102..105),
                                        },
                                        arguments: None,
                                    },
                                ],
                            },
                        },
                    ),
                ),
            },
        ),
    ],
    gt_token: Some(
        Gt,
    ),
    where_clause: None,
}





Path {
    leading_colon: None,
    segments: [
        PathSegment {
            ident: Ident {
                ident: "Trait",
                span: #0 bytes(110..115),
            },
            arguments: AngleBracketed(
                AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Lt,
                    args: [
                        Type(
                            Path(
                                TypePath {
                                    qself: None,
                                    path: Path {
                                        leading_colon: None,
                                        segments: [
                                            PathSegment {
                                                ident: Ident {
                                                    ident: "T",
                                                    span: #0 bytes(116..117),
                                                },
                                                arguments: None,
                                            },
                                        ],
                                    },
                                },
                            ),
                        ),
                        Comma,
                        Type(
                            Path(
                                TypePath {
                                    qself: None,
                                    path: Path {
                                        leading_colon: None,
                                        segments: [
                                            PathSegment {
                                                ident: Ident {
                                                    ident: "Vec",
                                                    span: #0 bytes(119..122),
                                                },
                                                arguments: AngleBracketed(
                                                    AngleBracketedGenericArguments {
                                                        colon2_token: None,
                                                        lt_token: Lt,
                                                        args: [
                                                            Type(
                                                                Path(
                                                                    TypePath {
                                                                        qself: None,
                                                                        path: Path {
                                                                            leading_colon: None,
                                                                            segments: [
                                                                                PathSegment {
                                                                                    ident: Ident {
                                                                                        ident: "T",
                                                                                        span: #0 bytes(123..124),
                                                                                    },
                                                                                    arguments: None,
                                                                                },
                                                                            ],
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        gt_token: Gt,
                                                    },
                                                ),
                                            },
                                        ],
                                    },
                                },
                            ),
                        ),
                    ],
                    gt_token: Gt,
                },
            ),
        },
    ],
}