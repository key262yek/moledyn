// Confirm the Random number generators

use plotlib::page::Page;
use plotlib::repr::{Plot, Histogram, HistogramBins};
use plotlib::style::{BoxStyle, LineStyle};
use plotlib::view::ContinuousView;
use rts::random_mod::{rng_seed, get_uniform, get_gaussian};
use std::f64::consts::PI;

fn bin_bounds(min: f64, max: f64, count: usize) -> HistogramBins{
    // 원하는 범위와 원하는 칸 개수를 정의하기 위한 함수
    // 그냥 Histogram crate의 기능을 이용하면 그래프마다 정확한 계산을 할 수 없음.
    let range = max - min;
    let mut bounds: Vec<f64> = (0..count)
        .map(|n| (n as f64 / count as f64) * range + min)
        .collect();
    bounds.push(max);
    return HistogramBins::Bounds(bounds);
}

#[test]
#[ignore]
fn histogram_uniform(){
    // Draw histogram of uniform random number generator
    let mut rng = rng_seed(123141231412);

    let mut data : Vec<f64> = Vec::new();
    for _i in 0..1_000_000{
        data.push(get_uniform(&mut rng));
    }
    let n : usize = 1000;
    let (min, max) = (0f64, 1f64);
    let hist = Histogram::from_slice(&data, bin_bounds(min, max, n))
                .style(&BoxStyle::new().fill("burlywood"));
    let v = ContinuousView::new().add(hist).x_range(min, max);

    Page::single(&v).save("tests/images/histogram_uniform.svg").expect("saving svg");
}

#[test]
#[ignore]
fn histogram_uniform_c() -> std::io::Result<()>{
    // Draw histogram of uniform random number generator in C
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    // Read file
    let file = File::open("tests/images/histogram_uniform.dat")?;
    let reader = BufReader::new(file);

    let mut data : Vec<f64> = Vec::new();

    for line in reader.lines(){
        let x : f64 = line?.parse::<f64>().expect("Not a number");
        data.push(x);
    }

    // Draw Histogram
    let n : usize = 1000;
    let (min, max) = (0f64, 1f64);
    let hist = Histogram::from_slice(&data, bin_bounds(min, max, n))
                .style(&BoxStyle::new().fill("burlywood"));
    let v = ContinuousView::new().add(hist).x_range(min, max);

    Page::single(&v).save("tests/images/histogram_uniform_C.svg").expect("saving svg");
    Ok(())
}

#[test]
#[ignore]
fn histogram_gaussian(){
    // Draw histogram of gaussian random number generator
    let mut rng = rng_seed(123141231412);

    // Draw a histogram of get_gaussian
    let mut data : Vec<f64> = Vec::new();

    let n : usize = 100;
    let (min, max) = (-4f64, 4f64);
    let dx : f64 = (max - min) / (n as f64);
    let dh : f64 = (1f64 / (2f64 * PI)).sqrt();


    for _i in 0..1_000_000{
        let x : f64 = get_gaussian(&mut rng);
        if x < min{
            data.push(min + (0.5 / n as f64));
        }
        else if max < x{
            data.push(max - (0.5 / n as f64));
        }
        else{
            data.push(x);
        }
    }
    let hist : Histogram = Histogram::from_slice(&data, bin_bounds(min, max, n))
                .style(&BoxStyle::new().fill("burlywood"));

    // Draw a real pdf curve of normal distribution
    let mut data_gaussian : Vec<(f64, f64)> = Vec::new();
    for i in 0..n{
        let x = (i as f64 + 0.5f64) * dx + min;
        let y = dh * (- x * x / 2f64).exp() * dx * 1_000_000f64;
        data_gaussian.push((x, y));
    }
    let gaussian : Plot = Plot::new(data_gaussian).line_style(LineStyle::new().colour("#35C788"));

    let v = ContinuousView::new().add(hist).add(gaussian).x_range(min, max).y_range(0f64, 2f64 * 1_000_000f64 * dh * dx);

    Page::single(&v).save("tests/images/histogram_gaussian.svg").expect("saving svg");
}

#[test]
#[ignore]
fn histogram_gaussian_c() -> std::io::Result<()>{
    // Draw histogram of uniform random number generator in C
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    // Read file
    let file = File::open("tests/images/histogram_gaussian.dat")?;
    let reader = BufReader::new(file);

    let mut data : Vec<f64> = Vec::new();

    let n : usize = 100;
    let (min, max) = (-4f64, 4f64);
    let dx : f64 = (max - min) / (n as f64);
    let dh : f64 = (1f64 / (2f64 * PI)).sqrt();

    for line in reader.lines(){
        let x : f64 = line?.parse::<f64>().expect("Not a number");
        if x < min{
            data.push(min + (0.5 / n as f64));
        }
        else if max < x{
            data.push(max - (0.5 / n as f64));
        }
        else{
            data.push(x);
        }
    }

    let hist : Histogram = Histogram::from_slice(&data, bin_bounds(min, max, n))
                .style(&BoxStyle::new().fill("burlywood"));

    // Draw a real pdf curve of normal distribution
    let mut data_gaussian : Vec<(f64, f64)> = Vec::new();
    for i in 0..n{
        let x = (i as f64 + 0.5f64) * dx - 4.0f64;
        let y = dh * (- x * x / 2.0f64).exp() * dx * 1_000_000f64;
        data_gaussian.push((x, y));
    }
    let gaussian : Plot = Plot::new(data_gaussian).line_style(LineStyle::new().colour("#35C788"));

    let v = ContinuousView::new().add(hist).add(gaussian).x_range(min, max).y_range(0f64, 2f64 *  1_000_000f64 * dh * dx);

    Page::single(&v).save("tests/images/histogram_gaussian_C.svg").expect("saving svg");
    Ok(())
}

