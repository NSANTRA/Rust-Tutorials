// Structure give more modularity and clarity rather than normal data types or tuples

fn calc1(parameters: &Parameters) -> usize {
    parameters.height * parameters.width
}

fn calc2(dimensions: (usize, usize)) -> usize {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Parameters {
    height: usize,
    width: usize
}

fn main() {
    let params: Parameters = Parameters {
        height: 20,
        width: 30
    };

    println!(
        "Area of rectangle with height: {} and width: {} is {}",
        params.height,
        params.width,
        calc1(&params)
    );

    println!(
        "Area of rectangle with height: {} and width: {} is {}",
        params.height,
        params.width,
        calc2((params.height, params.width))
    );

    println!(
        "The structure used here is {params:#?}",
    )
}