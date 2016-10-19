#[macro_export]
macro_rules! graphql_union {
    ( @as_item, $i:item) => { $i };
    ( @as_expr, $e:expr) => { $e };
    ( @as_path, $p:path) => { $p };
    ( @as_type, $t:ty) => { $t };

    // description: <description>
    (
        @ gather_meta,
        ($reg:expr, $acc:expr, $descr:expr),
        description : $value:tt $( $rest:tt )*
    ) => {
        $descr = Some(graphql_interface!(@as_expr, $value));

        graphql_union!(@ gather_meta, ($reg, $acc, $descr), $( $rest )*)
    };

    // Gathering meta for instance resolvers
    // instance_resolvers: | <ctxtvar> | [...]
    (
        @ gather_meta,
        ($reg:expr, $acc:expr, $descr:expr),
        instance_resolvers: | $ctxtvar:pat | { $( $srctype:ty => $resolver:expr ),* $(,)* } $( $rest:tt )*
    ) => {
        $acc = vec![
            $(
                $reg.get_type::<$srctype>()
            ),*
        ];

        graphql_union!(@ gather_meta, ($reg, $acc, $descr), $( $rest )*)
    };

    // To generate the "concrete type name" resolver, syntax case:
    // instance_resolvers: | <ctxtvar> | [...]
    (
        @ concrete_type_name,
        ($outname:tt, $ctxtarg:ident, $ctxttype:ty),
        instance_resolvers: | $ctxtvar:pat | { $( $srctype:ty => $resolver:expr ),* $(,)* } $( $rest:tt )*
    ) => {
        let $ctxtvar = &$ctxtarg;

        $(
            if let Some(_) = $resolver {
                return (<$srctype as $crate::GraphQLType<$ctxttype>>::name()).unwrap().to_owned();
            }
        )*

            panic!("Concrete type not handled by instance resolvers on {}", $outname);
    };

    // To generate the "resolve into type" resolver, syntax case:
    // instance_resolvers: | <ctxtvar> | [...]
    (
        @ resolve_into_type,
        ($outname:tt, $typenamearg:ident, $execarg:ident, $ctxttype:ty),
        instance_resolvers: | $ctxtvar:pat | { $( $srctype:ty => $resolver:expr ),* $(,)* } $( $rest:tt )*
    ) => {
        let $ctxtvar = &$execarg.context();

        $(
            if $typenamearg == (<$srctype as $crate::GraphQLType<$ctxttype>>::name()).unwrap().to_owned() {
                return $execarg.resolve(&$resolver);
            }
        )*

            panic!("Concrete type not handled by instance resolvers on {}", $outname);
    };

    // eat commas
    ( @ $mfn:ident, $args:tt, , $($rest:tt)* ) => {
        graphql_union!(@ $mfn, $args, $($rest)*);
    };

    // eat one tt
    ( @ $mfn:ident, $args:tt, $item:tt $($rest:tt)* ) => {
        graphql_union!(@ $mfn, $args, $($rest)*);
    };

    // end case
    ( @ $mfn:ident, $args:tt, ) => {};

    (
        ( $($lifetime:tt),* ) $name:ty : $ctxt:ty as $outname:tt | &$mainself:ident | {
            $( $items:tt )*
        }
    ) => {
        graphql_union!(@as_item, impl<$($lifetime)*> $crate::GraphQLType<$ctxt> for $name {
            fn name() -> Option<&'static str> {
                Some($outname)
            }

            #[allow(unused_assignments)]
            #[allow(unused_mut)]
            fn meta(registry: &mut $crate::Registry<$ctxt>) -> $crate::meta::MetaType {
                let mut types;
                let mut description = None;
                graphql_union!(@ gather_meta, (registry, types, description), $($items)*);
                let mut mt = registry.build_union_type::<$name>()(&types);

                if let Some(description) = description {
                    mt = mt.description(description);
                }

                mt.into_meta()
            }

            fn concrete_type_name(&$mainself, context: &$ctxt) -> String {
                graphql_union!(
                    @ concrete_type_name,
                    ($outname, context, $ctxt),
                    $($items)*);
            }

            fn resolve_into_type(
                &$mainself,
                type_name: &str,
                _: Option<Vec<$crate::Selection>>,
                executor: &mut $crate::Executor<$ctxt>,
            )
                -> $crate::ExecutionResult
            {
                graphql_union!(
                    @ resolve_into_type,
                    ($outname, type_name, executor, $ctxt),
                    $($items)*);
            }
        });

        impl<$($lifetime)*> $crate::IntoFieldResult<$name> for $name {
            fn into(self) -> $crate::FieldResult<$name> {
                Ok(self)
            }
        }
    };

    (
        <$($lifetime:tt),*> $name:ty : $ctxt:ty as $outname:tt | &$mainself:ident | {
            $( $items:tt )*
        }
    ) => {
        graphql_union!(
            ($($lifetime),*) $name : $ctxt as $outname | &$mainself | { $( $items )* });
    };

    (
        $name:ty : $ctxt:ty as $outname:tt | &$mainself:ident | {
            $( $items:tt )*
        }
    ) => {
        graphql_union!(() $name : $ctxt as $outname | &$mainself | { $( $items )* });
    };

    (
        $name:ty : $ctxt:ty | &$mainself:ident | {
            $( $items:tt )*
        }
    ) => {
        graphql_union!(() $name : $ctxt as (stringify!($name)) | &$mainself | { $( $items )* });
    };
}
