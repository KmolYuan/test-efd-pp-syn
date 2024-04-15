use four_bar::{
    csv, efd, mh,
    plot::{self, *},
    syn::{FbPPSyn, FbSyn, Mode},
};

fn syn_test<F, P: Clone + mh::MaybeParallel + 'static>(i: usize, func: F) -> P
where
    F: mh::ObjFunc<Ys = mh::WithProduct<f64, P>>,
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
            history.push(ctx.best.get_eval());
            pb.set_position(ctx.gen);
        })
        .solve();
    pb.finish();
    println!("Time [{i}]: {:.4?}", t0.elapsed());
    let path = format!("history_{i}.svg");
    plot::fb::history(SVGBackend::new(&path, (800, 800)), history).unwrap();
    s.into_result()
}

fn main() {
    // [Ref 1] ../four-bar-rs/test-fb/yu2.closed.csv
    // [Ref 2] ./crunode.closed.csv
    let w = std::fs::File::open("./crunode.closed.csv").unwrap();
    let target = csv::from_reader(w).unwrap();
    let target_efd = efd::Efd2::from_curve(&target, false);
    let fb = syn_test(1, FbPPSyn::from_curve(&target, Mode::Closed));
    let fb_str = ron::ser::to_string_pretty(&fb, Default::default()).unwrap();
    std::fs::write("syn_1.ron", fb_str).unwrap();
    let curve1 = fb.curve(180);
    println!(
        "Error [1]: {:.4?}",
        target_efd.err(&efd::Efd2::from_curve(&curve1, false))
    );
    let func = FbSyn::from_curve(&target, Mode::Closed);
    println!("Harmonic: {:?}", func.harmonic());
    let fb = syn_test(2, func);
    let fb_str = ron::ser::to_string_pretty(&fb, Default::default()).unwrap();
    std::fs::write("syn_2.ron", fb_str).unwrap();
    let curve2 = fb.curve(180);
    println!(
        "Error [2]: {:.4?}",
        target_efd.err(&efd::Efd2::from_curve(&curve2, false))
    );
    plot::fb::Figure::new(None)
        .legend(LegendPos::UL)
        .add_line("Target", &target, Style::Circle, RED)
        .add_line("P-P. Optimized", &curve1, Style::Line, BLUE)
        .add_line(
            "Optimized",
            &curve2,
            Style::DashedLine,
            full_palette::ORANGE_900,
        )
        .plot(SVGBackend::new("syn.svg", (1600, 1600)))
        .unwrap();
}
