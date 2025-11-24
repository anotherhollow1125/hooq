use hooq::hooq;

mod sub {
    pub trait Trait {}
}

fn failable<T>(val: T) -> Result<T, String> {
    Ok(val)
}

#[hooq(flavor = "hook", trait_use(sub::Trait))] // Attribute macro root.
#[hooq::method(.inspect_err(|_| { let _ = "error!"; }))] // All following attributes are inert.
#[hooq::hook_targets("?", "return", "tail_expr")]
#[hooq::tail_expr_idents("Err")]
#[hooq::ignore_tail_expr_idents("Ok")]
#[hooq::result_types("Result")]
#[hooq::hook_in_macros(true)]
#[hooq::binding(xxx = "xxx_value")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    failable(())?;

    #[hooq::skip_all]
    if failable(false)? {
        failable(())?;
    }

    #[hooq::skip]
    if failable(false)? {
        // Next line is not skipped.
        failable(())?;
    }

    #[hooq::method(.inspect_err(|_| { let _ = $xxx; }))]
    failable(())?;

    Ok(())
}
