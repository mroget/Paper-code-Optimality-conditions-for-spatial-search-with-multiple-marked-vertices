extern crate ndarray;
use ndarray::Array2;
use ndarray::Array3;
use ndarray::array;

use num::complex::Complex;

use std::f64::consts::PI;

use rand::thread_rng;
use rand::seq::SliceRandom; 

use rayon::prelude::*;
use rand::Rng;


/*******************************************************************/
/*                       QW operator                               */
/*******************************************************************/

fn coin(psi : &mut Array3<Complex<f64>>, coin : &Array2<Complex<f64>>) {
    /*
        Apply a coin operator to the walker.

        Entries :
            psi -> Walker's state.
            coin -> Coin matrix (2x2 matrix). 
        Output :
            Nothing but the function modifies the walker's state psi.
    */
    let n = psi.shape()[0];
    let mut phi = Array3::<Complex<f64>>::zeros((n, n, 2));
    for i in 0..n {
        for j in 0..n {
            for k in 0..2 {
        phi[[i,j,k]] = psi[[i,j,k]];
    }}}

    for i in 0..n {
        for j in 0..n {
            psi[[i,j,0]] = phi[[i,j,0]]*coin[[0,0]] + phi[[i,j,1]]*coin[[0,1]];
            psi[[i,j,1]] = phi[[i,j,0]]*coin[[1,0]] + phi[[i,j,1]]*coin[[1,1]];
        }
    }
}


fn scatterX(psi : &mut Array3<Complex<f64>>) {
    /*
        Apply the scattering operator on axis X to the walker.

        Entries :
            psi -> Walker's state.
        Output :
            Nothing but the function modifies the walker's state psi.
    */
    let n = psi.shape()[0];
    let mut phi = Array3::<Complex<f64>>::zeros((n, n, 2));
    for i in 0..n {
        for j in 0..n {
            for k in 0..2 {
        phi[[i,j,k]] = psi[[i,j,k]];
    }}}

    for i in 0..n {
        for j in 0..n {
            psi[[i,j,0]] = phi[[(i+1)%n,j,0]];
            psi[[i,j,1]] = phi[[(i+n-1)%n,j,1]];
        }
    }
}



fn scatterY(psi : &mut Array3<Complex<f64>>) {
    /*
        Apply the scattering operator on axis Y to the walker.

        Entries :
            psi -> Walker's state.
        Output :
            Nothing but the function modifies the walker's state psi.
    */
    let n = psi.shape()[0];
    let mut phi = Array3::<Complex<f64>>::zeros((n, n, 2));
    for i in 0..n {
        for j in 0..n {
            for k in 0..2 {
        phi[[i,j,k]] = psi[[i,j,k]];
    }}}

    for i in 0..n {
        for j in 0..n {
            psi[[i,j,0]] = phi[[i,(j+1)%n,0]];
            psi[[i,j,1]] = phi[[i,(j+n-1)%n,1]];
        }
    }
}


fn oracle(psi : &mut Array3<Complex<f64>>, search : &Vec<usize>) {
    /*
        Apply the oracle operator to the walker.

        Entries :
            psi -> Walker's state.
            search -> List of searched elements.
        Output :
            Nothing but the function modifies the walker's state psi.
    */
    let n = psi.shape()[0];
    let mut phi = Array3::<Complex<f64>>::zeros((n, n, 2));
    for i in 0..n {
        for j in 0..n {
            for k in 0..2 {
                phi[[i,j,k]] = psi[[i,j,k]];
    }}}

    for k in search.iter() {
        let i = k/n;
        let j = k%n;
        psi[[i,j,0]] = -phi[[i,j,1]];   // Oracle
        psi[[i,j,1]] = -phi[[i,j,0]];   // Oracle
    }
}


fn measure(psi : &Array3<Complex<f64>>, search : &Vec<usize>) -> f64{
    /*
        This function computes the probability of measuring a searched element given a state psi.

        Entries :
            psi -> Walker's state.
            search -> List of searched elements.
        Output :
            The probability of success.
    */

    let n = psi.shape()[0];

    let mut p : f64 = 0.;
    for k in search.iter() {
        let i = k/n;
        let j = k%n;
        p+= psi[[i,j,0]].norm().powf(2.) + psi[[i,j,1]].norm().powf(2.);
    }

    p
}


