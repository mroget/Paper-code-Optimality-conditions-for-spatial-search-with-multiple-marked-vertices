use pyo3::prelude::*;

mod qw;


///     hitting_time(x : float) -> int
///     --
///
///        This function returns the hitting time for a given grid size.
///        The formula sqrt(pi*x*log(x))/4
///
///        Entries :
///            x -> The ratio N/M
///        Output : 
///            The step at which the probability of success while searching one element will be optimal (for a grid of size nxn).
#[pyfunction]
pub fn hitting_time(x : f64) -> PyResult<usize> {
    Ok(qw::hitting_time(x))
}



///     qw(n : int, m : int, search : List(int)) -> float
///     --
///
///        This function runs the quantum search and returns the probability of success at the desired step for a grid of size N=n*n.
///
///        Entries :
///            n -> Size of the grid.
///            m -> Number of steps.
///            search -> List(int) of the searched elements.
///        Output :
///            The probability of measuring one of the searched element in a grid of size n*n for each steps between 0 and m.
#[pyfunction]
pub fn qw(n : usize, m : usize, search : Vec<usize>) -> PyResult<f64> {
    for i in 0..search.len() {
        assert!(search[i] < n*n);
    }
    Ok(qw::qw(n,m,&search))
}



///     qw_signal(n : int, m : int, search : List(int)) -> List(float)
///     --
///
///        This function runs the quantum search and returns the probability of success at each steps for a grid of size N=n*n.
///
///        Entries :
///            n -> Size of the grid.
///            m -> Number of steps.
///            search -> List(int) of the searched elements.
///        Output :
///            The probability of measuring one of the searched element in a grid of size n*n for each steps between 0 and m.
#[pyfunction]
fn qw_signal(n : usize, m : usize, search : Vec<usize>) -> PyResult<Vec<f64>> {
    for i in 0..search.len() {
        assert!(search[i] < n*n);
    }
    Ok(qw::qw_signal(n,m,&search))
}


///     qw_sample(n : int, nb_searched : int, nb_iter : int, scale_hitting_time : bool) -> List(float)
///     --
///
///        This function runs the quantum search with a configuration of searched elements drawn randomly and returns the probability of success for a grid of size N=n*n.
///
///        Entries :
///            n -> Size of the grid.
///            nb_searched -> Number of searched elements.
///            nb_iter -> Number of samples.
///            scale_hitting_time -> Wether the hitting time is computed in function of N/M (true) or N (False).
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
