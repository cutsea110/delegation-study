use log::{debug, trace};
use tx_rs::Tx;

use crate::UsecaseError;
use dao::{EmployeeDao, HaveEmployeeDao};
use payroll_domain::{Employee, EmployeeId};

// ユースケース: ChangeEmployee トランザクション(抽象レベルのビジネスロジック)
pub trait ChangeEmployee: HaveEmployeeDao {
    fn get_id(&self) -> EmployeeId;
    fn change(emp: &mut Employee);
    fn execute<'a>(&self) -> Result<(), UsecaseError> {
        trace!("ChangeEmployeeName::execute called");
        self.dao()
            .run_tx(|mut ctx| {
                trace!("ChangeEmployee::run_tx called");
                let mut emp = self.dao().fetch(self.get_id()).run(&mut ctx)?;
                debug!(r#"changing emp="{:?}""#, emp);
                Self::change(&mut emp);
                debug!(r#"changed emp="{:?}""#, emp);
                self.dao().update(emp).run(&mut ctx)
            })
            .map_err(UsecaseError::ChangeEmployeeFailed)
    }
}
