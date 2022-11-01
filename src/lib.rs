use std::{collections::HashMap, fmt};

pub trait Familyξ<Ξ> {
    type R;
}
type Runξ<T, Ξ> = <T as Familyξ<Ξ>>::R;

pub enum Expression<Ξ>
where
    LitS: Familyξ<Ξ>,
    VarS: Familyξ<Ξ>,
    AnnS: Familyξ<Ξ>,
    AbsS: Familyξ<Ξ>,
    AppS: Familyξ<Ξ>,
    ExpS: Familyξ<Ξ>,
{
    Lit(Runξ<LitS, Ξ>, i32),
    Var(Runξ<VarS, Ξ>, String),
    Ann(Runξ<AnnS, Ξ>, Box<Expression<Ξ>>, Typ),
    Abs(Runξ<AbsS, Ξ>, String, Box<Expression<Ξ>>),
    App(Runξ<AppS, Ξ>, Box<Expression<Ξ>>, Box<Expression<Ξ>>),
    Exp(Runξ<ExpS, Ξ>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Typ {
    I,
    Fun(Box<Typ>, Box<Typ>),
}

impl<Ξ> fmt::Debug for Expression<Ξ>
where
    LitS: Familyξ<Ξ>,
    VarS: Familyξ<Ξ>,
    AnnS: Familyξ<Ξ>,
    AbsS: Familyξ<Ξ>,
    AppS: Familyξ<Ξ>,
    ExpS: Familyξ<Ξ>,
    Runξ<LitS, Ξ>: fmt::Debug,
    Runξ<VarS, Ξ>: fmt::Debug,
    Runξ<AnnS, Ξ>: fmt::Debug,
    Runξ<AbsS, Ξ>: fmt::Debug,
    Runξ<AppS, Ξ>: fmt::Debug,
    Runξ<ExpS, Ξ>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Expression::*;
        match self {
            Lit(ξ, n) => f.debug_struct("Lit").field("ξ", ξ).field("n", n).finish(),
            Var(ξ, x) => f.debug_struct("Var").field("ξ", ξ).field("x", x).finish(),
            Ann(ξ, x, α) => f
                .debug_struct("Ann")
                .field("ξ", ξ)
                .field("x", x)
                .field("α", α)
                .finish(),
            Abs(ξ, x, m) => f
                .debug_struct("Abs")
                .field("ξ", ξ)
                .field("x", x)
                .field("m", m)
                .finish(),
            App(ξ, λ, x) => f
                .debug_struct("App")
                .field("ξ", ξ)
                .field("f", λ)
                .field("x", x)
                .finish(),
            Exp(ξ) => f.debug_struct("Exp").field("ξ", ξ).finish(),
        }
    }
}

pub struct LitS;
pub struct VarS;
pub struct AnnS;
pub struct AbsS;
pub struct AppS;
pub struct ExpS;

pub struct UD;
impl Familyξ<UD> for LitS {
    type R = ();
}
impl Familyξ<UD> for VarS {
    type R = ();
}
impl Familyξ<UD> for AnnS {
    type R = ();
}
impl Familyξ<UD> for AbsS {
    type R = ();
}
impl Familyξ<UD> for AppS {
    type R = ();
}
impl Familyξ<UD> for ExpS {
    type R = ();
}

pub type ExpUD = Expression<UD>;

pub struct TC;
impl Familyξ<TC> for LitS {
    type R = ();
}
impl Familyξ<TC> for VarS {
    type R = ();
}
impl Familyξ<TC> for AnnS {
    type R = ();
}
impl Familyξ<TC> for AbsS {
    type R = ();
}
impl Familyξ<TC> for AppS {
    type R = Typ;
}
impl Familyξ<TC> for ExpS {
    type R = ();
}

pub type ExpTC = Expression<TC>;

impl ExpTC {
    pub fn check(&self, γ: &HashMap<String, Typ>, α: Typ) -> bool {
        use Expression::*;
        match self {
            Lit(_, _) => true,
            Var(_, x) => γ.get(x).map(|x| *x == α).unwrap_or(false),
            Ann(_, x, β) => α == *β && x.check(γ, α),
            Abs(_, x, m) => match α {
                Typ::Fun(α, β) => {
                    let mut γ = γ.clone();
                    γ.insert(x.clone(), *α);
                    m.check(&γ, *β)
                }
                _ => false,
            },
            App(β, f, x) => {
                x.check(γ, β.clone()) && f.check(γ, Typ::Fun(Box::new(β.clone()), Box::new(α)))
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref Γ: HashMap<String, Typ> = [
            ("x", Typ::I),
            (
                "f",
                Typ::Fun(
                    Box::new(Typ::I),
                    Box::new(Typ::Fun(Box::new(Typ::I), Box::new(Typ::I)))
                )
            )
        ]
        .into_iter()
        .fold(HashMap::new(), |mut γ, (id, α)| {
            γ.insert(id.to_owned(), α);
            γ
        });
    }

    #[test]
    fn check_lit() {
        assert!(Expression::Lit((), 5).check(&Γ, Typ::I));
    }

    #[test]
    fn check_var() {
        assert!(Expression::Var((), "x".to_owned()).check(&Γ, Typ::I));
        assert!(Expression::Var((), "f".to_owned()).check(
            &Γ,
            Typ::Fun(
                Box::new(Typ::I),
                Box::new(Typ::Fun(Box::new(Typ::I), Box::new(Typ::I)))
            )
        ));
        assert!(!Expression::Var((), "y".to_owned()).check(&Γ, Typ::I));
    }

    #[test]
    fn check_ann() {
        let ann = Expression::Ann((), Box::new(Expression::Lit((), 10)), Typ::I);
        assert!(ann.check(&Γ, Typ::I));
        assert!(!ann.check(&Γ, Typ::Fun(Box::new(Typ::I), Box::new(Typ::I))));
    }

    #[test]
    fn check_abs() {
        // Identity
        let abs: ExpTC = Expression::Abs(
            (),
            "x".to_owned(),
            Box::new(Expression::Var((), "x".to_owned())),
        );
        let g = Typ::Fun(Box::new(Typ::I), Box::new(Typ::I));
        let α = Typ::Fun(Box::new(g.clone()), Box::new(g.clone()));
        assert!(abs.check(&Γ, α));
        assert!(!abs.check(&Γ, Typ::Fun(Box::new(g.clone()), Box::new(Typ::I))));
        // Application of abstracted variable on free variable
        let bbs: ExpTC = Expression::Abs(
            (),
            "y".to_owned(),
            Box::new(Expression::App(
                Typ::I,
                Box::new(Expression::Var((), "y".to_owned())),
                Box::new(Expression::Var((), "x".to_owned())),
            )),
        );
        assert!(bbs.check(&Γ, Typ::Fun(Box::new(g), Box::new(Typ::I))));
    }

    #[test]
    fn check_app() {
        let g = Typ::Fun(Box::new(Typ::I), Box::new(Typ::I));
        let app = |α| {
            Expression::App(
                α,
                Box::new(Expression::Var((), "f".to_owned())),
                Box::new(Expression::Var((), "x".to_owned())),
            )
        };
        assert!(app(Typ::I).check(&Γ, g.clone()));
        assert!(!app(g.clone()).check(&Γ, g))
    }
}
