use markov_text::MarkovTextModel;

fn main() {
    let sample_texts = ["Math is dumb", "Timo is cool"]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    let mut model = MarkovTextModel::<4>::new();
    model.add_sample_texts(&sample_texts);

    for _ in 0..200 {
        let message = model.generage_text();

        println!("{message}");
    }
}
