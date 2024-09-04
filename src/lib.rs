use std::{collections::HashMap, hash::Hash};
use rand::Rng;

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
enum Token {
    Start,
    Text(String),
    End
}

struct TokenSampler {
    token_count: usize,
    token_to_token_count: HashMap<Token, usize>
}

impl TokenSampler {
    fn new() -> TokenSampler {
        TokenSampler {
            token_count: 0,
            token_to_token_count: HashMap::new()
        }
    }

    fn add_token(&mut self, token: Token) {
        self.token_count += 1;
        *self.token_to_token_count.entry(token).or_insert(0) += 1;
    }

    fn sample(&self) -> Token {
        let mut random_index = rand::thread_rng().gen_range(0..self.token_count);
        for (token, count) in &self.token_to_token_count {
            if &random_index >= count {
                random_index -= count;
            } else {
                return token.clone();
            }
        }
        unreachable!("The token count does not match the tokens in the map");
    }
}

pub struct MarkovTextModel<const CONTEXT_LENGTH: usize = 1> {
    token_to_token_sampler: HashMap<[Token; CONTEXT_LENGTH], TokenSampler>
}

impl<const CONTEXT_LENGTH: usize> MarkovTextModel<CONTEXT_LENGTH> {

    /// Creates an empty character based MarkovTextModel, where the `CONTEXT_LENGTH` parameter is the context length in characters.
    pub fn new() -> MarkovTextModel<CONTEXT_LENGTH> {
        assert!(CONTEXT_LENGTH > 0, "The context length must be greater than 0");
        MarkovTextModel::<CONTEXT_LENGTH> {
            token_to_token_sampler: HashMap::new()
        }
    }
    
    pub fn add_sample_text(&mut self, text: &str) {
        self.add_tokenized_sample_text(&self.tokenize(text));
    }

    pub fn add_sample_texts(&mut self, texts: &Vec<&str>) {
        for text in texts {
            self.add_sample_text(text);
        }
    }

    pub fn generage_text(&self) -> String {
        let mut current_context = [const { Token::Start }; CONTEXT_LENGTH];
        let mut cumulative_text = "".to_owned();

        loop {
            let next_token = self.token_to_token_sampler[&current_context].sample();

            match &next_token {
                Token::Start => unreachable!("Start token should not be reachable after Start token"),
                Token::End => return cumulative_text,
                Token::Text(current_token_text) => cumulative_text = cumulative_text + &current_token_text
            }

            for i in 1..CONTEXT_LENGTH {
                current_context[i - 1] = current_context[i].clone();
            }
            current_context[CONTEXT_LENGTH - 1] = next_token;
        }
    }
    
    // TODO: Make this more user definable. Maybe with an optional closure when constructing the model?
    fn tokenize(&self, text: &str) -> Vec<Token> {
        let mut tokens = vec![Token::Start; CONTEXT_LENGTH];
        tokens.extend(text.chars().map(|letter| Token::Text(letter.to_string())));
        // tokens.extend(text.split_whitespace().map(|word| Token::Word(word.to_owned())));
        tokens.push(Token::End);
        tokens
    }

    fn add_tokenized_sample_text(&mut self, tokenized_text: &Vec<Token>) {
        for window in tokenized_text.windows(CONTEXT_LENGTH + 1) {

            let mut context = [const { Token::Start }; CONTEXT_LENGTH];

            for i in 0..CONTEXT_LENGTH {
                context[i] = window[i].clone();
            }

            let next_token = &window[CONTEXT_LENGTH];

            self.token_to_token_sampler.entry(context).or_insert(TokenSampler::new()).add_token(next_token.clone());
        }
    }
}
