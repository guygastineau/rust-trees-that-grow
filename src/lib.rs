use std::fmt;

pub trait Familyξ<Ξ> { type R; }
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

#[derive(Debug)]
pub enum Typ { I, V, Fun (Box<Typ>, Box<Typ>) }

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
            Lit(ξ, n) => f.debug_struct("Lit")
                .field("ξ", ξ)
                .field("n", n)
                .finish(),
            Var(ξ, x) => f.debug_struct("Var")
                .field("ξ", ξ)
                .field("x", x)
                .finish(),
            Ann(ξ, x, α) => f.debug_struct("Ann")
                .field("ξ", ξ)
                .field("x", x)
                .field("α", α)
                .finish(),
            Abs(ξ, x, m) => f.debug_struct("Abs")
                .field("ξ", ξ)
                .field("x", x)
                .field("m", m)
                .finish(),
            App(ξ, λ, x) => f.debug_struct("App")
                .field("ξ", ξ)
                .field("f", λ)
                .field("x", x)
                .finish(),
            Exp(ξ) => f.debug_struct("Exp")
                .field("ξ", ξ)
                .finish(),
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
impl Familyξ<UD> for LitS { type R = (); }
impl Familyξ<UD> for VarS { type R = (); }
impl Familyξ<UD> for AnnS { type R = (); }
impl Familyξ<UD> for AbsS { type R = (); }
impl Familyξ<UD> for AppS { type R = (); }
impl Familyξ<UD> for ExpS { type R = (); }

pub type ExpUD = Expression<UD>;

pub struct TC;
impl Familyξ<TC> for LitS { type R = (); }
impl Familyξ<TC> for VarS { type R = (); }
impl Familyξ<TC> for AnnS { type R = (); }
impl Familyξ<TC> for AbsS { type R = (); }
impl Familyξ<TC> for AppS { type R = Typ; }
impl Familyξ<TC> for ExpS { type R = (); }

pub type ExpTC = Expression<TC>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_lit() {
        println!("{:?}", Expression::Lit((), 5) as ExpTC)
    }
}
