use metis_contract::branch::Branch;

pub trait BranchService {
    fn create_branch(&self, branch: Branch) -> Result<Branch, &'static str>;
}

pub struct StubBranchService;

impl BranchService for StubBranchService {
    fn create_branch(&self, _branch: Branch) -> Result<Branch, &'static str> {
        Err("branches are scaffolding-only in phase 1")
    }
}
