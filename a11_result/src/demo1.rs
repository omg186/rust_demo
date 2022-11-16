// Topic : Result & the question mark operator

// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * 确定员工是否可以使用数字钥匙卡进入建筑物
// * Employees that can access the building are:
// * 可以进入大楼的员工是
//      * Maintenance crews
//      * 维修人员
//      * Marketing department employees
//      * 市场部员工
//      * Managers
//      * 管理员
// * Other employees that work at the company are:
// * 在工作的其他员工是:
//      * Line supervisors
//      * 生产线主管
//      * Kitchen staff
//      * 厨房工作人员
//      * Assembly technicians
//      * 装配技术员
// * Ensure that terminated employees cannot access the building regardless of their position
// * 确保被解雇的员工无法进入大楼 无论他们的职位如何
// Notes:
// * Use an enum to represent all types of employees
// * 使用枚举表示所有类型的员工
pub enum Position {
    // MaintenanceCrews,
    // MarketingDepartment,
    Managers,
    // LineSupervisors,
    // KitchenStaff,
    AssemblyTechnicians,
}
pub enum Status {
    Active,
    Terminated,
}
// * Use a struct to store the employee type and whether they are still employed
pub struct Employee {
    pub position: Position,
    pub status: Status,
}
// * Use a function that returns a Result to determine if the employee may enter the building
pub fn try_access(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Terminated => Err("terminated".to_owned()),
        _ => Ok(()),
    }
}
// * Print whether the employee may access the building
//      * Must use a function that utilizes the question mark operator to do this

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_success_work() -> Result<(), String> {
        let employee = Employee {
            position: Position::Managers,
            status: Status::Active,
        };
        try_access(&employee)?;
        Ok(())
    }
    #[test]
    fn it_error_work() {
        let employee = Employee {
            position: Position::Managers,
            status: Status::Terminated,
        };
        let result = try_access(&employee);
        assert!(result.is_err());
    }
}
