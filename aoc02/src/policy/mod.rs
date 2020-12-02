mod new_policy;
mod old_policy;

pub use new_policy::NewPolicy;
pub use old_policy::OldPolicy;
pub trait Policy {
    fn from_str(input: &str) -> Self;
    fn is_password_valid(&self, pw: &str) -> bool;
}
