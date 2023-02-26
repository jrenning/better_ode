use pyo3::{prelude::*, types::PyFunction};
use pyo3::types::PyTuple;
use nalgebra::DVector;




/// Formats the sum of two numbers as string.
#[pyfunction]
fn solve_diffeq_system(system: PyObject, yo: Vec<f64>) -> PyResult<ODE_Results> {
    // Initial state.
    //type State = DVector<f64>;
    type State = Vector2<f64>;
    let y0 = State::from_vec(yo);

    struct DifferentalSystem {
        eq_system: PyObject
    };

    impl ode_solvers::System<State> for DifferentalSystem {
        fn system(&self, t: Time, y: &State, dy: &mut State) {
            Python::with_gil(|py| {
                let mut wrapped_vec = vec![];
                for val in y.row_iter() {
                    let x = val[0];
                    wrapped_vec.push(x);
                }
                // args in rust syntax
                let args_r = (t, wrapped_vec.clone());
                // convert to python tuple
                let args_p: Py<PyTuple> = args_r.into_py(py);
                let res1 = match self.eq_system.call1(py, args_p.as_ref(py)) {
                    Ok(y) => y,
                    Err(error) =>  {
                        println!("An error occured: {}", error);
                        panic!();
                    }
                };
                let change: Vec<f64> = match res1.extract(py) {
                    Ok(change) => change,
                    Err(err) => panic!("Err")
                };
                for (i, val) in change.into_iter().enumerate() {
                    dy[i] = val;
                }
            })
        }
    }

    // Create the structure containing the ODEs.
    let system = DifferentalSystem {eq_system: system};

    // Create a stepper and run the integration.
    let mut stepper = Dop853::new(system, 0., 800.0, 1.0, y0, 1.0e-2, 1.0e-6);
    let res = stepper.integrate();

    // Handle result.
    match res {
        Ok(stats) => println!("{}", stats),
        Err(e) => println!("An error occured: {}", e),
    }

    let y_co = stepper.y_out().to_owned();
    let mut series1: Vec<f64> = vec![];
    let mut series2: Vec<f64> = vec![];
    let mut x: Vec<f64> = (0..800).map(f64::from).collect();

    for x in y_co.iter() {
        series1.push(x[0]);
        series2.push(x[1]);
    }

    let series1_data: Vec<(f64, f64)> = x.clone().into_iter().zip(series1.into_iter()).collect();
    let series2_data: Vec<(f64, f64)> = x.into_iter().zip(series2.into_iter()).collect();
    let series_return = vec![series1_data, series2_data];
    Ok(ODE_Results { series_return })
}


#[pyclass]
struct ODE_Results {
    #[pyo3(get, set)]
    series_return: Vec<Vec<(f64, f64)>>
}

/// A Python module implemented in Rust.
#[pymodule]
fn better_ode(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve_diffeq_system, m)?)?;
    m.add_class::<ODE_Results>()?;
    Ok(())
}

// Chemical reaction of Robertson.
// This ode is stiff and is used to test the automatic stiffness detection in dopri5 and/or dop853.

use ode_solvers::dop853::*;
use ode_solvers::*;
use std::{fs::File, io::BufWriter, io::Write, path::Path};


type Time = f64;

use plotters::prelude::*;

fn main() {
    // Initial state.
    // let y0 = State::new(500.0, 200.0);

    // // Create the structure containing the ODEs.
    // let system = ChemicalReaction;

    // // Create a stepper and run the integration.
    // let mut stepper = Dop853::new(system, 0., 800.0, 1.0, y0, 1.0e-2, 1.0e-6);
    // let res = stepper.integrate();

    // // Handle result.
    // match res {
    //     Ok(stats) => println!("{}", stats),
    //     Err(e) => println!("An error occured: {}", e),
    // }


    // let root = BitMapBackend::new("test1.png", (640, 480)).into_drawing_area();
    // root.fill(&WHITE);
    // let root = root.margin(10, 10, 10, 10);
    // // After this point, we should be able to draw construct a chart context
    // let mut chart = ChartBuilder::on(&root)
    //     // Set the caption of the chart
    //     .caption("This is our first plot", ("sans-serif", 40).into_font())
    //     // Set the size of the label region
    //     .x_label_area_size(20)
    //     .y_label_area_size(40)
    //     // Finally attach a coordinate on the drawing area and make a chart context
    //     .build_cartesian_2d(0f64..800f64, 0f64..2300f64)?;

    // chart.configure_mesh()
    //     .x_labels(20)
    //     .y_labels(10)
    //     .x_desc("Time (hours")
    //     .y_desc("Population")
    //     .disable_mesh()
    //     .x_label_formatter(&|v| format!("{:.1}", v))
    //     .y_label_formatter(&|v| format!("{:.1}", v))
    //     .draw()?;

    // let y_co = stepper.y_out().to_owned();

    

    // let mut series1: Vec<f64> = vec![];
    // let mut series2: Vec<f64> = vec![];
    // let mut x: Vec<f64> = (0..800).map(f64::from).collect();

    // for x in y_co.iter() {
    //     series1.push(x[0]);
    //     series2.push(x[1]);
    // }

    // let series1_data: Vec<(f64, f64)> = x.clone().into_iter().zip(series1.into_iter()).collect();
    // let series2_data: Vec<(f64, f64)> = x.into_iter().zip(series2.into_iter()).collect();
    // let color2 = Palette99::pick(1).mix(0.9);
    // chart.draw_series(LineSeries::new(
    //     series1_data,
    //     &RED,
    // ))?
    // .label("Rabbits")
    // .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color2.filled()));
    // let color = Palette99::pick(0).mix(0.9);
    // chart.draw_series(LineSeries::new(
    //     series2_data,
    //     &BLUE
    // ))?
    // .label("Foxes")
    // .legend(move |(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], color.filled()));

    // chart
    //     .configure_series_labels()
    //     .border_style(&BLACK)
    //     .draw()?;

    // root.present()?;
    // Ok(())
}