pub fn tick(mut psi : &mut Array3<Complex<f64>>, search : &Vec<usize>) {
    /*
        This function applies one step of the quantum search to the state psi.

        Entries :
            psi -> Walker's state.
            search -> List of searched elements.
        Output :
            Nothing but the function modifies the walker's state psi.
    */

    let one = Complex::new(1./(2. as f64).sqrt(), 0.);
    let i = Complex::new(0., 1./(2. as f64).sqrt());
    let Cx : Array2::<Complex<f64>> = array![[one,i],[i,one]];
    let Cy : Array2::<Complex<f64>> = array![[one,-i],[-i,one]];

    oracle(&mut psi, search);
    coin(&mut psi, &Cx);
    scatterX(&mut psi);
    coin(&mut psi, &Cy);
    scatterY(&mut psi);
}




/*******************************************************************/
/*                   Simulation functions                          */
/*******************************************************************/


pub fn hitting_time(x : f64) -> usize {
    /*
        This function returns the hitting time for a given grid size.
        The formula sqrt(pi*x*log(x))/4

        Entries :
            x -> The ratio N/M
        Output : 
            The step at which the probability of success while searching one element will be optimal (for a grid of size nxn).
    */
    ((PI*x*x.ln()).sqrt()/4.).floor() as usize
}


pub fn qw(n : usize, m : usize, search : &Vec<usize>) -> f64 {
    /*
        This function runs the quantum search and returns the probability of success at the desired step for a grid of size N=n*n.

        Entries :
            n -> Size of the grid.
            m -> Number of steps.
            search -> List(int) of the searched elements.
        Output :
            The probability of measuring one of the searched element in a grid of size n*n for each steps between 0 and m.
    */
    let mut psi = Array3::<Complex<f64>>::ones((n, n, 2));
    for i in 0..n {
        for j in 0..n {
            for k in 0..2 {
        psi[[i,j,k]] = psi[[i,j,k]]/(2.*(n as f64)*(n as f64)).sqrt()
    }}}
    
    for i in 0..m {
        tick(&mut psi,&search);
    }

    measure(&psi,&search)
}


pub fn qw_signal(n : usize, m : usize, search : &Vec<usize>) -> Vec<f64> {
    /*
        This function runs the quantum search and returns the probability of success at each steps for a grid of size N=n*n.

        Entries :
            n -> Size of the grid.
            m -> Number of steps.
            search -> List(int) of the searched elements.
        Output :
            The probability of measuring one of the searched element in a grid of size n*n for each steps between 0 and m.
    */
    let mut psi = Array3::<Complex<f64>>::ones((n, n, 2));
    for i in 0..n {
        for j in 0..n {
            for k in 0..2 {
        psi[[i,j,k]] = psi[[i,j,k]]/(2.*(n as f64)*(n as f64)).sqrt()
    }}}


    let mut p = Vec::with_capacity(m+1);
    p.push(measure(&psi,&search));
    
    for i in 0..m {
        tick(&mut psi,&search);
        p.push(measure(&psi,&search));
    }

    p
}

pub fn qw_sample_one(n : usize, nb_searched : usize, scale_hitting_time : bool) -> f64 {
    /*
        This function runs the quantum search with a configuration of searched elements drawn randomly and returns the probability of success for a grid of size N=n*n.

        Entries :
            n -> Size of the grid.
            nb_searched -> Number of searched elements.
            scale_hitting_time -> Wether the hitting time is computed in function of N/M (true) or N (False).
        Output :
            The probability of measuring one of the searched element in a grid of size nxn for the optimal number of steps.
            The searched element are chosen uniformly and the number of steps is given by the hitting_time function.
    */

    let mut rng = &mut rand::thread_rng();
    let range : Vec<usize> = (0..(n*n)).collect();
    let search : Vec<usize> = range.choose_multiple(&mut rng,nb_searched).cloned().collect();

    let hit = if scale_hitting_time {hitting_time(((n*n) as f64)/nb_searched as f64)} else {hitting_time((n*n) as f64)};

    qw(n,hit,&search)
}


pub fn qw_sample(n : usize, nb_searched : usize, nb_iter : usize, scale_hitting_time : bool) -> Vec<f64> {
    /*
        This function runs the quantum search with a configuration of searched elements drawn randomly and returns the probability of success for a grid of size N=n*n.

        Entries :
            n -> Size of the grid.
            nb_searched -> Number of searched elements.
            nb_iter -> Number of samples.
            scale_hitting_time -> Wether the hitting time is computed in function of N/M (true) or N (False).
        Output :
            The probability of measuring one of the searched element in a grid of size nxn for the optimal number of steps.
            The searched element are chosen uniformly and the number of steps is given by the hitting_time function.
    */
    let steps : Vec<usize> = (0..nb_iter).collect();
    let ret : Vec<f64> = steps.par_iter().map(|&i| qw_sample_one(n,nb_searched,scale_hitting_time)).collect();
    ret
}
