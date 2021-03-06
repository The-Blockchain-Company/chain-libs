//! Internal macros

macro_rules! std_ops_gen {
    ($lty: ident, $class: ident, $rty: ident, $out: ident, $f: ident) => {
        impl<'a> $class<$rty> for &'a $lty {
            type Output = $out;

            fn $f(self, other: $rty) -> Self::Output {
                self.$f(&other)
            }
        }

        impl<'b> $class<&'b $rty> for $lty {
            type Output = $out;

            fn $f(self, other: &'b $rty) -> Self::Output {
                (&self).$f(other)
            }
        }

        impl $class<$rty> for $lty {
            type Output = $out;

            fn $f(self, other: $rty) -> Self::Output {
                (&self).$f(&other)
            }
        }
    };
}

macro_rules! gen_group_tests {
    () => {
        #[cfg(test)]
        mod group_tests {
            use super::*;
            use cryptoxide::blake2b::Blake2b;
            use cryptoxide::digest::Digest;
            use smoke::{
                generator::{self, BoxGenerator},
                Generator,
            };

            fn fe_generator() -> BoxGenerator<Scalar> {
                generator::Array5::new(generator::num::<u8>())
                    .map(|a| {
                        let mut hash = Blake2b::new(64);
                        hash.input(&a);
                        Scalar::hash_to_scalar(&hash)
                    })
                    .into_boxed()
            }

            #[test]
            fn add_zero() {
                let fe = Scalar::from_u64(64);
                let ge = GroupElement::generator() * fe;
                assert_eq!(GroupElement::zero() + &ge, ge);
            }

            #[test]
            fn clone() {
                let fe1 = Scalar::from_bytes(&[1u8; 32]);
                assert_eq!(fe1, fe1.clone());
            }
            #[test]
            fn associative() {
                let fe1 = Scalar::from_u64(124);
                let fe2 = Scalar::from_u64(434);
                let fe3 = Scalar::from_u64(124 + 434);

                let ge1 = GroupElement::generator() * &fe1;
                let ge2 = GroupElement::generator() * &fe2;
                let ge3 = GroupElement::generator() * &fe3;

                let ge3_got = ge1 + ge2;

                assert_eq!(fe3, fe1 + fe2);
                assert_eq!(ge3_got, ge3);
            }

            #[test]
            fn ge_inverse() {
                let fe1 = Scalar::from_u64(124);
                let g = (GroupElement::generator() * &fe1) * fe1.inverse();
                assert_eq!(g, GroupElement::generator())
            }

            #[test]
            fn fe_inverse() {
                let fe1 = Scalar::from_u64(124);
                assert_eq!(&fe1 * fe1.inverse(), Scalar::one())
            }

            #[test]
            fn arithmetic_correct() {
                use smoke::{forall, property, run, Testable};
                run(|ctx| {
                    // add associative
                    forall(fe_generator().and(fe_generator()).and(fe_generator()))
                        .ensure(|((e1, e2), e3)| property::equal((e1 + e2) + e3, e1 + (e2 + e3)))
                        .test(ctx);

                    forall(fe_generator().and(fe_generator()))
                        .ensure(|(e1, e2)| property::equal(e1 + e2, e2 + e1))
                        .test(ctx);

                    forall(fe_generator())
                        .ensure(|e| property::equal(e.clone(), e.clone()))
                        .test(ctx);

                    forall(fe_generator())
                        .ensure(|e| property::equal(e * e.inverse(), Scalar::one()))
                        .test(ctx);

                    forall(fe_generator())
                        .ensure(|e1| property::equal(e1 + e1.negate(), Scalar::zero()))
                        .test(ctx);

                    forall(fe_generator().and(fe_generator()))
                        .ensure(|(e1, e2)| property::equal((e1 - e2) + e2, e1.clone()))
                        .test(ctx);

                    forall(fe_generator().and(fe_generator()).and(fe_generator()))
                        .ensure(|((e1, e2), e3)| property::equal((e1 + e2) * e3, e1 * e3 + e2 * e3))
                        .test(ctx);

                    /*
                    forall(fe_generator())
                        .ensure(|e1| {
                            let g = (GroupElement::generator() * e1) * e1.inverse();
                            property::equal(g, GroupElement::generator())
                        })
                        .test(ctx);
                        */
                })
            }
        }
    };
}
