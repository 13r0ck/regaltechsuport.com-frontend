use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::format::{Text, Nothing};
use easy_hasher::easy_hasher::*;
use rand::prelude::*;
use rand_seeder::Seeder;
use rand_pcg::Pcg64;
use rand::seq::SliceRandom;

struct PasswdGen {
    passwd: Option<String>,
    // Hashed (load/click/delta_fetch) time as seed for the password
    site_load_time: Option<f64>,
    button_press_time: Option<f64>,
    delta_fetch: Option<f64>,
    words: Option<Vec<String>>,
}

struct State {
    passwd_gen: PasswdGen,
    contact_one_state: String,
    contact_two_state: String,
    support_two_state: String,
    password_state: String,
    password_value: String,
}

pub struct Home {
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

pub enum Msg {
    CreatePasswd,
    StartFetch,
    GetWordsSuccess(String),
    GetWordsError,
    GetSupport1,
    GetSupport2,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let now = js_sys::Date::new_0().get_time();
        link.send_message(Msg::StartFetch);
        Self {
            state: State {
                passwd_gen: PasswdGen {
                    passwd: None,
                    site_load_time: Some(now),
                    button_press_time: None,
                    delta_fetch: None,
                    words: None,
                },
                contact_one_state: "hidden".to_string(),
                contact_two_state: "hidden".to_string(),
                support_two_state: "inline".to_string(),
                password_state: "invisible".to_string(),
                password_value: "Click Generate!".to_string(),
            },
            link,
            task: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::StartFetch => {
                // grab the list of words via a fetch
                let get_request = Request::get("/sections/passwd/words.txt")
                    .body(Nothing)
                    .expect("Failed get request");
                let callback = self.link.callback(|response: Response<Text>| {
                    if let (_, Ok(body)) = response.into_parts() {
                        return Msg::GetWordsSuccess(body);
                    } else {
                        Msg::GetWordsError
                    }
                });
                self.task = Some(FetchService::fetch(get_request, callback).unwrap());
                true
            },
            Msg::CreatePasswd => {
                let now = js_sys::Date::new_0().get_time();
                self.state.passwd_gen.button_press_time = Some(now);
                // b/c rand reliablilty on wasm is spotty we cannot use system
                // seed sources to create the seed. Rather we create one from the hash
                // from physical values, time (page load time), and virtual distance
                // from server (delta time to fetch from server). And time of button
                // press so that hash is different every time
                let seed = keccak512(&format!("{}{}{}", 
                                        &self.state.passwd_gen.button_press_time.as_ref().unwrap(),
                                        &self.state.passwd_gen.site_load_time.as_ref().unwrap(),
                                        &self.state.passwd_gen.delta_fetch.as_ref().unwrap())).to_hex_string();
                // generate random object (rng) with seed
                let mut rng: Pcg64 = Seeder::from(seed).make_rng();
                let mut rand_num_list: Vec<String> = vec!["".to_string(),"".to_string(),rng.gen_range(0,10).to_string()];
                rand_num_list.shuffle(&mut rng);
                let word_list = self.state.passwd_gen.words.as_ref().unwrap()[33..].to_vec();
                let seperator = "@%+\\/!#$^?:,(){}[]~-_.\"".chars().choose(&mut rng).unwrap();
                let mut three_words: [String; 3] = [
                                    word_list.choose(&mut rng).unwrap().clone(),
                                    word_list.choose(&mut rng).unwrap().clone(),
                                    word_list.choose(&mut rng).unwrap().clone()];
                let tw_index = rng.gen_range(0,3);
                three_words[tw_index] = three_words[tw_index].to_uppercase();
                self.state.passwd_gen.passwd = Some(format!("{}{}{}{}{}{}{}{}",
                        three_words[0],
                        rand_num_list[0],
                        seperator,
                        three_words[1],
                        rand_num_list[1],
                        seperator,
                        three_words[2],
                        rand_num_list[2]
                ));
                self.state.password_value = self.state.passwd_gen.passwd.clone().unwrap();
                self.state.password_state = "visible".to_string();
                true
            },
            Msg::GetWordsSuccess(words_txt) => {
                // if the fetch is successfull with grabbing words from server
                let now = js_sys::Date::new_0().get_time();
                if let Some(site_load_time) = self.state.passwd_gen.site_load_time {
                    let delta_fetch = now - site_load_time;
                    self.state.passwd_gen.delta_fetch = Some(delta_fetch);
                }
                self.state.passwd_gen.words = Some(words_txt.lines()
                                        .map(|s| s.to_string())
                                        .collect());
                true
            },
            Msg::GetWordsError => {
                // There was some error, I would rather not show the password generator
                // than to show a possible insecure one.
                println!("There was an error generating the password. Password gen hidden");
                false
            },
            Msg::GetSupport1 => {
                self.state.contact_one_state = "inline-block".to_string();
                self.state.contact_two_state = "hidden".to_string();
                self.state.support_two_state = "inline".to_string();
                true
            },
            Msg::GetSupport2 => {
                self.state.contact_one_state = "hidden".to_string();
                self.state.contact_two_state = "inline-block".to_string();
                self.state.support_two_state = "hidden".to_string();
                true
            },
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
                <body>
                    <header class="hidden lg:inline w-screen fixed z-50 elevation-3">
                        <div class="blur flex">
                            <div class="align-center max-w-7xl flex-1 mx-auto">
                                <div class="float-left px-8 py-3 mx-auto">
                                    <h1 class="text-gray-900 font-bold text-xl flex my-auto">
                                        <img src="/img/email.svg" alt="email icon" class="pr-2 my-auto h-5"/>
                                        {"hello@regaltechsupport.com"}
                                    </h1>
                                </div>
                                <div class="float-right px-8 py-3 flex-1">
                                    <h1 class="text-gray-900 float-right font-bold text-xl flex my-auto">
                                        <img src="/img/phone.svg" alt="phone icon" class="pr-2 my-auto h-5"/>
                                        {"(719) 440 - 9462"}
                                    </h1>
                                </div>
                            </div>
                        </div>
                    </header>
                    <main>
                        <div class="flex min-h-jumbotron h-screen justify-center">
                            <div class="z-10 absolute flex">
                                <div class="mt-10 xl:mt-24 h-96">
                                    <img class="w-20 sm:w-96 lg:w-56 xl:w-48 mx-auto" src="/img/logo.svg" alt="Crown logo"/>
                                    <h1 class="text-4xl md:text-7xl xl:text-6xl text-gray-800 mx-auto text-center mt-3 lg:mt-4 font-rtsBold">{"Regal Tech Support"}</h1>
                                    <h2 class="text-xl md:text-4xl lg:text-3xl xl:text-2xl text-gray-800 mx-auto mt-1 lg:mt-4 font-rts text-center">{"IT support at home and at work "}<br class="inline xl:hidden"/>{"in Colorado Springs"}</h2>
                                    <div class="flex">
                                    <a href="#contact1" onclick=self.link.callback(move |_| Msg::GetSupport1) class="text-xl sm:text-3xl xl:text-2xl focus:outline-none py-2 px-5 py-4 px-8 mx-auto justify-center mt-8 text-white font-rtsBold uppercase tracking-wide rounded-md items-center elevation-10 hover:from-blue-300 bg-gradient-to-tr active:bg-gradient-to-bl from-green-300 via-blue-500 to-purple-600">{"GET SUPPORT"}</a>
                                    </div>
                                </div>
                            </div>
                            <img class="z-0 w-screen min-h-jumbotron h-screen object-cover object-center" src="/img/jumbotron.svg" alt="Castle in mountains"/>
                        </div>
                        <div class="max-w-7xl px-1 lg:px-20 md:mx-auto overflow-hidden">
                            <div id="contact1" class=format!("bg-polygons-green rounded-bl-lg w-full rounded-br-lg bg-cover {}", &self.state.contact_one_state)>
                                <form action="https://formspree.io/f/xgeplqze" method="POST" class="text-center p-1 lg:p-2 lg:p-5 lg:p-10 elevation-24 m-3 lg:m-10 rounded-lg bg-white">
                                    <div class="flex m-1 lg:m-3">
                                        <p class="float-left text-xl sm:text-3xl lg:text-3xl xl:text-2xl w-44 lg:w-32 xl:w-24 text-left bg-indigo-700 text-white p-2 border-4 border-indigo-700 rounded m-1 font-rtsBold uppercase tracking-wide">{"To:"}</p>
                                        <p class="float-left text-xl sm:text-3xl lg:text-3xl xl:text-2xl text-left bg-indigo-700 text-white p-2 border-4 flex-grow border-indigo-700 rounded m-1 font-rts">{"hello@regaltechsupport.com"}</p>
                                    </div>
                                    <div class="flex clear-both m-1 lg:m-3">
                                        <p class="float-left w-20 text-xl sm:text-3xl lg:text-3xl xl:text-2xl w-44 lg:w-32 xl:w-24 text-left bg-indigo-700 text-white p-2 rounded border-4 border-indigo-700 m-1 font-rtsBold uppercase tracking-wide">{"Name:"}</p>
                                        <input required={true} name="name" class="text-xl sm:text-3xl lg:text-3xl xl:text-2xl float-left p-2 flex-grow m-1 resize-none border-4 rounded-lg border-indigo-700 focus:outline-none focus:border-indigo-600" type="text" placeholder="What is your name?"/><br/>
                                    </div>
                                    <div class="flex clear-both m-1 lg:m-3">
                                        <p class="float-left text-xl sm:text-3xl lg:text-3xl xl:text-2xl w-44 lg:w-32 xl:w-24 text-left bg-indigo-700 text-white p-2 rounded border-4 border-indigo-700 m-1 font-rtsBold uppercase tracking-wide">{"Email:"}</p>
                                        <input required={true} pattern="[a-z0-9._%+-]+@[a-z0-9.-]+\\.[a-z]{2,63}$" name="_replyto" class="text-xl sm:text-3xl lg:text-3xl xl:text-2xl float-left p-2 flex-grow m-1 resize-none border-4 rounded-lg border-indigo-700 focus:outline-none focus:border-indigo-600" type="email" placeholder="example@email.com"/><br/>
                                    </div>
                                    <div class="flex clear-both m-1 lg:m-3">
                                        <textarea required={true} name="message" class="text-xl sm:text-3xl lg:text-3xl xl:text-2xl float-left p-2 flex-grow m-1 resize-none border-4 rounded-lg border-indigo-700 focus:outline-none focus:border-indigo-600" rows=5 placeholder="How can we help?"></textarea>
                                    </div>
                                    <div class="flex clear-both m-1 lg:m-3">
                                        <input class="focus:outline-none p-5 text-xl sm:text-3xl lg:text-3xl xl:text-2xl py-5 px-8 mx-auto flex mt-8 text-gray-100 font-rtsBold uppercase tracking-wide rounded-md justify-center w-full bg-gradient-to-tr hover:from-green-400 active:bg-gradient-to-bl from-green-500 to-green-700" type="submit" value="SEND!" />
                                    </div>
                                        <h3 class="text-xl sm:text-3xl lg:text-3xl xl:text-2xl pt-1 font-rtsBold">{"Or call/text us at "}<br class="inline lg:hidden" /><span>{"(719) 440 - 9462"}</span>{"!"}</h3>
                                </form>
                            </div>
                            <div class="inline-block mx-5 my-10">
                                <h2 class="text-xl sm:text-6xl lg:text-4xl xl:text-5xl font-rtsBold text-gray-800 text-right">{"What Makes Us Different?"}</h2>
                                <div class="flex-row mt-3">
                                    <img class="lg:w-1/2 h-64 sm:h-80 lg:h-96 p-3 mx-auto lg:float-left" src="/img/jester_weld.svg" alt="Regal Tech Support Marcot welding laptop"/>
                                    <p class="lg:w-1/2 lg:float-right p-5 text-xl sm:text-3xl lg:text-2xl xl:text-2xl">{"Typical IT support is there only when things break. We prefer a proactive approach. By spending time to understand your situation we will create a reliable technology experience for you! We believe that reliability and security are possible with any budget!"}</p>
                                </div>
                            </div>
                            <div class="bg-polygons-gray rounded-full bg-top h-8 my-12 flex"></div>
                            <div class="inline-block mx-5 my-10">
                                <h2 class="text-xl sm:text-6xl lg:text-4xl xl:text-5xl font-rtsBold text-gray-800 text-left">{"What We Do"}</h2>
                                <img class="lg:w-1/2 h-64 sm:h-80 lg:h-96 p-3 mx-auto lg:float-right" src="/img/jester_bluescreen.svg" alt="Regal Tech Support Mascot using defibrillator on laptop"/>
                                <div class="lg:w-1/2 lg:float-left p-3 align-middle">
                                    <p class="text-xl sm:text-3xl lg:text-3xl xl:text-2xl">{"We build modern solutions for you at home and at work."}</p>
                                    <ul class="my-4 text-center lg:text-left lg:mx-1 md:mx-8">
                                        <li class="text-xl sm:text-3xl lg:text-3xl xl:text-2xl">{"WiFi not working?"}</li>
                                        <li class="text-xl sm:text-3xl lg:text-3xl xl:text-2xl">{"Printer won't print?"}</li>
                                        <li class="text-xl sm:text-3xl lg:text-3xl xl:text-2xl">{"Zoom cannot share screen?"}</li>
                                        <li class="text-xl sm:text-3xl lg:text-3xl xl:text-2xl">{"How to backup Windows?"}</li>
                                        <li class="text-xl sm:text-3xl lg:text-3xl xl:text-2xl">{"Worried about hackers?"}</li>
                                    </ul>
                                </div>
                                <p class="inline-block mt-2 p-5 text-xl sm:text-3xl lg:text-3xl xl:text-2xl">{"Regal Tech Support is reliable support for your household, family, or small buisness. Contact us and we will discuss possible solutions for free!"}</p>
                            </div>
                            <div class="bg-polygons-gray rounded-full bg-center h-8 my-12 flex"></div>
                            <div class="inline-block mx-5 my-10">
                                <h2 class="text-xl sm:text-6xl lg:text-4xl xl:text-5xl font-rtsBold text-gray-800 text-right">{"24/7 Support"}</h2>
                                <div class="mt-3">
                                    <img class="lg:w-1/2 h-64 sm:h-80 lg:h-96 p-3 mx-auto lg:float-left" src="/img/jester_fire.svg" alt="Laptop on fire"/>
                                    <p class="lg:w-1/2 lg:float-right p-5 text-xl sm:text-3xl lg:text-3xl xl:text-2xl lg:mb-5">{"Nobody gets to choose when things break. We will be there for you whenever and wherever you need us!"}</p>
                                </div>
                                <div class="flex clear-both">
                                    <a href="#contact2" onclick=self.link.callback(move |_| Msg::GetSupport2) class=format!("text-xl sm:text-3xl xl:text-2xl focus:outline-none py-2 px-5 py-4 px-8 mt-8 justify-center mx-auto text-white font-rtsBold uppercase tracking-wide rounded-md elevation-10 hover:from-blue-300 bg-gradient-to-tr active:bg-gradient-to-bl from-green-300 via-blue-500 to-purple-600 {}", &self.state.support_two_state)>{"GET SUPPORT"}</a>
                                </div>
                            </div>
                            <div id="contact2" class=format!("bg-polygons-gold rounded-lg w-full bg-cover {}", &self.state.contact_two_state)>
                                <form action="https://formspree.io/f/xgeplqze" method="POST" class="text-center p-1 lg:p-2 lg:p-5 lg:p-10 elevation-24 m-3 lg:m-10 rounded-lg bg-white">
                                    <div class="flex m-1 lg:m-3">
                                        <p class="float-left text-xl sm:text-3xl lg:text-3xl xl:text-2xl w-44 lg:w-32 xl:w-24 text-left bg-indigo-700 text-white p-2 border-4 border-indigo-700 rounded m-1 font-rtsBold uppercase tracking-wide">{"To:"}</p>
                                        <p class="float-left text-xl sm:text-3xl lg:text-3xl xl:text-2xl text-left bg-indigo-700 text-white p-2 border-4 flex-grow border-indigo-700 rounded m-1 font-rts">{"hello@regaltechsupport.com"}</p>
                                    </div>
                                    <div class="flex clear-both m-1 lg:m-3">
                                        <p class="float-left w-20 text-xl sm:text-3xl lg:text-3xl xl:text-2xl w-44 lg:w-32 xl:w-24 text-left bg-indigo-700 text-white p-2 rounded border-4 border-indigo-700 m-1 font-rtsBold uppercase tracking-wide">{"Name:"}</p>
                                        <input required={true} name="name" class="text-xl sm:text-3xl lg:text-3xl xl:text-2xl float-left p-2 flex-grow m-1 resize-none border-4 rounded-lg border-indigo-700 focus:outline-none focus:border-indigo-600" type="text" placeholder="What is your name?"/><br/>
                                    </div>
                                    <div class="flex clear-both m-1 lg:m-3">
                                        <p class="float-left w-20 text-xl sm:text-3xl lg:text-3xl xl:text-2xl w-44 lg:w-32 xl:w-24 text-left bg-indigo-700 text-white p-2 rounded border-4 border-indigo-700 m-1 font-rtsBold uppercase tracking-wide">{"Email:"}</p>
                                        <input required={true} pattern="[a-z0-9._%+-]+@[a-z0-9.-]+\\.[a-z]{2,63}$" name="_replyto" class="text-xl sm:text-3xl lg:text-3xl xl:text-2xl float-left p-2 flex-grow m-1 resize-none border-4 rounded-lg border-indigo-700 focus:outline-none focus:border-indigo-600" type="email" placeholder="example@email.com"/><br/>
                                    </div>
                                    <div class="flex clear-both m-1 lg:m-3">
                                        <textarea required={true} name="message" class="text-xl sm:text-3xl lg:text-3xl xl:text-2xl float-left p-2 flex-grow m-1 resize-none border-4 rounded-lg border-indigo-700 focus:outline-none focus:border-indigo-600" rows=5 placeholder="How can we help?"></textarea>
                                    </div>
                                    <div class="flex clear-both m-1 lg:m-3">
                                        <input class="focus:outline-none p-5 text-xl sm:text-3xl lg:text-3xl xl:text-2xl py-5 px-8 mx-auto flex mt-8 text-gray-100 font-rtsBold uppercase tracking-wide rounded-md justify-center w-full bg-gradient-to-tr hover:from-green-400 active:bg-gradient-to-bl from-green-500 to-green-700" type="submit" value="SEND!" />
                                    </div>
                                        <h3 class="text-xl sm:text-3xl lg:text-3xl xl:text-2xl pt-1 font-rtsBold">{"Or call/text us at "}<br class="inline lg:hidden" /><span>{"(719) 440 - 9462"}</span>{"!"}</h3>
                                </form>
                            </div>
                            <div class="bg-polygons-gray rounded-full bg-left-bottom h-8 my-12 flex"></div>
                                <div id="tools" style="background-size: cover; background-repeat: no-repeat" class="bg-polygons-indigo flex-row rounded-lg my-10">
                                    <div class="flex">
                                        <div class="p-5 bg-white flex-row mx-auto ml-10 float-left rounded-bl-lg rounded-br-lg elevation-5">
                                            <h2 class="text-xl sm:text-6xl lg:text-4xl xl:text-5xl font-rtsBold text-gray-800 text-left">{"Useful Tools!"}</h2>
                                            <p class="p-5 text-xl sm:text-3xl lg:text-3xl xl:text-2xl font-rts text-left">{"Always free, enjoy!"}</p>
                                        </div>
                                    </div>
                                    <div class="inline-block">
                                        <div class="block w-full xl:flex xl:float-left xl:w-1/2">
                                            <div class="p-10">
                                                <div id="password_gen" class="bg-white rounded-lg elevation-24 text-center p-10">
                                                    <div class="">
                                                        <h2 class="font-rts text-xl sm:text-6xl lg:text-4xl xl:text-3xl p-2 underline">{"Secure Password Generator"}</h2>
                                                        <p class="text-xl sm:text-3xl lg:text-3xl xl:text-2xl">{"Should it have a number? "}<br class="visible lg:invisible" />{"What about replacing an "}<span class="italic bg-gray-300 rounded-sm">{"a"}</span>{" with "}<span class="italic bg-gray-300 rounded-sm">{"@"}</span>{"?"}</p>
                                                        <p class="text-xl sm:text-3xl lg:text-3xl xl:text-2xl">{"Don't worry! We have you covered! Just click generate until you see a cryptographically secure password you like. Stop by any time!"}</p>
                                                    </div>
                                                    <h3 class=format!("p-5 text-xl sm:text-3xl lg:text-4xl xl:text-xl bg-indigo-700 border-4 border-indigo-700 mt-4 py-4 px-8 select-all mx-auto text-gray-100 font-rts tracking-wide rounded-md justify-center {}",&self.state.password_state)>{&self.state.password_value}</h3>
                                                    <button onclick=self.link.callback(move |_| Msg::CreatePasswd) class="focus:outline-none p-5 text-xl sm:text-3xl lg:text-3xl xl:text-2xl py-5 px-8 mx-auto flex mt-8 text-gray-100 font-rtsBold uppercase tracking-wide rounded-md justify-center w-full bg-gradient-to-tr hover:from-green-400 active:bg-gradient-to-bl from-green-500 to-green-700">{"GENERATE!"}</button>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="inline w-full xl:flex xl:float-right xl:w-1/2">
                                            <div class="p-10">
                                                <div id="breach_check" class="bg-white rounded-lg elevation-24 text-center p-10">
                                                    <div class="">
                                                        <h2 class="font-rts text-xl sm:text-6xl lg:text-4xl xl:text-3xl p-2 underline">{"Data Breach Checker"}</h2>
                                                        <p class="text-xl sm:text-3xl lg:text-3xl xl:text-2xl">{"Enter your email here and we will check daily to see if any of your accounts have been compromised."}</p>
                                                        <p class="text-xl sm:text-3xl lg:text-3xl xl:text-2xl">{"We will only contact you after the first scan is complete, and if you are affected by any future data breaches. No marketing emails."}<br class="inline lg:hidden"/>{"We promise!"}</p>
                                                    </div>
                                                    <form action="https://formspree.io/f/xrgoadlk" method="POST">
                                                        <input required={true} pattern="[a-z0-9._%+-]+@[a-z0-9.-]+\\.[a-z]{2,63}$" type="email" placeholder="email@example.com" name="name" class="focus:outline-none p-5 text-xl sm:text-3xl lg:text-3xl xl:text-2xl border-4 border-indigo-700 mt-4 py-4 px-8 mx-auto text-gray-800 font-rts uppercase rounded-md text-center w-full" />
                                                        <input class="focus:outline-none p-5 text-xl sm:text-3xl lg:text-3xl xl:text-2xl py-5 px-8 mx-auto flex mt-8 text-gray-100 font-rtsBold uppercase tracking-wide rounded-md justify-center w-full bg-gradient-to-tr hover:from-green-400 active:bg-gradient-to-bl from-green-500 to-green-700" type="submit" value="SUBMIT!" />
                                                    </form>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                        </div>
                    </main>
                </body>
        }
    }
}
