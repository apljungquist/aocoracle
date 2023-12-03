mod textarea;

use yew::prelude::*;

trait ToEnglish {
    fn to_english(&self) -> String;
}

impl ToEnglish for aoclib::Part {
    fn to_english(&self) -> String {
        match self {
            aoclib::Part::One => "One",
            aoclib::Part::Two => "Two",
        }
        .into()
    }
}

enum Msg {
    Run,
    RemoveAnswer,
    Update(String),
    SetPart(aoclib::Part),
}

struct Model {
    input: String,
    part: aoclib::Part,
    options_changed: bool,
    answers: Option<Vec<String>>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input: "16,1,2,0,4,2,7,1,2,19".into(),
            part: aoclib::Part::One,
            options_changed: true,
            answers: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::RemoveAnswer => {}
            Msg::Run => self.options_changed = false,
            _ => self.options_changed = true,
        }
        match msg {
            Msg::RemoveAnswer => {
                if let Some(answers) = self.answers.as_mut() {
                    answers.remove(0);
                } else {
                    log::error!("This should not be possible in the UI");
                }
            }
            Msg::Run => {
                self.answers = None;
                let args = aoclib::Cli::new(None, None, Some(self.part), true);
                match aoclib::helper(&args, &self.input) {
                    Ok(answers) => {
                        self.answers = Some(answers.into_values().collect());
                    }
                    Err(error) => {
                        log::error!("Oracle failed with error {}", error)
                    }
                };
            }
            Msg::SetPart(part) => self.part = part,
            Msg::Update(content) => self.input = content,
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = ctx.link().callback(Msg::Update);
        let answer: Html = {
            if let Some(answers) = &self.answers {
                if let Some(answer) = answers.iter().next() {
                    html! {
                        <span>{format!( "The answer is {}", answer) }</span>
                    }
                } else {
                    html! {
                        <span>{ r"¯\_(ツ)_/¯" }</span>
                    }
                }
            } else {
                html! {
                    <span>{ "..." }</span>
                }
            }
        };
        let button: Html = {
            if self.options_changed || self.answers.is_none() {
                html!(
                        <button onclick={ctx.link().callback(|_| Msg::Run)} >
                            { "Tell us the answer!" }
                        </button>
                )
            } else {
                let answers = self.answers.as_ref().expect("None is handled in if above");
                html!(
                        <button onclick={ctx.link().callback(|_| Msg::RemoveAnswer)} disabled={answers.len()<2}>
                            { "Tell us another one!" }
                        </button>
                )
            }
        };

        html! {
            <div class="column">
                <div class="row" id="input-row">
                    <textarea::TextInput{on_change} value={self.input.clone()}/>
                </div>
                <div class="row" id="options-row">
                    <label>
                        <input
                            id="radio_part_one"
                            name="part" type="radio"
                            checked={self.part==aoclib::Part::One}
                            onclick={ctx.link().callback(|_| Msg::SetPart(aoclib::Part::One))}
                        />
                        <span>{ "Part One" }</span>
                    </label>
                    <label>
                        <input
                            id="radio_part_two"
                            name="part" type="radio"
                            checked={self.part==aoclib::Part::Two}
                            onclick={ctx.link().callback(|_| Msg::SetPart(aoclib::Part::Two))}
                        />
                        <span>{ "Part Two" }</span>
                    </label>
                </div>
                <div class="row" id="button-row">
                    {button}
                </div>
                <div class="row" id="answer-row">
                    {answer}
                </div>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
