use four_bar::{
    csv, mh,
    plot::{self, *},
    syn::{FbPPSyn, FbSyn, Mode},
};

fn syn_test<F, P>(i: usize, func: F) -> P
where
    F: mh::ObjFunc<Fitness = mh::Product<P, f64>>,
{
    const GEN: u64 = 50;
    let t0 = std::time::Instant::now();
    let mut history = Vec::with_capacity(GEN as usize);
    let pb = indicatif::ProgressBar::new(GEN);
    let s = mh::Solver::build(mh::De::default(), func)
        .seed(0)
        .pop_num(200)
        .task(|ctx| ctx.gen == GEN)
        .callback(|ctx| {
            history.push(ctx.best_f.fitness());
            pb.set_position(ctx.gen);
        })
        .solve()
        .unwrap();
    pb.finish();
    println!("Time [{i}]: {:?}", t0.elapsed());
    let path = format!("history_{i}.svg");
    plot::fb::history(SVGBackend::new(&path, (800, 800)), history).unwrap();
    s.into_result()
}

fn main() {
    // [Ref] ../four-bar-rs/test-fb/slice.open.csv
    let w = std::fs::File::open("test3.csv").unwrap();
    let target = csv::from_reader(w).unwrap();
    let curve1 = syn_test(1, FbPPSyn::from_curve(&target, Mode::Closed)).curve(90);
    let curve2 = syn_test(2, FbSyn::from_curve(&target, Mode::Closed)).curve(90);
    plot::fb::Figure::new(None)
        .legend(LegendPos::UL)
        .add_line("Target", &target, Style::Circle, RED)
        .add_line("P-P. Optimized", &curve1, Style::Line, BLUE)
        .add_line("Optimized", &curve2, Style::Line, full_palette::ORANGE_900)
        .plot(SVGBackend::new("syn.svg", (1600, 1600)))
        .unwrap();
}
