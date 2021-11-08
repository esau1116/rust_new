// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
enum Position{
    Maintenance,
    Marketing,
    Managers,
    Supervisors,
    Kitchen,
    Technicians,
}
// * Use a struct to store the employee type and whether they are
//   still employed
struct Status{
    Acitive,
    Terminated

}
struct Employee{
    status: Status,
    position:Position,
}
// * Use a function that returns a Result to determine if the employee
//   may enter the building
fn get_access(employee:&Employee) -> Result<(), String>{
    match employee.status{
        Status::Terminated => return err("cant access".to_owned())
    }
}
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

fn main() {}
