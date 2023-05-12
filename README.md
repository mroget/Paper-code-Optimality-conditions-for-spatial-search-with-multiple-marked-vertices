# Paper-code-Optimality-conditions-for-spatial-search-with-multiple-marked-vertices
This repository provides the source code to reproduce the figures and results of the paper "Optimality conditions for spatial search with multiple marked vertices", Mathieu Roget, Hachem Kadri, and Giuseppe Di Molfetta : https://arxiv.org/abs/2201.12937

## 1. Organisation of this repository
The main content of this repository is the notebook *code.ipynb* which contains all the python code used to produce our results with a lot of comments and explanations.
However, in order to increase the efficiency of the simulations, the notebook is calling a rust implementation of the QWSearch algorithm introduced in our paper.
	- *QWSearch_rust-0.1.0.tar.gz* : This is the source of the rust file you will have to compile in order to execute the python notebook.
	- *QWSearch_rust/* : This folder contains the rust code with everything necessary to modify the source code. Thois part is adressed to people trying to modify the implementation of the algorithm for further work. If you just want to run the notebook, then you don't need to care about this folder.

## 2. Compilation of the rust implementation
Make sure that you have a working python3 and pip installation.
### 2.1. Install Rust and Cargo
Cargo is the rust package manager. You can find more information about cargo and rust by following this link: https://doc.rust-lang.org/cargo/index.html
#### On Linux & Mac
You can install both of them by running 
```
curl https://sh.rustup.rs -sSf | sh
```
#### On Windows
You can install both by installing the executable that you will find on the official webpage: https://doc.rust-lang.org/cargo/getting-started/installation.html

### 2.2 Install Maturin
Maturin is a tool to create python package. More information here: https://www.maturin.rs/index.html
You can install maturin using cargo with the following command:
```
cargo install --locked maturin
```
Several other options for installation are available here: https://www.maturin.rs/installation.html

### 2.3 Build
You can now build and install the package necessary for running the notebook by using the command:
```
pip install QWSearch_rust-0.1.0.tar.gz 
```
**Note that this will install a python package qwsearch on your computer which contains the implementation of the QWSearch algorithm. Tou can uninstall this package with the following command:**
```
pip uninstall QWSearch_rust
```

## 3. Notebook's Dependencies
The required packages to run the notebook are 
	- `numpy`
	- `pandas`
	- `matplotlib`
	- `seaborn`
	- `qwsearch`
Here is the command for a quick install:
```
pip install numpy pandas matplotlib seaborn
```