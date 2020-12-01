use yew::prelude::*;

pub struct Home {}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
                <body>
                    <div class="bg-white">
                        <header class="sm:visible w-screen fixed z-50 elevation-3">
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
                                <div class="z-10 absolute">
                                    <div class="mt-10 sm:mt-24 max-w-xs sm:max-w-full inline-block h-96">
                                        <img class="w-20 sm:w-48 mx-auto" src="/img/logo.svg" alt="Crown logo"/>
                                        <h1 class="text-4xl sm:text-6xl text-gray-800 mx-auto text-center mt-3 sm:mt-4 font-rtsBold">{"Regal Tech Support"}</h1>
                                        <h2 class="text-xl sm:text-2xl text-gray-800 mx-auto mt-1 sm:mt-4 font-rts text-center">{"IT Support at home and at work in Colorado Springs"}</h2>
                                        <button type="button" class="text-xl sm:text-2xl focus:outline-none py-2 px-5 sm:py-4 sm:px-8 mx-auto flex mt-8 text-white font-rtsBold uppercase tracking-wide rounded-md items-center elevation-10 hover:from-blue-300 bg-gradient-to-tr from-green-300 via-blue-500 to-purple-600">{"GET SUPPORT"}</button>
                                    </div>
                                </div>
                                <img class="z-0 w-screen min-h-jumbotron h-screen object-cover object-center" src="/img/jumbotron.svg" alt="Castle in mountains"/>
                            </div>
                            <div class="max-w-7xl px-1 sm:px-20 md:mx-auto">
                                <div id="contact1" class="bg-polygons-green rounded-bl-lg inline-block w-full rounded-br-lg bg-cover">
                                    <form action="#" class="text-center p-1 sm:p-2 sm:p-5 sm:p-10 elevation-24 m-3 sm:m-10 rounded-lg bg-white">
                                        <div class="flex m-1 sm:m-3">
                                            <p class="float-left text-xs w-20 sm:text-base sm:w-28 text-left bg-indigo-700 text-white p-2 border-4 border-indigo-700 rounded m-1 font-rtsBold uppercase tracking-wide">{"To:"}</p>
                                            <p class="float-left text-xs sm:text-base text-left bg-indigo-700 text-white p-2 border-4 flex-grow border-indigo-700 rounded m-1 font-rtsBold uppercase tracking-wide">{"hello@regaltechsupport.com"}</p>
                                        </div>
                                        <div class="flex clear-both m-1 sm:m-3">
                                            <p class="float-left w-20 text-xs sm:text-base sm:w-28 text-left bg-indigo-700 text-white p-2 rounded border-4 border-indigo-700 m-1 font-rtsBold uppercase tracking-wide">{"From:"}</p>
                                            <input class="float-left p-2 flex-grow m-1 resize-none border-4 rounded-lg border-indigo-700 focus:outline-none focus:border-indigo-600" type="text" placeholder="example@email.com"/><br/>
                                        </div>
                                        <div class="flex clear-both m-1 sm:m-3">
                                            <p class="float-left w-20 sm:w-28 text-xs sm:text-base text-left bg-indigo-700 text-white p-2 rounded border-4 border-indigo-700 m-1 font-rtsBold uppercase tracking-wide">{"Message:"}</p>
                                            <textarea class="float-left p-2 flex-grow m-1 resize-none border-4 rounded-lg border-indigo-700 focus:outline-none focus:border-indigo-600" rows=5 placeholder="How can we help?"></textarea>
                                        </div>
                                        <div class="flex clear-both m-1 sm:m-3">
                                            <input class="justify-center text-white p-2 flex w-full sm:flex-grow rounded m-1 py-3 font-rtsBold uppercase tracking-wide focus:outline-none bg-gradient-to-tr hover:from-green-400 from-green-500 to-green-700" type="submit" value="SEND!" />
                                        </div>
                                            <h3 class="pt-1 font-rtsBold text-lg">{"Or call/text us at "}<br class="inline sm:hidden" /><span>{"(719) 440 - 9462"}</span>{"!"}</h3>
                                    </form>
                                </div>
                                <div class="inline-block mx-5 my-10">
                                    <h2 class="flex-1 text-4xl md:text-5xl font-rts text-gray-800 text-right">{"What Makes Us Different?"}</h2>
                                    <div class="flex-row mt-3">
                                        <img class="sm:w-1/2 h-78 sm:h-96 p-3 mx-auto sm:float-left" src="/img/jester_bluescreen.svg" alt="Cartoon charachter doing CPR on laptop"/>
                                        <p class="sm:w-1/2 sm:float-right p-3 text-lg md:text-2xl">{"Typical IT support is there only when things break. We prefer a proactive approach. By spending time to understand your situation we will create a reliable technology experience for you! We believe that reliability and security are possible with any budget!"}</p>
                                    </div>
                                </div>
                                <div class="bg-polygons-gray rounded-full bg-top h-8 my-12 flex"></div>
                                <div class="inline-block mx-5 my-10">
                                    <h2 class="text-4xl md:text-5xl font-rts text-gray-800 text-left">{"What We Do"}</h2>
                                    <div class="bg-blue-500 h-1/4 sm:w-1/2 p-3 sm:float-right"><p>{"Mascot place holder"}</p></div>
                                    <div class="sm:w-1/2 sm:float-left p-3 align-middle">
                                        <p class="text-lg md:text-2xl">{"We build modern solutions for you at home and at work."}</p>
                                        <ul class="my-4 mx-1 md:mx-8">
                                            <li class="text-md md:text-xl">{"WiFi not working?"}</li>
                                            <li class="text-md md:text-xl">{"Printer won't print?"}</li>
                                            <li class="text-md md:text-xl">{"Zoom cannot share screen?"}</li>
                                            <li class="text-md md:text-xl">{"How to backup Windows?"}</li>
                                            <li class="text-md md:text-xl">{"Worried about hackers?"}</li>
                                        </ul>
                                    </div>
                                    <p class="inline-block mt-2 text-lg md:text-2xl">{"Regal Tech Support is reliable support for your household, family, or small buisness. Contact us and we will discuss possible solutions for free!"}</p>
                                </div>
                                <div class="bg-polygons-gray rounded-full bg-center h-8 my-12 flex"></div>
                                <div class="inline-block mx-5 my-10">
                                    <h2 class="flex-1 text-4xl font-rts md:text-5xl text-gray-800 text-right">{"24/7 Support"}</h2>
                                    <div class="flex-row mt-3">
                                        <div class="bg-blue-500 h-1/4 sm:w-1/2 p-3 sm:float-left mb-3"><p>{"Mascot place holder"}</p></div>
                                        <p class="sm:w-1/2 sm:float-right p-3 text-lg md:text-2xl sm:mb-5">{"Nobody gets to choose when things break. We will be there for you whenever and wherever you need us!"}</p>
                                        <button type="button" class="focus:outline-none text-xl sm:text-2xl py-2 px-5 sm:py-4 sm:px-8 mx-auto clear-both flex mt-8 text-gray-100 font-rtsBold uppercase tracking-wide rounded-md items-center hover:from-blue-300 bg-gradient-to-tr from-green-300 via-blue-500 to-purple-600">{"GET SUPPORT"}</button>
                                    </div>
                                </div>
                                <div id="contact2" class="bg-polygons-gold rounded-lg mb-10 inline-block w-full rounded-br-lg bg-cover">
                                    <form action="#" class="text-center p-1 sm:p-2 sm:p-5 sm:p-10 elevation-24 m-3 sm:m-10 rounded-lg bg-white">
                                        <div class="flex m-1 sm:m-3">
                                            <p class="float-left text-xs w-20 sm:text-base sm:w-28 text-left bg-indigo-700 text-white p-2 border-4 border-indigo-700 rounded m-1 font-rtsBold uppercase tracking-wide">{"To:"}</p>
                                            <p class="float-left text-xs sm:text-base text-left bg-indigo-700 text-white p-2 border-4 flex-grow border-indigo-700 rounded m-1 font-rtsBold uppercase tracking-wide">{"hello@regaltechsupport.com"}</p>
                                        </div>
                                        <div class="flex clear-both m-1 sm:m-3">
                                            <p class="float-left w-20 text-xs sm:text-base sm:w-28 text-left bg-indigo-700 text-white p-2 rounded border-4 border-indigo-700 m-1 font-rtsBold uppercase tracking-wide">{"From:"}</p>
                                            <input class="float-left p-2 flex-grow m-1 resize-none border-4 rounded-lg border-indigo-700 focus:outline-none focus:border-indigo-600" type="text" placeholder="example@email.com" /><br/>
                                        </div>
                                        <div class="flex clear-both m-1 sm:m-3">
                                            <p class="float-left w-20 sm:w-28 text-xs sm:text-base text-left bg-indigo-700 text-white p-2 rounded border-4 border-indigo-700 m-1 font-rtsBold uppercase tracking-wide">{"Message:"}</p>
                                            <textarea class="float-left p-2 flex-grow m-1 resize-none border-4 rounded-lg border-indigo-700 focus:outline-none focus:border-indigo-600" rows=5 placeholder="How can we help?"></textarea>
                                        </div>
                                        <div class="flex clear-both m-1 sm:m-3">
                                            <input class="justify-center text-white p-2 py-3 flex w-full sm:flex-grow rounded m-1 font-rtsBold uppercase tracking-wide focus:outline-none bg-gradient-to-tr hover:from-green-400 from-green-500 to-green-700" type="submit" value="SEND!" />
                                        </div>
                                            <h3 class="pt-1 font-rtsBold text-lg">{"Or call/text us at "}<br class="inline sm:hidden" /><span>{"(719) 440 - 9462"}</span>{"!"}</h3>
                                    </form>
                                </div>
                                <div class="bg-polygons-gray rounded-full bg-left-bottom h-8 my-12 flex"></div>
                                    <div id="tools" style="background-size: cover; background-repeat: no-repeat" class="bg-polygons-indigo flex-row rounded-lg my-10">
                                        <div class="flex">
                                            <div class="p-5 bg-white flex-row mx-auto ml-10 float-left rounded-bl-lg rounded-br-lg elevation-5">
                                                <h2 class="text-4xl md:text-5xl flex text-gray-800 font-rts text-left">{"Useful Tools!"}</h2>
                                                <p class="text-lg font-rts">{"Always free, enjoy!"}</p>
                                            </div>
                                        </div>
                                        <div class="flex ml-10 lg:justify-around">
                                            <div class="flex-row lg:flex-col flex-1 mr-10">
                                                <div class="bg-white rounded-lg mx-auto my-10 lg:w-5/12 lg:float-left elevation-24">
                                                    <div id="password_gen" class="max-w-xl p-5 pb-10 lg:p-10 text-center mx-auto lg:h-tools">
                                                        <h2 class="font-rts text-2xl underline">{"Secure Password Generator"}</h2>
                                                        <p>{"Should it have a number? "}<br class="visible sm:invisible" />{"What about replacing an "}<span class="italic bg-gray-300 rounded-sm">{"a"}</span>{" with "}<span class="italic bg-gray-300 rounded-sm">{"@"}</span>{"?"}</p>
                                                        <p>{"Don't worry! We have you covered! Just click generate until you see a cryptographically secure password you like. Stop by any time!"}</p>
                                                        <h3 class="text-2xl bg-indigo-700 py-4 select-all px-8 mx-auto flex mt-8 text-gray-100 font-rtsBold uppercase tracking-wide rounded-md justify-center lg:mt-14">{"PASSWORD"}</h3>
                                                        <button type="button" class="focus:outline-none text-2xl py-4 px-8 mx-auto flex mt-8 text-gray-100 font-rtsBold uppercase tracking-wide rounded-md justify-center w-full bg-gradient-to-tr hover:from-green-400 from-green-500 to-green-700">{"GENERATE!"}</button>
                                                    </div>
                                                </div>
                                                <div class="bg-white rounded-lg mx-auto my-10 lg:w-5/12 lg:h-tools lg:float-right elevation-24">
                                                    <div id="breach_check" class="max-w-xl p-5 lg:p-10 text-center mx-auto">
                                                        <h2 class="font-rts text-2xl underline">{"Data Breach Checker"}</h2>
                                                        <p>{"Enter your email here and we will check daily to see if any of your accounts have been compromised."}</p>
                                                        <p>{"We will only contact you after the first scan is complete, and if you are affected by any future data breaches. No marketing emails. We promise!"}</p>
                                                        <form href="#">
                                                            <input placeholder="email@example.com" class="focus:outline-none text-xl border-4 border-indigo-700 py-4 px-8 mx-auto mt-8 text-gray-800 font-rts uppercase rounded-md text-center w-full" />
                                                            <input class="focus:outline-none text-2xl py-4 px-8 mx-auto flex mt-8 text-gray-100 font-rtsBold uppercase tracking-wide rounded-md justify-center w-full bg-gradient-to-tr hover:from-green-400 from-green-500 to-green-700" type="submit" value="SUBMIT!" />
                                                        </form>
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                            </div>
                        </main>
                    </div>
                </body>
        }
    }
}
