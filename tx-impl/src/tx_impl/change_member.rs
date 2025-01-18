use anyhow;
use log::trace;
use std::{cell::RefCell, rc::Rc};

use crate::ChangeMember;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{EmployeeId, MemberId};
use payroll_factory::PayrollFactory;
use tx_app::{Response, Transaction};

// ユースケース: ChangeMember トランザクションの実装 (struct)
#[derive(Debug)]
pub struct ChangeMemberTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    member_id: MemberId,
    emp_id: EmployeeId,
    dues: f32,

    dao: T,
    payroll_factory: F,
}
impl<T, F> ChangeMemberTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    pub fn new(
        member_id: MemberId,
        emp_id: EmployeeId,
        dues: f32,
        dao: T,
        payroll_factory: F,
    ) -> Self {
        Self {
            member_id,
            emp_id,
            dues,
            dao,
            payroll_factory,
        }
    }
}

impl<T, F> HaveEmployeeDao for ChangeMemberTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    type Ctx<'a> = T::Ctx<'a>;

    fn dao<'a>(&self) -> &impl EmployeeDao<Ctx<'a> = Self::Ctx<'a>> {
        &self.dao
    }
}
impl<T, F> ChangeMember for ChangeMemberTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn get_member_id(&self) -> MemberId {
        self.member_id
    }
    fn get_emp_id(&self) -> EmployeeId {
        self.emp_id
    }
    fn get_dues(&self) -> f32 {
        self.dues
    }
    fn get_affiliation(&self) -> Rc<RefCell<dyn payroll_domain::Affiliation>> {
        self.payroll_factory
            .mk_union_affiliation(self.get_member_id(), self.get_dues())
    }
}
// 共通インターフェースの実装
impl<T, F> Transaction for ChangeMemberTx<T, F>
where
    T: EmployeeDao,
    F: PayrollFactory,
{
    fn execute(&self) -> Result<Response, anyhow::Error> {
        trace!("ChangeMemberTx::execute called");
        ChangeMember::execute(self)
            .map(|_| Response::Void)
            .map_err(Into::into)
    }
}
