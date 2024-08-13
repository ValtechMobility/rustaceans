use pyo3::prelude::*;

// Macros to quickly add classes and functions to the module
macro_rules! add_classes {
    ($module:ident, $($class:ty), +) => {
        $(
            $module.add_class::<$class>()?;
        )+
    };
}

macro_rules! add_function {
    ($module:ident, $($function:ident),+) => {
        $(
            $module.add_wrapped(wrap_pyfunction!($function))?;
        )+
    };
}

#[pyclass(module = "rustaceans", eq, eq_int)]
#[derive(PartialEq, Clone, Debug)]
enum Role {
    Admin,
    User,
    Guest,
}

#[pyclass(module = "rustaceans", eq, eq_int)]
#[derive(PartialEq, Clone, Debug)]
enum Location {
    Munich,
    Frankfurt,
}

#[pyclass(module = "rustaceans", get_all, set_all)]
struct Employee {
    name: String,
    role: Role,
    location: Location,
}

#[pymethods]
impl Employee {
    #[new]
    #[pyo3(signature = (name, role, location))]
    fn new(name: String, role: Role, location: Location) -> Self {
        Employee {
            name,
            role,
            location,
        }
    }
}

#[pyfunction]
fn pretty_print_role(role: &Role) {
    println!("Role: {:?}", role);
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustaceans(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Package all functions and classes by using the macros
    add_classes!(m, Role, Location);
    add_function!(m, pretty_print_role);
    Ok(())
}
