use pyo3::prelude::*;

mod qw;


///     hitting_time(n : int) -> int
///     --
///
///       This function returns the hitting time for a given grid size.
///
///       Entries :
///            n -> Size of the grid.
///        Output : 
///            The step at which the probability of success while searching one element will be optimal (for a grid of size nxn).
#[pyfunction]
pub fn hitting_time(n : usize) -> PyResult<usize> {
    Ok(qw::hitting_time(n))
}



///     qw(n : int, m : int, search : List(int)) -> float
///     --
///
///        This function runs the quantum search and returns the probability of success at the desired step.
///
///        Entries :
///            n -> Size of the grid.
///            m -> Number of steps.
///            search -> List(int) of the searched elements.
///        Output :
///            The probability of measuring one of the searched element in a grid of size nxn for each steps between 0 and m.
#[pyfunction]
pub fn qw(n : usize, m : usize, search : Vec<usize>) -> PyResult<f64> {
    for i in 0..search.len() {
        assert!(search[i] < n*n);
    }
    Ok(qw::qw(n,m,&search))
}



///     qw_signal(N : int, M : int, search : List(int)) -> List(float)
///     --
///
///        This function runs the quantum search and returns the probability of success at each steps.
///
///        Entries :
///            n -> Size of the grid.
///            m -> Number of steps.
///            search -> List(int) of the searched elements.
///        Output :
///            The probability of measuring one of the searched element in a grid of size nxn for each steps between 0 and m.
#[pyfunction]
fn qw_signal(n : usize, m : usize, search : Vec<usize>) -> PyResult<Vec<f64>> {
    for i in 0..search.len() {
        assert!(search[i] < n*n);
    }
    Ok(qw::qw_signal(n,m,&search))
}


///     qw_sample(n : int, nb_searched : int, nb_iter : int) -> List(float)
///     --
///
///        This function runs the quantum search with a configuration of searched elements drawn randomly and returns the probability of success.
///
///        Entries :
///            n -> Size of the grid.
///            nb_searched -> Number of searched elements.
///            nb_iter -> Number of samples.
///        Output :
///            The probability of measuring one of the searched element in a grid of size nxn for the optimal number of steps.
///            The searched element are chosen uniformly and the number of steps is given by the hitting_time function.
#[pyfunction]
pub fn qw_sample(n : usize, nb_searched : usize, nb_iter : usize, scale_hitting_time : bool) -> PyResult<Vec<f64>> {
    Ok(qw::qw_sample(n,nb_searched,nb_iter,scale_hitting_time))
}


#[pymodule]
fn qwsearch(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hitting_time, m)?)?;
    m.add_function(wrap_pyfunction!(qw, m)?)?;
    m.add_function(wrap_pyfunction!(qw_signal, m)?)?;
    m.add_function(wrap_pyfunction!(qw_sample, m)?)?;
    Ok(())
}
