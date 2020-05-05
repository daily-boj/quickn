/*
    date   : 2020 / 5 / 5
    author : quickn (quickn.ga)
    email  : quickwshell@gmail.com
*/

mod aho_corasick {
    use std::collections::{HashMap, VecDeque};
    use std::hash::Hash;

    #[derive(Clone, Copy, Hash, Debug, Eq, PartialEq)]
    pub struct State(i32);

    const FAILED: State = State(-1);
    const ZERO_STATE: State = State(0);

    impl State {
        fn increase(&mut self) {
            self.0 += 1;
        }

        fn is_zero(&self) -> bool {
            *self == ZERO_STATE
        }

        fn is_failed(&self) -> bool {
            *self == FAILED
        }
    }

    #[derive(Debug, Clone)]
    pub struct GotoFunction {
        data: HashMap<(State, char), State>,
        output: HashMap<State, String>,
        alphabets: Vec<char>,
    }

    impl GotoFunction {
        fn new(keywords: Vec<String>, alphabets: Vec<char>) -> Self {
            let mut hash: HashMap<(State, char), State> = HashMap::new();
            let mut output: HashMap<State, String> = HashMap::new();
            //output.insert(ZERO_STATE, String::new());
            let mut new_state = ZERO_STATE;
            for keyword in keywords {
                let key: Vec<char> = keyword.chars().collect();
                let (mut state, mut j): (State, usize) = (ZERO_STATE, 0);
                while let Some(&func) = hash.get(&(state, key[j])) {
                    state = func;
                    j += 1;
                    if j >= key.len() {
                        break;
                    }
                }
                for &word in key.get(j..key.len()).unwrap() {
                    new_state.increase();
                    hash.insert((state, word), new_state);
                    state = new_state;
                }
                output.insert(state, keyword);
            }
            for &alphabet in &alphabets {
                hash.insert((FAILED, alphabet), ZERO_STATE);
                if let None = hash.get(&(ZERO_STATE, alphabet)) {
                    hash.insert((ZERO_STATE, alphabet), ZERO_STATE);
                }
            }
            for p_state in 1..=new_state.0 {
                for &alphabet in &alphabets {
                    if let None = hash.get(&(State(p_state), alphabet)) {
                        hash.insert((State(p_state), alphabet), FAILED);
                    }
                }
            }
            Self {
                data: hash,
                output: output,
                alphabets: alphabets,
            }
        }

        fn goto(&self, state: State, word: char) -> Option<&State> {
            self.data.get(&(state, word))
        }

        fn output(&self, state: State) -> Option<&String> {
            self.output.get(&state)
        }
    }

    #[derive(Clone, Debug)]
    pub struct FailureFunction {
        data: HashMap<State, State>,
    }

    impl FailureFunction {
        fn new(goto: &mut GotoFunction) -> Self {
            let mut hash: HashMap<State, State> = HashMap::new();
            hash.insert(FAILED, ZERO_STATE);
            hash.insert(ZERO_STATE, ZERO_STATE);
            let mut q: VecDeque<State> = VecDeque::new();
            for alphabet in goto.alphabets.clone() {
                let tmp = *goto.goto(ZERO_STATE, alphabet).unwrap();
                if !tmp.is_zero() {
                    q.push_back(tmp);
                    hash.insert(tmp, ZERO_STATE);
                }
            }
            while let Some(r) = q.pop_front() {
                for alphabet in goto.alphabets.clone() {
                    let tmp = *goto.goto(r, alphabet).unwrap();
                    if !tmp.is_failed() {
                        q.push_back(tmp);
                        let mut state = *hash.get(&r).unwrap();
                        while (*goto.goto(state, alphabet).unwrap()).is_failed() {
                            state = *hash.get(&state).unwrap();
                        }
                        hash.insert(tmp, *goto.goto(state, alphabet).unwrap());
                        if let Some(output) = goto.output(*hash.get(&tmp).unwrap()) {
                            if let Some(output2) = goto.output(tmp) {
                                goto.output.insert(tmp, output2.clone() + &output.clone());
                            } else {
                                goto.output.insert(tmp, output.clone());
                            }
                        }
                    }
                }
            }
            Self { data: hash }
        }

        fn fail(&self, state: State) -> Option<&State> {
            self.data.get(&state)
        }
    }

    #[derive(Clone, Debug)]
    pub struct PatternMatching {
        goto: GotoFunction,
        fail: FailureFunction,
    }

    impl PatternMatching {
        pub fn new(keywords: Vec<String>, alphabets: Vec<char>) -> Self {
            let mut goto = GotoFunction::new(keywords, alphabets);
            let fail = FailureFunction::new(&mut goto);
            Self { goto, fail }
        }

        pub fn matching(&self, s: String) -> Option<String> {
            let mut state = ZERO_STATE;
            for c in s.chars() {
                while (*self.goto.goto(state, c).unwrap()).is_failed() {
                    state = *self.fail.fail(state).unwrap();
                }
                state = *self.goto.goto(state, c).unwrap();
                if let Some(output) = self.goto.output(state) {
                    return Some(output.clone());
                }
            }
            return None;
        }
    }
}